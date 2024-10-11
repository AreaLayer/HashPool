use ldk_node::LightningNode;
use lightning::ln::channelmanager::ChannelManager;
use lightning::invoice::Invoice;

fn main () {
    let node = LightningNode::new();
    let channel_manager = ChannelManager::new();
    let invoice = Invoice::new();
}

impl LightningNode {
    pub fn new() -> Self {
        LightningNode {
            channel_manager: ChannelManager::new(),
            invoice: Invoice::new(),
        }
    }
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            channels: Vec::new(),
        }
    }
}

impl Invoice {
    pub fn new() -> Self {
        Invoice {
            amount: 0,
            description: String::new(),
        }
    }
}