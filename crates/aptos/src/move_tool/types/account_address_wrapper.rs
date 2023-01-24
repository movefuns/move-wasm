use std::str::FromStr;

use super::cli_error::CliError;
use move_core_types::account_address::AccountAddress;

/// A wrapper around `AccountAddress` to be more flexible from strings than AccountAddress
#[derive(Clone, Copy, Debug)]
pub struct AccountAddressWrapper {
    pub account_address: AccountAddress,
}

impl FromStr for AccountAddressWrapper {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountAddressWrapper {
            account_address: load_account_arg(s)?,
        })
    }
}

/// Loads an account arg and allows for naming based on profiles
pub fn load_account_arg(str: &str) -> Result<AccountAddress, CliError> {
    println!("load_account_arg, {}", str);
    if str.starts_with("0x") {
        AccountAddress::from_hex_literal(str).map_err(|err| {
            CliError::CommandArgumentError(format!("Failed to parse AccountAddress {}", err))
        })
    } else if let Ok(account_address) = AccountAddress::from_str(str) {
        Ok(account_address)
    }
    //    else if let Some(Some(private_key)) =
    //        CliConfig::load_profile(Some(str), ConfigSearchMode::CurrentDirAndParents)?
    //            .map(|p| p.private_key)
    //    {
    //        let public_key = private_key.public_key();
    //        Ok(account_address_from_public_key(&public_key))
    //    }
    else {
        Err(CliError::CommandArgumentError(
            "'--account' or '--profile' after using aptos init must be provided".to_string(),
        ))
    }
}
