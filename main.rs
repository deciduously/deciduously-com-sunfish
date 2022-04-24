#![warn(clippy::pedantic)]

use clap::Parser;
use std::sync::Arc;
use sunfish::Sunfish;
use tracing_subscriber::prelude::*;

mod serve;

/// Operating characteristics of the server.
#[derive(Parser)]
struct Args {
	#[clap(env, long, help = "Host IP to bind", default_value = "0.0.0.0")]
	host: String,
	#[clap(env, long, help = "Port to bind", default_value = "8080")]
	port: u16,
}

struct Context {
	sunfish: Sunfish,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	setup_tracing();
	let sunfish = sunfish::init!();
	let host = args.host.parse()?;
	let addr = std::net::SocketAddr::new(host, args.port);
	let context = Context { sunfish };
	let context = Arc::new(context);
	serve::serve(addr, context).await?;
	Ok(())
}

fn setup_tracing() {
	let env_layer = tracing_subscriber::filter::EnvFilter::try_from_env("deciduously_com_TRACING");
	let env_layer = if cfg!(debug_assertions) {
		Some(env_layer.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("[]=info")))
	} else {
		env_layer.ok()
	};
	if let Some(env_layer) = env_layer {
		if cfg!(debug_assertions) {
			let format_layer = tracing_subscriber::fmt::layer().pretty();
			let subscriber = tracing_subscriber::registry()
				.with(env_layer)
				.with(format_layer);
			subscriber.init();
		} else {
			let json_layer = tracing_subscriber::fmt::layer().json();
			let subscriber = tracing_subscriber::registry()
				.with(env_layer)
				.with(json_layer);
			subscriber.init();
		}
	}
}
