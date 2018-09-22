pub mod schema;
pub mod transactions;
pub mod service;
pub mod api;
pub mod contracts;
pub mod errors;

#[macro_use]
extern crate exonum;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::MemoryDB;
use service::AuctionService;


fn node_config() -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = "0.0.0.0:8000".parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let peer_address = "0.0.0.0:2000".parse().unwrap();
    NodeConfig {
        listen_address: peer_address,
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: Some(peer_address),
        network: Default::default(),
        connect_list: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
        database: Default::default(),
    }
}

fn main() {
    // @todo Why do I need to unwrap the result if I don't use it?
    exonum::helpers::init_logger().unwrap();

    let node = Node::new(
        // @todo Should I worry about using RocksDB? Is it same level db as used in transactions above?
        // RocksDB::open(Path::new("path"), DbOptions::default())?
        MemoryDB::new(), //  Into<Arc<Database>>
        vec![Box::new(AuctionService)],
        node_config(),
        None // @todo Can't find this parameter in docs. What is this?
    );
    println!("Launch main loop");
    node.run().unwrap()
}
