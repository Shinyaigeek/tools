//! Codegen tools mostly used to generate ast and syntax definitions. Adapted from rust analyzer's codegen

pub mod glue;

use std::{
    env,
    fmt::Display,
    path::{Path, PathBuf},
};

pub use crate::glue::{pushd, pushenv};

pub use anyhow::{bail, Context as _, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Overwrite,
    Verify,
}

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(2)
    .unwrap()
    .to_path_buf()
}

pub fn run_rustfmt(mode: Mode) -> Result<()> {
    let _dir = pushd(project_root());
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    match mode {
        Mode::Overwrite => run!("cargo fmt"),
        Mode::Verify => run!("cargo fmt -- --check"),
    }?;
    Ok(())
}

pub fn reformat(text: impl Display) -> Result<String> {
    reformat_without_preamble(text).map(prepend_generated_preamble)
}

const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/codegen`";
pub fn prepend_generated_preamble(content: impl Display) -> String {
    format!("//! {}\n\n{}\n", PREAMBLE, content)
}

pub fn reformat_without_preamble(text: impl Display) -> Result<String> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    run!(
        "rustfmt --config fn_single_line=true";
        <text.to_string().as_bytes()
    )
}

pub fn ensure_rustfmt() -> Result<()> {
    let out = run!("rustfmt --version")?;
    if !out.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}
