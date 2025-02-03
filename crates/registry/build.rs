//! Build script that generates a `configs.json` file from the configs.

use maili_genesis::ChainConfig;
use maili_superchain::{Superchain, SuperchainConfig, Superchains};

fn main() {
    // Get the directory of this file from the environment
    let src_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Copy the `superchain-registry/chainList.json` file to `etc/chainList.json`
    let chain_list = format!("{}/superchain-registry/chainList.json", src_dir);
    std::fs::copy(chain_list, "etc/chainList.json").unwrap();

    // Get the `superchain-registry/superchain/configs` directory`
    let configs_dir = format!("{}/superchain-registry/superchain/configs", src_dir);
    let configs = std::fs::read_dir(configs_dir).unwrap();

    // Get all the directories in the `configs` directory
    let mut superchains = Superchains::default();
    for config in configs {
        let config = config.unwrap();
        let config_path = config.path();
        let superchain_name = config.file_name().into_string().unwrap();
        let mut superchain =
            Superchain { name: superchain_name, chains: Vec::new(), ..Default::default() };
        if config_path.is_dir() {
            let config_files = std::fs::read_dir(&config_path).unwrap();
            for config_file in config_files {
                let config_file = config_file.unwrap();
                let config_file_path = config_file.path();

                // Read the `superchain.toml` as the `SuperchainConfig`
                let config_file_name = config_file.file_name().into_string().unwrap();
                if config_file_name == "superchain.toml" {
                    let config = std::fs::read_to_string(config_file_path).unwrap();
                    let config: SuperchainConfig = toml::from_str(&config).unwrap();
                    superchain.config = config;
                    continue;
                }

                // Read the config file as a `ChainConfig`
                let config = std::fs::read_to_string(config_file_path).unwrap();
                let config: ChainConfig = toml::from_str(&config).unwrap();
                superchain.chains.push(config);
            }
            superchains.superchains.push(superchain);
        }
    }
    std::fs::write("etc/configs.json", serde_json::to_string_pretty(&superchains).unwrap())
        .unwrap();
}
