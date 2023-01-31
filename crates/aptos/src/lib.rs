pub mod framwork;
pub mod move_tool;
pub mod types;

use crate::move_tool::{
    types::{cli_command::CliCommand, result::CliResult},
    CompilePackage,
    Disassemble,
    TODO,
    Interactive
};
use clap::Parser;

/// Command Line Interface (CLI) for developing and interacting with the Aptos blockchain
#[derive(Parser)]
#[clap(name = "aptos", author, version, propagate_version = true)]
pub enum Tool {
    Interactive(Interactive),
    Compile(CompilePackage),
    Disassemble(Disassemble),
    Run(TODO),
    Test(TODO)
}

impl Tool {
    pub fn execute(self) -> CliResult {
        use Tool::*;
        match self {
            Interactive(tool) => tool.execute_serialized(),
            Compile(tool) => tool.execute_serialized(),
            Disassemble(tool) => tool.execute_serialized(),
            Run(tool)=> tool.execute_serialized(),
            Test(tool)=> tool.execute_serialized()
        }
    }
}

//pub trait ParserX: FromArgMatches {
//    /// Parse from `std::env::args_os()`, exit on error
//    fn parseJson() -> move_tool::types::result::CliTypedResult<Self> {
//
//        let order = std::fs::read("/Volumes/dev/project/movefuns/move-wasm/order.json").unwrap();
//
//        let mut matches = <Self as CommandFactory>::command().get_matches();
//        let res = <Self as FromArgMatches>::from_arg_matches_mut(&mut matches)
//            .map_err(format_error::<Self>);
//        match res {
//            Ok(s) => s,
//            Err(e) => {
//                // Since this is more of a development-time error, we aren't doing as fancy of a quit
//                // as `get_matches`
//                e.exit()
//            }
//        }
//    }
//}