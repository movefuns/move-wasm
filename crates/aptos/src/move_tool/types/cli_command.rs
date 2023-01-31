use std::time::Instant;

use async_trait::async_trait;

use serde::Serialize;

use super::{
    result::{CliResult, CliTypedResult},
    utils::{start_logger, to_common_result, to_common_success_result},

};

/// A common trait for all CLI commands to have consistent outputs
#[async_trait]
pub trait CliCommand<T: Serialize + Send>: Sized + Send {
    /// Returns a name for logging purposes
     fn command_name(&self) -> &'static str;

    /// Executes the command, returning a command specific type
     fn execute(self) -> CliTypedResult<T>;

    /// Executes the command, and serializes it to the common JSON output type
     fn execute_serialized(self) -> CliResult {
        let command_name = self.command_name();
        start_logger();
        let start_time = Instant::now();
        to_common_result(command_name, start_time, self.execute())
    }

    /// Same as execute serialized without setting up logging
     fn execute_serialized_without_logger(self) -> CliResult {
        let command_name = self.command_name();
        let start_time = Instant::now();
        to_common_result(command_name, start_time, self.execute())
    }

    /// Executes the command, and throws away Ok(result) for the string Success
     fn execute_serialized_success(self) -> CliResult {
        start_logger();
        let command_name = self.command_name();
        let start_time = Instant::now();
        to_common_success_result(command_name, start_time, self.execute())
    }
}
