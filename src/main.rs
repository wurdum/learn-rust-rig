use rig::completion::{Chat, Completion, Message, ToolDefinition};
use rig::providers::openai;
use rig::tool::Tool;
use std::future::Future;

#[tokio::main]
async fn main() {
    let openai = openai::Client::from_env();
    let agent = openai
        .agent("gpt-4o")
        .preamble("You are an investment advisor who focuses on long-term investments.")
        .tool(ClientTool)
        .temperature(1.0)
        .build();

    let profile_response = agent
        .chat("Hello, I'm John. Can you advise how can I manage my savings better?", vec![])
        .await
        .expect("Failed to chat with agent");

    let profile = Message {
        role: "user".to_string(),
        content: format!("Client profile: {}", profile_response),
    };

    let recommendation_response = agent
        .chat("Provide the recommendation considering user's investment profile", vec![profile])
        .await
        .expect("Failed to chat with agent");

    println!("{}", recommendation_response);
}

#[derive(serde::Deserialize)]
struct ClientToolArgs {
    name: String
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ClientTool;

#[derive(Debug, thiserror::Error)]
#[error("Client not found")]
struct ClientNotFound;

#[derive(serde::Deserialize, serde::Serialize)]
struct Client {
    age: u32,
    risk_profile: String,
}

impl Tool for ClientTool {
    const NAME: &'static str = "get_client_profile";

    type Error = ClientNotFound;
    type Args = ClientToolArgs;
    type Output = Client;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Provides basic information about the client".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Client's name"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if args.name == "John" {
            Ok(Client {
                age: 35,
                risk_profile: "moderate".to_string(),
            })
        } else {
            Err(ClientNotFound)
        }
    }
}