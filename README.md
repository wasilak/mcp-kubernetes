# Kubernetes MCP Server

An experimental Model Context Protocol (MCP) server that exposes Kubernetes resources to AI chat interfaces. This project demonstrates how to integrate Kubernetes functionality with AI assistants, allowing them to help analyze and manage your cluster resources.

## Overview

This project is built on top of the amazing [mcpr](https://github.com/conikeec/mcpr) project by [@conikeec](https://github.com/conikeec), which provides a protocol for AI assistants to interact with external tools and services.

The server currently implements a simple but powerful example: listing Kubernetes namespaces. However, the architecture is designed to be easily extended with more Kubernetes functionality.

## Use Cases

- **Resource Discovery**: Ask your AI assistant to list and describe resources in your cluster
- **Troubleshooting**: Have the AI help analyze logs and resource states
- **Resource Management**: Get AI assistance in creating or modifying resources
- **Documentation**: Ask the AI to explain what a particular resource does in your cluster
- **Security Analysis**: Have the AI help identify potential security issues in your cluster configuration

## Example Interaction

```
User: Can you show me all namespaces in my cluster?
AI: I'll check that for you...
[Uses the list_namespaces tool]
Here are the namespaces in your cluster:
- default
- kube-system
- kube-public
- monitoring
```

## Future Possibilities

The current implementation is just the beginning. Here are some potential extensions:

- List and describe pods, deployments, services
- Show logs for specific pods
- Get resource utilization metrics
- Check resource health and status
- Analyze resource configurations
- Suggest optimizations
- Help with troubleshooting

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/mcp-kubernetes.git
cd mcp-kubernetes
```

2. Build the project:
```bash
cargo build --release
```

3. Configure your AI assistant to use the server (instructions depend on your AI platform)

## Requirements

- Rust 1.70 or later
- Kubernetes cluster with appropriate permissions
- kubeconfig configured for your cluster

## Configuration

The server uses your default kubeconfig. Make sure you have the necessary permissions to access the resources you want to expose.

## Development

To add new Kubernetes functionality:

1. Define a new tool in the server configuration
2. Implement the handler function
3. Register the handler with the server

Example:
```rust
let new_tool = Tool {
    name: "list_pods".to_string(),
    description: Some("Lists pods in a namespace".to_string()),
    input_schema: ToolInputSchema {
        r#type: "object".to_string(),
        properties: Some(
            [(
                "namespace".to_string(),
                json!({
                    "type": "string",
                    "description": "Namespace to list pods from"
                }),
            )]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        ),
        required: Some(vec!["namespace".to_string()]),
    },
};
```

## Acknowledgments

This project is built on top of the [mcpr](https://github.com/conikeec/mcpr) project by [@conikeec](https://github.com/conikeec). Many thanks to the author for creating this excellent protocol and implementation.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
