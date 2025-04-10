# Kubernetes MCP Server

An experimental Model Context Protocol (MCP) server that exposes Kubernetes functionality to AI chat interfaces through kubectl. This project demonstrates how to integrate Kubernetes with AI assistants, allowing them to help analyze and manage your cluster resources using familiar kubectl commands.

## Overview

This project is built on top of the amazing [mcpr](https://github.com/conikeec/mcpr) project by [@conikeec](https://github.com/conikeec), which provides a protocol for AI assistants to interact with external tools and services.

The server provides a single `kubectl` tool that can execute any kubectl command, giving AI assistants full access to your Kubernetes cluster's capabilities.

## Use Cases

- **Resource Discovery**: Ask your AI assistant to list and describe resources using kubectl commands
- **Troubleshooting**: Have the AI help analyze logs and resource states using kubectl
- **Resource Management**: Get AI assistance in creating or modifying resources
- **Documentation**: Ask the AI to explain what a particular resource does in your cluster
- **Security Analysis**: Have the AI help identify potential security issues in your cluster configuration

## Example Interaction

```
User: Can you show me all pods in the default namespace?
AI: I'll check that for you...
[Uses the kubectl tool with command "get pods -n default"]
Here are the pods in the default namespace:
NAME                     READY   STATUS    RESTARTS   AGE
nginx-7c658794b9-2j4k5   1/1     Running   0          2d
```

## Future Possibilities

The current implementation provides a foundation for many powerful interactions:

- Complex troubleshooting scenarios using multiple kubectl commands
- Automated resource analysis and recommendations
- Interactive debugging sessions
- Security scanning and recommendations
- Performance optimization suggestions

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
- kubectl installed and configured
- Kubernetes cluster with appropriate permissions
- kubeconfig configured for your cluster

## Configuration

The server uses your default kubeconfig and kubectl installation. Make sure you have the necessary permissions to execute the commands you want to run.

## Security Considerations

Since this server allows execution of arbitrary kubectl commands, it's important to:

1. Run it with appropriate RBAC permissions
2. Consider implementing command validation or whitelisting
3. Monitor and audit command execution
4. Use it in a controlled environment

## Acknowledgments

This project is built on top of the [mcpr](https://github.com/conikeec/mcpr) project by [@conikeec](https://github.com/conikeec). Many thanks to the author for creating this excellent protocol and implementation.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
