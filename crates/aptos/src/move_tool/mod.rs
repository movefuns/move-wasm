pub mod types;

use clap::{ArgEnum, Parser};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::framwork::{BuildOptions, BuiltPackage};

pub use move_core_types::account_address::AccountAddress;
use types::move_package_dir::MovePackageDir;
use super::move_tool::types::{
    cli_command::CliCommand,
    result::CliTypedResult,
    cli_error::CliError,
    utils::set_bytecode_version
};

#[derive(Parser)]
pub struct IncludedArtifactsArgs {
    /// Artifacts to be generated when building the package
    ///
    /// Which artifacts to include in the package. This can be one of `none`, `sparse`, and
    /// `all`. `none` is the most compact form and does not allow to reconstruct a source
    /// package from chain; `sparse` is the minimal set of artifacts needed to reconstruct
    /// a source package; `all` includes all available artifacts. The choice of included
    /// artifacts heavily influences the size and therefore gas cost of publishing: `none`
    /// is the size of bytecode alone; `sparse` is roughly 2 times as much; and `all` 3-4
    /// as much.
    #[clap(long, default_value_t = IncludedArtifacts::Sparse)]
    pub(crate) included_artifacts: IncludedArtifacts,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum IncludedArtifacts {
    None,
    Sparse,
    All,
}

impl Display for IncludedArtifacts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use IncludedArtifacts::*;
        match self {
            None => f.write_str("none"),
            Sparse => f.write_str("sparse"),
            All => f.write_str("all"),
        }
    }
}

impl FromStr for IncludedArtifacts {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IncludedArtifacts::*;
        match s {
            "none" => Ok(None),
            "sparse" => Ok(Sparse),
            "all" => Ok(All),
            _ => Err("unknown variant"),
        }
    }
}

impl IncludedArtifacts {
    pub(crate) fn build_options(
        self,
        skip_fetch_latest_git_deps: bool,
        named_addresses: BTreeMap<String, AccountAddress>,
        bytecode_version: u32,
    ) -> BuildOptions {
        use IncludedArtifacts::*;
        match self {
            None => BuildOptions {
                with_srcs: false,
                with_abis: false,
                with_source_maps: false,
                // Always enable error map bytecode injection
                with_error_map: true,
                named_addresses,
                skip_fetch_latest_git_deps,
                bytecode_version: Some(bytecode_version),
                ..BuildOptions::default()
            },
            Sparse => BuildOptions {
                with_srcs: true,
                with_abis: false,
                with_source_maps: false,
                with_error_map: true,
                named_addresses,
                skip_fetch_latest_git_deps,
                bytecode_version: Some(bytecode_version),
                ..BuildOptions::default()
            },
            All => BuildOptions {
                with_srcs: true,
                with_abis: true,
                with_source_maps: true,
                with_error_map: true,
                named_addresses,
                skip_fetch_latest_git_deps,
                bytecode_version: Some(bytecode_version),
                ..BuildOptions::default()
            },
        }
    }
}

/// Compiles a package and returns the associated ModuleIds
#[derive(Parser)]
pub struct CompilePackage {
    /// Save the package metadata in the package's build directory
    ///
    /// If set, package metadata should be generated and stored in the package's build directory.
    /// This metadata can be used to construct a transaction to publish a package.
    #[clap(long)]
    pub(crate) save_metadata: bool,
    #[clap(flatten)]
    pub(crate) included_artifacts_args: IncludedArtifactsArgs,
    #[clap(flatten)]
    pub(crate) move_options: MovePackageDir,
}

/// TODO
/// Disassemble the Move bytecode pointed to
#[derive(Parser)]
#[clap(name = "disassemble")]
pub struct Disassemble {
    /// Start a disassembled bytecode-to-source explorer
    #[clap(long = "interactive")]
    pub interactive: bool,
    /// The package name. If not provided defaults to current package modules only
    #[clap(long = "package")]
    pub package_name: Option<String>,
    /// The name of the module or script in the package to disassemble
    #[clap(long = "name")]
    pub module_or_script_name: String,
}

/// TODO
/// Start a explorer
#[derive(Parser)]
#[clap(name= "Interactive")]
pub struct Interactive {

}

impl CliCommand<Vec<String>> for Interactive {
    fn command_name(&self) -> &'static str {
        "Interactive"
    }
    fn execute(self) -> CliTypedResult<Vec<String>> {
        unimplemented!("Interactive")
    }
}

/// TODO
#[derive(Parser)]
#[clap(name= "TODO")]
pub struct TODO {

}

impl CliCommand<Vec<String>> for TODO {
    fn command_name(&self) -> &'static str {
        "TODO"
    }
    fn execute(self) -> CliTypedResult<Vec<String>> {
        unimplemented!("TODO")
    }
}

impl CliCommand<Vec<String>> for Disassemble {
    fn command_name(&self) -> &'static str {
        "Disassemble"
    }
    fn execute(self) -> CliTypedResult<Vec<String>> {
        unimplemented!("TODO")
    }
}

impl CliCommand<Vec<String>> for CompilePackage {
    fn command_name(&self) -> &'static str {
        "CompilePackage"
    }
    fn execute(self) -> CliTypedResult<Vec<String>> {
        set_bytecode_version(self.move_options.bytecode_version);
        let build_options = BuildOptions {
            install_dir: self.move_options.output_dir.clone(),
            ..self
                .included_artifacts_args
                .included_artifacts
                .build_options(
                    self.move_options.skip_fetch_latest_git_deps,
                    self.move_options.named_addresses(),
                    self.move_options.bytecode_version_or_detault(),
                )
        };
        let pack = BuiltPackage::build(self.move_options.get_package_path()?, build_options)
            .map_err(|e| CliError::MoveCompilationError(format!("{:#}", e)))?;
        if self.save_metadata {
            pack.extract_metadata_and_save()?;
        }
        let ids = pack
            .modules()
            .into_iter()
            .map(|m| m.self_id().to_string())
            .collect::<Vec<_>>();
        Ok(ids)
    }
}
