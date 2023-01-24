
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
//use aptos_crypto::{
//    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
//    x25519, PrivateKey, ValidCryptoMaterial, ValidCryptoMaterialStringExt,
//};
use move_core_types::account_address::AccountAddress;

/// Config saved to `.aptos/config.yaml`
#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    /// Map of profile configs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<BTreeMap<String, ProfileConfig>>,
}

/// An individual profile
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProfileConfig {
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub network: Option<Network>,
    /// Private key for commands.
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub private_key: Option<Ed25519PrivateKey>,
    /// Public key for commands
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub public_key: Option<Ed25519PublicKey>,
    /// Account for commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountAddress>,
    /// URL for the Aptos rest endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_url: Option<String>,
    /// URL for the Faucet endpoint (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faucet_url: Option<String>,
}

/// ProfileConfig but without the private parts
#[derive(Debug, Serialize)]
pub struct ProfileSummary {
    pub has_private_key: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
//    pub public_key: Option<Ed25519PublicKey>,
//    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faucet_url: Option<String>,
}

impl From<&ProfileConfig> for ProfileSummary {
    fn from(config: &ProfileConfig) -> Self {
        ProfileSummary {
//            has_private_key: config.private_key.is_some(),
//            public_key: config.public_key.clone(),
            has_private_key:false,
            account: config.account,
            rest_url: config.rest_url.clone(),
            faucet_url: config.faucet_url.clone(),
        }
    }
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            profiles: Some(BTreeMap::new()),
        }
    }
}