pub mod types;

use clap::{ArgEnum, Parser};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    fs,
    path::Path,
    str::FromStr,
};

use crate::framwork::{BuildOptions, BuiltPackage};

use super::move_tool::types::{
    cli_command::CliCommand, cli_error::CliError, result::CliTypedResult,
    utils::set_bytecode_version,
};
pub use move_core_types::account_address::AccountAddress;
use types::move_package_dir::MovePackageDir;

use move_binary_format::{
    binary_views::BinaryIndexedView,
    file_format::{CompiledModule, CompiledScript},
};

use move_coverage::coverage_map::CoverageMap;

use move_bytecode_source_map::{mapping::SourceMapping, utils::source_map_from_file};
use move_command_line_common::files::{
    MOVE_COMPILED_EXTENSION, MOVE_EXTENSION, SOURCE_MAP_EXTENSION,
};
use move_disassembler::disassembler::{Disassembler, DisassemblerOptions};
use move_ir_types::location::Spanned;

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
pub struct Disassemble {
    /// Skip printing of private functions.
    #[clap(long = "skip-private")]
    pub skip_private: bool,

    /// Do not print the disassembled bytecodes of each function.
    #[clap(long = "skip-code")]
    pub skip_code: bool,

    /// Do not print locals of each function.
    #[clap(long = "skip-locals")]
    pub skip_locals: bool,

    /// Do not print the basic blocks of each function.
    #[clap(long = "skip-basic-blocks")]
    pub skip_basic_blocks: bool,

    /// Treat input file as a script (default is to treat file as a module)
    #[clap(short = 's', long = "script")]
    pub is_script: bool,

    /// The path to the bytecode file to disassemble; let's call it file.mv. We assume that two
    /// other files reside under the same directory: a source map file.mvsm (possibly) and the Move
    /// source code file.move.
    #[clap(short = 'b', long = "bytecode")]
    pub bytecode_file_path: String,

    /// (Optional) Path to a coverage file for the VM in order to print trace information in the
    /// disassembled output.
    #[clap(short = 'c', long = "move-coverage-path")]
    pub code_coverage_path: Option<String>,
}

/// Start a explorer
#[derive(Parser)]
#[clap(name = "interactive")]
pub struct Interactive {
    #[clap(long = "order-path")]
    pub order_path: Option<String>,
    #[clap(long = "output-path")]
    pub output_path: Option<String>,
}

impl CliCommand<Vec<String>> for Interactive {
    fn command_name(&self) -> &'static str {
        "InteractiveA"
    }

    fn execute(self) -> CliTypedResult<Vec<String>> {
        loop {
            const DEFAULT_CONNAND: &str = "/workspace/order/";
            const DEFAULT_OK: &str = "/workspace/order/result_ok";
            const DEFAULT_ERR: &str = "/workspace/order/result_err";

            let s = match self.order_path {
                Some(ref v) => v.clone(),
                None => String::from(DEFAULT_CONNAND),
            };

            let path = std::path::Path::new(&s);
            while !path.exists() {
                let t = std::time::Duration::from_millis(1000);
                std::thread::sleep(t);
            }

            let s = String::from_utf8(fs::read(&s).unwrap()).unwrap();

            if s == "exit" {
                break;
            }

            let mut test: Vec<&str> = s.split(",").collect();
            test.insert(0, "");

            match crate::Tool::parse_from(test).execute() {
                Ok(inner) => println!("{}", inner),
                Err(inner) => {
                    println!("{}", inner);
                }
            }

            fs::remove_file(path);
        }

        Ok(vec!["Exit".to_string()])
    }
}

/// TODO
#[derive(Parser)]
#[clap(name = "TODO")]
pub struct TODO {}

impl CliCommand<Vec<String>> for TODO {
    fn command_name(&self) -> &'static str {
        "TODO"
    }
    fn execute(self) -> CliTypedResult<Vec<String>> {
        unimplemented!("TODO")
    }
}

impl CliCommand<String> for Disassemble {
    fn command_name(&self) -> &'static str {
        "Disassemble"
    }

    fn execute(self) -> CliTypedResult<String> {
        let move_extension = MOVE_EXTENSION;
        let mv_bytecode_extension = MOVE_COMPILED_EXTENSION;
        let source_map_extension = SOURCE_MAP_EXTENSION;

        let source_path = Path::new(&self.bytecode_file_path);
        let extension = source_path
            .extension()
            .expect("Missing file extension for bytecode file");
        if extension != mv_bytecode_extension {
            println!(
                "Bad source file extension {:?}; expected {}",
                extension, mv_bytecode_extension
            );
            std::process::exit(1);
        }

        let bytecode_bytes =
            fs::read(&self.bytecode_file_path).expect("Unable to read bytecode file");

        let source_path = Path::new(&self.bytecode_file_path).with_extension(move_extension);
        let source = fs::read_to_string(&source_path).ok();
        let source_map = source_map_from_file(
            &Path::new(&self.bytecode_file_path).with_extension(source_map_extension),
        );

        let mut disassembler_options = DisassemblerOptions::new();
        disassembler_options.print_code = !self.skip_code;
        disassembler_options.only_externally_visible = self.skip_private;
        disassembler_options.print_basic_blocks = !self.skip_basic_blocks;
        disassembler_options.print_locals = !self.skip_locals;

        // TODO: make source mapping work with the Move source language
        let no_loc = Spanned::unsafe_no_loc(()).loc;
        let module: CompiledModule;
        let script: CompiledScript;
        let bytecode = if self.is_script {
            script = CompiledScript::deserialize(&bytecode_bytes)
                .expect("Script blob can't be deserialized");
            BinaryIndexedView::Script(&script)
        } else {
            module = CompiledModule::deserialize(&bytecode_bytes)
                .expect("Module blob can't be deserialized");
            BinaryIndexedView::Module(&module)
        };

        let mut source_mapping = {
            if let Ok(s) = source_map {
                SourceMapping::new(s, bytecode)
            } else {
                SourceMapping::new_from_view(bytecode, no_loc)
                    .expect("Unable to build dummy source mapping")
            }
        };

        if let Some(source_code) = source {
            source_mapping
                .with_source_code((source_path.to_str().unwrap().to_string(), source_code));
        }

        let mut disassembler = Disassembler::new(source_mapping, disassembler_options);

        if let Some(file_path) = &self.code_coverage_path {
            disassembler.add_coverage_map(
                CoverageMap::from_binary_file(file_path)
                    .unwrap()
                    .to_unified_exec_map(),
            );
        }

        let dissassemble_string = disassembler.disassemble().expect("Unable to dissassemble");

        println!("{}", dissassemble_string);

        Ok(dissassemble_string)
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
