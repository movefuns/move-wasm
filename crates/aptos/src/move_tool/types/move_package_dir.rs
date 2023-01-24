use clap::Parser;
use std::{
    path::PathBuf,
    collections::BTreeMap,
    str::FromStr
};

use super::{
    result::CliTypedResult,
    utils::dir_default_to_current,
};
use move_core_types::account_address::AccountAddress;
use super::account_address_wrapper::AccountAddressWrapper;

/// Options for compiling a move package dir
#[derive(Debug, Parser)]
pub struct MovePackageDir {
    /// Path to a move package (the folder with a Move.toml file)
    #[clap(long, parse(from_os_str))]
    pub package_dir: Option<PathBuf>,
    /// Path to save the compiled move package
    ///
    /// Defaults to `<package_dir>/build`
    #[clap(long, parse(from_os_str))]
    pub output_dir: Option<PathBuf>,
    /// Named addresses for the move binary
    ///
    /// Example: alice=0x1234, bob=0x5678
    ///
    /// Note: This will fail if there are duplicates in the Move.toml file remove those first.
    #[clap(long, parse(try_from_str = super::utils::parse_map), default_value = "")]
    pub(crate) named_addresses: BTreeMap<String, AccountAddressWrapper>,

    /// Skip pulling the latest git dependencies
    ///
    /// If you don't have a network connection, the compiler may fail due
    /// to no ability to pull git dependencies.  This will allow overriding
    /// this for local development.
    #[clap(long)]
    pub(crate) skip_fetch_latest_git_deps: bool,

    /// Specify the version of the bytecode the compiler is going to emit.
    #[clap(long)]
    pub bytecode_version: Option<u32>,
}

impl MovePackageDir {
    pub fn new(package_dir: PathBuf) -> Self {
        Self {
            package_dir: Some(package_dir),
            output_dir: None,
            named_addresses: Default::default(),
            skip_fetch_latest_git_deps: true,
            bytecode_version: None,
        }
    }

    pub fn get_package_path(&self) -> CliTypedResult<PathBuf> {
        dir_default_to_current(self.package_dir.clone())
    }

    /// Retrieve the NamedAddresses, resolving all the account addresses accordingly
    pub fn named_addresses(&self) -> BTreeMap<String, AccountAddress> {
        self.named_addresses
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.account_address))
            .collect()
    }

    pub fn bytecode_version_or_detault(&self) -> u32 {
        self.bytecode_version.unwrap_or(5)
    }

    pub fn add_named_address(&mut self, key: String, value: String) {
        self.named_addresses
            .insert(key, AccountAddressWrapper::from_str(&value).unwrap());
    }
}