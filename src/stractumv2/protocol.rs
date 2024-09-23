use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    id: u32,
    method: String,
    params: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    id: Option<u32>,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

struct MiniStratumClient {
    pool_address: String,
    pool_port: u16,
    username: String,
    password: String,
}

impl MiniStratumClient {
    pub fn new(pool_address: String, pool_port: u16, username: String, password: String) -> Self {
        Self {
            pool_address,
            pool_port,
            username,
            password,
        }
    }

    pub async fn connect(&self) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", self.pool_address, self.pool_port);
        let stream = TcpStream::connect(&addr).await?;
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut buffer = String::new();

        // Subscribe to the pool
        self.send_request(&mut writer, 1, "mining.subscribe", vec![]).await?;

        // Authorize the miner
        self.send_request(&mut writer, 2, "mining.authorize", vec![self.username.clone(), self.password.clone()]).await?;

        // Read incoming messages
        loop {
            buffer.clear();
            let bytes_read = reader.read_line(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }

            let response: JsonRpcResponse = serde_json::from_str(&buffer)?;
            self.handle_response(response).await?;
        }

        Ok(())
    }

    async fn send_request(
        &self,
        writer: &mut tokio::net::tcp::OwnedWriteHalf,
        id: u32,
        method: &str,
        params: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let request = JsonRpcRequest {
            id,
            method: method.to_string(),
            params,
        };

        let request_json = serde_json::to_string(&request)?;
        writer.write_all(request_json.as_bytes()).await?;
        writer.write_all(b"\n").await?;

        Ok(())
    }

    async fn handle_response(&self, response: JsonRpcResponse) -> Result<(), Box<dyn Error>> {
        if let Some(error) = response.error {
            println!("Error: {:?}", error);
        } else if let Some(result) = response.result {
            println!("Result: {:?}", result);

            // Handle specific messages (e.g., mining.notify for new jobs)
            if let Some(job) = result.as_array() {
                if job.len() > 0 && job[0] == "mining.notify" {
                    println!("Mining job received: {:?}", job);
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool_address = "your.mining.pool".to_string();
    let pool_port = 3333;
    let username = "miner_username".to_string();
    let password = "miner_password".to_string();

    let client = MiniStratumClient::new(pool_address, pool_port, username, password);
    client.connect().await?;

    Ok(())
}
