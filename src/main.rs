use k8s_openapi::api::core::v1::Namespace;
use kube::{Client, api::Api};
use log::info;
use mcpr::{
    error::MCPError,
    schema::common::{Tool, ToolInputSchema},
    server::{Server, ServerConfig},
    transport::stdio::StdioTransport,
};
use serde_json::{Value, json};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), MCPError> {
    // Initialize logging
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    // Create a transport
    let transport = StdioTransport::new();

    // Create a namespaces tool
    let namespaces_tool = Tool {
        name: "list_namespaces".to_string(),
        description: Some("Lists all Kubernetes namespaces".to_string()),
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(HashMap::new()),
            required: Some(Vec::new()),
        },
    };

    // Configure the server
    let server_config = ServerConfig::new()
        .with_name("Kubernetes Server")
        .with_version("1.0.0")
        .with_tool(namespaces_tool);

    // Create the server
    let mut server = Server::new(server_config);

    // Register tool handlers
    server.register_tool_handler("list_namespaces", |_params: Value| async move {
        let client = Client::try_default().await.map_err(|e| {
            MCPError::Protocol(format!("Failed to create Kubernetes client: {}", e))
        })?;

        let namespaces: Api<Namespace> = Api::all(client);
        let list = namespaces
            .list(&Default::default())
            .await
            .map_err(|e| MCPError::Protocol(format!("Failed to list namespaces: {}", e)))?;

        let names = list
            .items
            .into_iter()
            .filter_map(|ns| ns.metadata.name)
            .collect::<Vec<_>>();

        info!("Listed {} namespaces", names.len());

        Ok(json!({
            "result": names
        }))
    })?;

    // Start the server
    info!("Starting Kubernetes server...");
    server.serve(transport).await?;

    Ok(())
}
