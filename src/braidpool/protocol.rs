use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures::{StreamExt, SinkExt};
use serde::{Serialize, Deserialize};
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

struct BraidpoolClient {
    pool_address: String,
    username: String,
    password: String,
}

impl BraidpoolClient {
    pub fn new(pool_address: String, username: String, password: String) -> Self {
        Self {
            pool_address,
            username,
            password,
        }
    }

    pub async fn connect(&self) -> Result<(), Box<dyn Error>> {
        // Connect to the Braidpool WebSocket server
        let url = format!("ws://{}", self.pool_address);
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!("Connected to Braidpool!");

        // Subscribe to the pool (Braidpool equivalent of "mining.subscribe")
        self.send_request(&mut ws_stream, 1, "braidpool.subscribe", vec![]).await?;

        // Authorize the miner (Braidpool equivalent of "mining.authorize")
        self.send_request(&mut ws_stream, 2, "braidpool.authorize", vec![self.username.clone(), self.password.clone()]).await?;

        // Listen for messages from the pool
        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if msg.is_text() {
                let text = msg.into_text()?;
                let response: JsonRpcResponse = serde_json::from_str(&text)?;
                self.handle_response(response).await?;
            }
        }

        Ok(())
    }

    async fn send_request(
        &self,
        ws_stream: &mut tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
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
        ws_stream.send(Message::Text(request_json)).await?;

        Ok(())
    }

    async fn handle_response(&self, response: JsonRpcResponse) -> Result<(), Box<dyn Error>> {
        if let Some(error) = response.error {
            println!("Error: {:?}", error);
        } else if let Some(result) = response.result {
            println!("Result: {:?}", result);

            // Handle specific messages (e.g., braidpool.notify for new jobs)
            if let Some(job) = result.as_array() {
                if job.len() > 0 && job[0] == "braidpool.notify" {
                    println!("Braidpool job received: {:?}", job);
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool_address = "your.braidpool.server:port".to_string();
    let username = "miner_username".to_string();
    let password = "miner_password".to_string();

    let client = BraidpoolClient::new(pool_address, username, password);
    client.connect().await?;

    Ok(())
}
