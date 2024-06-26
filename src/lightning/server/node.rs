use crate::server::Server;

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Node {
    pub server: Server,
}

impl Node {
    pub fn new() -> Node {
        Node {
            server: Server::new(),
        }
    }
}

impl Node {
    pub fn start(&self) {
        self.server.start();
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.server.stop();
    }
}

impl local_node::LocalNode for Node {
    fn start(&self) {
        self.start();
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


