use std::{collections::BTreeMap, env, path::PathBuf, str::FromStr, time::Instant};

use super::{cli_error::CliError, result::CliTypedResult};
use itertools::Itertools;
use serde::Serialize;
use super::{
    result::CliResult,
    result::ResultWrapper
};

/// Error message for parsing a map
const PARSE_MAP_SYNTAX_MSG: &str = "Invalid syntax for map. Example: Name=Value,Name2=Value";

/// Parses an inline map of values
///
/// Example: Name=Value,Name2=Value
pub fn parse_map<K: FromStr + Ord, V: FromStr>(str: &str) -> anyhow::Result<BTreeMap<K, V>>
where
    K::Err: 'static + std::error::Error + Send + Sync,
    V::Err: 'static + std::error::Error + Send + Sync,
{
    let mut map = BTreeMap::new();

    // Split pairs by commas
    for pair in str.split_terminator(',') {
        // Split pairs by = then trim off any spacing
        let (first, second): (&str, &str) = pair
            .split_terminator('=')
            .collect_tuple()
            .ok_or_else(|| anyhow::Error::msg(PARSE_MAP_SYNTAX_MSG))?;
        let first = first.trim();
        let second = second.trim();
        if first.is_empty() || second.is_empty() {
            return Err(anyhow::Error::msg(PARSE_MAP_SYNTAX_MSG));
        }

        // At this point, we just give error messages appropriate to parsing
        let key: K = K::from_str(first)?;
        let value: V = V::from_str(second)?;
        map.insert(key, value);
    }
    Ok(map)
}

pub fn current_dir() -> CliTypedResult<PathBuf> {
    env::current_dir().map_err(|err| {
        CliError::UnexpectedError(format!("Failed to get current directory {}", err))
    })
}

pub fn dir_default_to_current(maybe_dir: Option<PathBuf>) -> CliTypedResult<PathBuf> {
    if let Some(dir) = maybe_dir {
        Ok(dir)
    } else {
        current_dir()
    }
}

// TODO:
pub fn start_logger() {
    //    let mut logger = aptos_logger::Logger::new();
    //    logger.channel_size(1000).is_async(false).level(Level::Warn);
    //    logger.build();
}

/// For pretty printing outputs in JSON
pub fn to_common_result<T: Serialize>(
    command: &str,
    start_time: Instant,
    result: CliTypedResult<T>,
) -> CliResult {
    let latency = start_time.elapsed();
    let is_err = result.is_err();

//    if !telemetry_is_disabled() {
        let error = if let Err(ref error) = result {
            // Only print the error type
            Some(error.to_str())
        } else {
            None
        };
//        send_telemetry_event(command, latency, !is_err, error).await;
//    }

    let result: ResultWrapper<T> = result.into();
    let string = serde_json::to_string_pretty(&result).unwrap();
    if is_err {
        Err(string)
    } else {
        Ok(string)
    }
}

/// Convert any successful response to Success
pub fn to_common_success_result<T>(
        command: &str,
        start_time: Instant,
        result: CliTypedResult<T>,
        ) -> CliResult {
    to_common_result(command, start_time, result.map(|_| "Success"))
}

const VAR_BYTECODE_VERSION: &str = "MOVE_BYTECODE_VERSION";

pub fn set_bytecode_version(version: Option<u32>) {
    // Note: this is a bit of a hack to get the compiler emit bytecode with the right
    //       version. In the future, we should add an option to the Move package system
    //       that would allow us to configure this directly instead of relying on
    //       environment variables.
    if let Some(ver) = version {
        env::set_var(VAR_BYTECODE_VERSION, ver.to_string());
    } else if env::var(VAR_BYTECODE_VERSION) == Err(env::VarError::NotPresent) {
        env::set_var(VAR_BYTECODE_VERSION, "5");
    }
}
