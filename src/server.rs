mod id;
mod schema;
mod transactions;
mod service;
mod api;
mod contracts;
mod errors;

#[macro_use]
extern crate exonum;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate argparse;

use service::AuctionService;
use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::MemoryDB;
use argparse::{ArgumentParser, Store};


fn node_config(peer_port: &String, api_port: &String) -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = ("0.0.0.0:".to_string() + &api_port).parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let peer_address = ("0.0.0.0:".to_string() + &peer_port).parse().unwrap();
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
    let mut peer_port: String = "2000".to_string();
    let mut api_port: String = "8000".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Exonum server providing auction service");
        ap.refer(&mut peer_port)
            .add_option(&["-p", "--peer-port"], Store,
            "Network port to use as part of NodeConfig::listen_adress field");
        ap.refer(&mut api_port)
            .add_option(&["-a", "--api-port"], Store,
            "Name for the greeting");
        ap.parse_args_or_exit();
    }
    peer_port = peer_port.parse::<u16>().map(|x| x.to_string()).unwrap();
    api_port = api_port.parse::<u16>().map(|x| x.to_string()).unwrap();
    println!("-p {} -a {}", peer_port, api_port);

    // @todo Why do I need to unwrap the result if I don't use it?
    exonum::helpers::init_logger().unwrap();

    let node = Node::new(
        // @todo Should I worry about using RocksDB? Is it same level db as used in transactions above?
        // RocksDB::open(Path::new("path"), DbOptions::default())?
        MemoryDB::new(), //  Into<Arc<Database>>
        vec![Box::new(AuctionService)],
        node_config(&peer_port, &api_port),
        None // @todo Can't find this parameter in docs. What is this?
    );
    println!("Launch main loop");
    node.run().unwrap()
}
