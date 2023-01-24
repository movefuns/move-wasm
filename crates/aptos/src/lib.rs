pub mod framwork;
pub mod move_tool;
pub mod types;

use crate::move_tool::{
    types::{cli_command::CliCommand, result::CliResult},
    CompilePackage,
    Disassemble,
    TODO
};
use clap::Parser;

/// Command Line Interface (CLI) for developing and interacting with the Aptos blockchain
#[derive(Parser)]
#[clap(name = "aptos", author, version, propagate_version = true)]
pub enum Tool {
    Compile(CompilePackage),
    Disassemble(Disassemble),
    Run(TODO),
    Test(TODO)
}

impl Tool {
    pub fn execute(self) -> CliResult {
        use Tool::*;
        match self {
            Compile(tool) => tool.execute_serialized(),
            Disassemble(tool) => tool.execute_serialized(),
            Run(tool)=> tool.execute_serialized(),
            Test(tool)=> tool.execute_serialized()
        }
    }
}
