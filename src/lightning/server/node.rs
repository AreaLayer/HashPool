use crate::server::Server;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


