use bitcoin_rpc::RpcApi;

use structum_core::{
    node::{Node, NodeConfig},
    rpc::RpcClient,
};


pub struct BitcoinNode {
    pub node: Node,
}

impl BitcoinNode {
    pub fn new(config: NodeConfig) -> Self {
        let node = Node::new(config);
        Self { node }
    }
    pub async fn start(&self) -> Result<(), String> {
        self.node.start().await
    }
    pub async fn stop(&self) -> Result<(), String> {
        self.node.stop().await
    }
    pub async fn get_rpc_client(&self) -> Result<RpcClient, String> {
        self.node.get_rpc_client().await
    }
    pub async fn get_rpc_api(&self) -> Result<RpcApi, String> {
        self.node.get_rpc_api().await
    }
    pub fn get_rpc_url(&self) -> String {
        self.node.get_rpc_url()
    }
    pub fn get_rpc_port(&self) -> u16 {
        self.node.get_rpc_port()
    }
    pub fn get_rpc_user(&self) -> String {
        self.node.get_rpc_user()
    }
}