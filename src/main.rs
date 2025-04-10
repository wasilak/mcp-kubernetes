use log::info;
use mcpr::{
    error::MCPError,
    schema::common::{Tool, ToolInputSchema},
    server::{Server, ServerConfig},
    transport::stdio::StdioTransport,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), MCPError> {
    // Initialize logging
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    // Create a transport
    let transport = StdioTransport::new();

    // Create a kubectl tool
    let kubectl_tool = Tool {
        name: "kubectl".to_string(),
        description: Some("Execute kubectl commands".to_string()),
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(
                [(
                    "command".to_string(),
                    json!({
                        "type": "string",
                        "description": "The kubectl command to execute (e.g., 'get pods -n default')"
                    }),
                )]
                .into_iter()
                .collect::<HashMap<_, _>>(),
            ),
            required: Some(vec!["command".to_string()]),
        },
    };

    // Configure the server
    let server_config = ServerConfig::new()
        .with_name("Kubernetes Server")
        .with_version("1.0.0")
        .with_tool(kubectl_tool);

    // Create the server
    let mut server = Server::new(server_config);

    // Register tool handlers
    server.register_tool_handler("kubectl", |params: Value| async move {
        let command = params
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MCPError::Protocol("Missing command parameter".to_string()))?;

        info!("Executing kubectl command: {}", command);

        // Split the command into parts
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(MCPError::Protocol("Empty command".to_string()));
        }

        // Execute the command
        let output = Command::new("kubectl")
            .args(&parts)
            .output()
            .map_err(|e| MCPError::Protocol(format!("Failed to execute kubectl: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(MCPError::Protocol(format!("kubectl error: {}", stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        info!("Command output: {}", stdout);

        Ok(json!({
            "result": stdout
        }))
    })?;

    // Start the server
    info!("Starting Kubernetes server...");
    server.serve(transport).await?;

    Ok(())
}
