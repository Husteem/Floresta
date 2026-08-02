// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::FlorestadError;

#[derive(Default, Debug, Deserialize)]
pub struct Wallet {
    pub xpubs: Option<Vec<String>>,
    pub descriptors: Option<Vec<String>>,
    pub addresses: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize)]
pub struct ChainStore {
    pub block_index_size: Option<usize>,
    pub headers_file_size: Option<usize>,
    pub fork_file_size: Option<usize>,
}

#[derive(Default, Debug, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub wallet: Wallet,
    pub chain_store: Option<ChainStore>,
}

impl ConfigFile {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, FlorestadError> {
        let config_file = fs::read_to_string(path.as_ref())?;

        Ok(toml::from_str(&config_file)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_deserialization() {
        // Test parsing of a complete, valid config
        let toml_str = r#"
            [wallet]
            xpubs = ["xpub1", "xpub2"]
            descriptors = ["desc1"]
            addresses = ["addr1"]

            [chain_store]
            block_index_size = 10000000
            headers_file_size = 10000000
            fork_file_size = 10000
        "#;
        let config: ConfigFile = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.wallet.xpubs.unwrap(),
            vec!["xpub1".to_string(), "xpub2".to_string()]
        );
        let chain_store = config.chain_store.unwrap();
        assert_eq!(chain_store.block_index_size, Some(10000000));
        assert_eq!(chain_store.headers_file_size, Some(10000000));
        assert_eq!(chain_store.fork_file_size, Some(10000));

        // Test optional fields and default parsing (missing fields/sections)
        let toml_str_partial = r#"
            [chain_store]
            block_index_size = 5000
        "#;
        let config_partial: ConfigFile = toml::from_str(toml_str_partial).unwrap();
        assert!(config_partial.wallet.xpubs.is_none());
        let chain_store_partial = config_partial.chain_store.unwrap();
        assert_eq!(chain_store_partial.block_index_size, Some(5000));
        assert_eq!(chain_store_partial.headers_file_size, None);
        assert_eq!(chain_store_partial.fork_file_size, None);

        // Test invalid data type for usize fields (should fail deserialization)
        let toml_invalid_type = r#"
            [chain_store]
            block_index_size = "not-a-number"
        "#;
        let res: Result<ConfigFile, _> = toml::from_str(toml_invalid_type);
        assert!(res.is_err());

        // Test negative number for usize fields (should fail deserialization)
        let toml_negative = r#"
            [chain_store]
            block_index_size = -5
        "#;
        let res_neg: Result<ConfigFile, _> = toml::from_str(toml_negative);
        assert!(res_neg.is_err());
    }
}
