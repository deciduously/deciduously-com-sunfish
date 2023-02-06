//! Helper functions to serve teh Sunfish app.

use crate::Context;
use backtrace::Backtrace;
use futures::{future::FutureExt, Future};
use std::{
	cell::RefCell,
	convert::Infallible,
	panic::AssertUnwindSafe,
	sync::{
		atomic::{self, AtomicUsize},
		Arc,
	},
};
use tracing::error;

/// Convenience function for serving the default handler.
pub async fn serve<C>(addr: std::net::SocketAddr, context: Arc<C>) -> hyper::Result<()>
where
	C: 'static + Send + Sync,
{
	serve_handler(addr, context, handle).await
}

/// Produce a [`http::Response`] from a [`http::Request`].
async fn handle(mut request: http::Request<hyper::Body>) -> http::Response<hyper::Body> {
	let context = Arc::clone(request.extensions().get::<Arc<Context>>().unwrap());
	let context = context.clone();
	let response = context
		.sunfish
		.handle(&mut request)
		.await
		.unwrap_or_else(|error| {
			error!(%error, backtrace = %error.backtrace());
			Some(
				http::Response::builder()
					.status(http::StatusCode::INTERNAL_SERVER_ERROR)
					.body(hyper::Body::from("internal server error"))
					.unwrap(),
			)
		});
	response.unwrap_or_else(|| {
		http::Response::builder()
			.status(http::StatusCode::NOT_FOUND)
			.body(hyper::Body::from("not found"))
			.unwrap()
	})
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Serve the provided handler, intercepting panics and returning 500 errors instead.
async fn serve_handler<C, H, F>(
	addr: std::net::SocketAddr,
	context: Arc<C>,
	handler: H,
) -> hyper::Result<()>
where
	C: 'static + Send + Sync,
	H: 'static + Fn(http::Request<hyper::Body>) -> F + Send + Sync,
	F: Future<Output = http::Response<hyper::Body>> + Send,
{
	// Create a task local that will store the panic message and backtrace if a panic occurs.
	tokio::task_local! {
		static PANIC_MESSAGE_AND_BACKTRACE: RefCell<Option<(String, Backtrace)>>;
	}
	async fn service<C, H, F>(
		handler: Arc<H>,
		context: Arc<C>,
		mut request: http::Request<hyper::Body>,
	) -> Result<http::Response<hyper::Body>, Infallible>
	where
		C: Send + Sync + 'static,
		H: Fn(http::Request<hyper::Body>) -> F + Send + Sync + 'static,
		F: Future<Output = http::Response<hyper::Body>> + Send,
	{
		let method = request.method().clone();
		let path = request.uri().path_and_query().unwrap().path().to_owned();
		tracing::info!(path = %path, method = %method, "request");
		request.extensions_mut().insert(context);
		request
			.extensions_mut()
			.insert(NEXT_ID.fetch_add(1, atomic::Ordering::SeqCst));
		let result = AssertUnwindSafe(handler(request)).catch_unwind().await;
		let start = std::time::SystemTime::now();
		let response = result.unwrap_or_else(|_| {
			let body = PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
				let panic_message_and_backtrace = panic_message_and_backtrace.borrow();
				let (message, backtrace) = panic_message_and_backtrace.as_ref().unwrap();
				tracing::error!(
					method = %method,
					path = %path,
					backtrace = ?backtrace,
					"500"
				);
				format!("{message}\n{backtrace:?}")
			});
			http::Response::builder()
				.status(http::StatusCode::INTERNAL_SERVER_ERROR)
				.body(hyper::Body::from(body))
				.unwrap()
		});
		tracing::info!(
			"Response generated in {}μs",
			start.elapsed().unwrap_or_default().as_micros()
		);
		Ok(response)
	}
	// Install a panic hook that will record the panic message and backtrace if a panic occurs.
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(|panic_info| {
		let value = (panic_info.to_string(), Backtrace::new());
		tracing::error!("{}\n{:?}", value.0, value.1);
		PANIC_MESSAGE_AND_BACKTRACE
			.try_with(|panic_message_and_backtrace| {
				panic_message_and_backtrace.borrow_mut().replace(value);
			})
			.ok();
	}));
	// Wrap the request handler and context with Arc to allow sharing a reference to it with each task.
	let handler = Arc::new(handler);
	let service = hyper::service::make_service_fn(|_| {
		let handler = handler.clone();
		let context = context.clone();
		async move {
			Ok::<_, Infallible>(hyper::service::service_fn(move |request| {
				let handler = handler.clone();
				let context = context.clone();
				PANIC_MESSAGE_AND_BACKTRACE.scope(RefCell::new(None), async move {
					service(handler, context, request).await
				})
			}))
		}
	});
	let server = hyper::server::Server::try_bind(&addr)?;
	tracing::info!("🚀 serving at {}", addr);
	server.serve(service).await?;
	std::panic::set_hook(hook);
	Ok(())
}
