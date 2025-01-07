//! Get the version string of the toolchain.

use anyhow::Context;
use rustc_hash::FxHashMap;
use semver::Version;
use toolchain::Tool;

use crate::{toolchain_info::QueryConfig, utf8_stdout};

pub(crate) fn get(
    config: QueryConfig<'_>,
    extra_env: &FxHashMap<String, String>,
) -> Result<Option<Version>, anyhow::Error> {
    let (mut cmd, prefix) = match config {
        QueryConfig::Cargo(sysroot, cargo_toml) => {
            (sysroot.tool(Tool::Cargo, cargo_toml.parent()), "cargo ")
        }
        QueryConfig::Rustc(sysroot, current_dir) => {
            (sysroot.tool(Tool::Rustc, current_dir), "rustc ")
        }
    };
    cmd.envs(extra_env);
    cmd.arg("--version");
    let out = utf8_stdout(&mut cmd).with_context(|| format!("Failed to query rust toolchain version via `{cmd:?}`, is your toolchain setup correctly?"))?;

    let version =
        out.strip_prefix(prefix).and_then(|it| Version::parse(it.split_whitespace().next()?).ok());
    if version.is_none() {
        tracing::warn!("Failed to parse `{cmd:?}` output `{out}` as a semver version");
    }
    anyhow::Ok(version)
}
