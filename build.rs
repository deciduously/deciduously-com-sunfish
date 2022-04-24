use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
	let crate_path = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
	let workspace_path = crate_path.clone();
	let crate_out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
	let css_paths = vec![workspace_path.to_owned()];
	println!("cargo:rerun-if-changed=.");
	sunfish::build(sunfish::BuildOptions {
		workspace_path,
		crate_path,
		crate_out_dir,
		css_paths,
	})?;
	Ok(())
}
