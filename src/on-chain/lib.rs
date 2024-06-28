use crate::OnChain;

pub struct OnChainImpl {}

impl OnChain for OnChainImpl {
    fn new() -> Self {
        Self {}
    }
}

impl OnChainImpl {
    pub fn get_balance(&self, address: &str) -> Result<u64, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.mempool{}", address, env::var("MEMPOOL_API_KEY").unwrap());
        let response = client.get(&url).send()?;
        let json: Value = response.json()?;
        let balance = json["result"].as_str().unwrap();
        Ok(balance.parse::<u64>().unwrap())
    }
}
impl OnChain for OnChainImpl {
    fn new() -> Self {
        Self {}
    }
}

impl OnChainImpl {
    pub fn get_balance(&self, address: &str) -> Result<u64, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.mempool{}", address, env::var("MEMPOOL_API_KEY").unwrap());
        let response = client.get(&url).send()?;
        let json: Value = response.json()?;
        let balance = json["result"].as_str().unwrap();
        Ok(balance.parse::<u64>().unwrap())
    }
}




