use pinwheel::prelude::*;
use std::borrow::Cow;

#[derive(builder, Default, new)]
#[new(default)]
pub struct Code {
	#[builder]
	pub code: Option<Cow<'static, str>>,
	#[builder]
	pub language: Option<Language>,
	#[builder]
	pub line_numbers: Option<bool>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Language {
	Elixir,
	Go,
	Javascript,
	Php,
	Python,
	Ruby,
	Rust,
}

impl Component for Code {
	fn into_node(self) -> Node {
		let mut code = self.code.unwrap_or(Cow::Borrowed(""));
		if let Some(language) = self.language {
			code = highlight(code.as_ref(), language).into();
		}
		let line_numbers = self.line_numbers.unwrap_or(false);
		let line_numbers = if line_numbers {
			Some(LineNumbers {
				count: count_lines(&code),
			})
		} else {
			None
		};
		div()
			.class("code")
			.child(line_numbers)
			.child(div().class("code-inner").inner_html(code))
			.into_node()
	}
}

pub struct LineNumbers {
	pub count: usize,
}

impl Component for LineNumbers {
	fn into_node(self) -> Node {
		div()
			.class("code-line-numbers-wrapper")
			.children((0..self.count).map(|index| {
				div()
					.class("code-line-numbers")
					.child((index + 1).to_string())
			}))
			.into_node()
	}
}

pub struct InlineCodeBlock {
	pub code: Cow<'static, str>,
}

impl InlineCodeBlock {
	pub fn new(code: impl Into<Cow<'static, str>>) -> InlineCodeBlock {
		InlineCodeBlock { code: code.into() }
	}
}

impl Component for InlineCodeBlock {
	fn into_node(self) -> Node {
		span().class("inline-code").child(self.code).into_node()
	}
}

fn count_lines(text: &str) -> usize {
	let n_lines = text.split('\n').count();
	if text.ends_with('\n') {
		n_lines - 1
	} else {
		n_lines
	}
}

pub struct HighlightCodeForLanguage {
	pub elixir: Cow<'static, str>,
	pub go: Cow<'static, str>,
	pub javascript: Cow<'static, str>,
	pub php: Cow<'static, str>,
	pub python: Cow<'static, str>,
	pub ruby: Cow<'static, str>,
	pub rust: Cow<'static, str>,
}

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn highlight_code_for_language(
	code_for_language: &HighlightCodeForLanguage,
) -> HighlightCodeForLanguage {
	HighlightCodeForLanguage {
		elixir: highlight(&code_for_language.elixir, Language::Elixir).into(),
		go: highlight(&code_for_language.go, Language::Go).into(),
		javascript: highlight(&code_for_language.javascript, Language::Javascript).into(),
		php: highlight(&code_for_language.php, Language::Php).into(),
		python: highlight(&code_for_language.python, Language::Python).into(),
		ruby: highlight(&code_for_language.ruby, Language::Ruby).into(),
		rust: highlight(&code_for_language.rust, Language::Rust).into(),
	}
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! highlight_configuration_js {
	($i:ident) => {
		static $i: once_cell::sync::Lazy<tree_sitter_highlight::HighlightConfiguration> =
			once_cell::sync::Lazy::new(|| {
				let language = tree_sitter_javascript::language();
				let query = tree_sitter_javascript::HIGHLIGHT_QUERY;
				let mut config =
					tree_sitter_highlight::HighlightConfiguration::new(language, query, "", "", "")
						.unwrap();
				config.configure(&NAMES);
				config
			});
	};
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! highlight_configuration_rust {
	($i:ident) => {
		static $i: once_cell::sync::Lazy<tree_sitter_highlight::HighlightConfiguration> =
			once_cell::sync::Lazy::new(|| {
				let language = tree_sitter_rust::language();
				let query = tree_sitter_rust::HIGHLIGHTS_QUERY;
				let mut config =
					tree_sitter_highlight::HighlightConfiguration::new(language, query, "", "", "")
						.unwrap();
				config.configure(&NAMES);
				config
			});
	};
}

#[cfg(target_arch = "wasm32")]
pub fn highlight(_code: &str, _language: Language) -> String {
	unimplemented!()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn highlight(code: &str, language: Language) -> String {
	static NAMES: once_cell::sync::Lazy<Vec<String>> = once_cell::sync::Lazy::new(|| {
		[
			"comment",
			"function",
			"keyword",
			"operator",
			"punctuation",
			"string",
			"type",
			"variable",
		]
		.iter()
		.copied()
		.map(String::from)
		.collect()
	});
	highlight_configuration_js!(ELIXIR);
	highlight_configuration_js!(GO);
	highlight_configuration_js!(JAVASCRIPT);
	highlight_configuration_js!(PHP);
	highlight_configuration_js!(PYTHON);
	highlight_configuration_js!(RUBY);
	highlight_configuration_rust!(RUST);
	let highlight_configuration = match language {
		Language::Elixir => &ELIXIR,
		Language::Go => &GO,
		Language::Javascript => &JAVASCRIPT,
		Language::Php => &PHP,
		Language::Python => &PYTHON,
		Language::Ruby => &RUBY,
		Language::Rust => &RUST,
	};
	let mut highlighter = tree_sitter_highlight::Highlighter::new();
	let highlights = highlighter
		.highlight(highlight_configuration, code.as_bytes(), None, |_| None)
		.unwrap();
	let mut highlighted_code = String::new();
	for event in highlights {
		match event.unwrap() {
			tree_sitter_highlight::HighlightEvent::Source { start, end } => {
				highlighted_code.push_str(&code[start..end]);
			}
			tree_sitter_highlight::HighlightEvent::HighlightStart(highlight) => {
				highlighted_code.push_str(&format!(
					"<span class=\"{}\">",
					NAMES.get(highlight.0).unwrap()
				));
			}
			tree_sitter_highlight::HighlightEvent::HighlightEnd => {
				highlighted_code.push_str("</span>");
			}
		}
	}
	highlighted_code
}
