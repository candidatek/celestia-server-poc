# celestia-server-poc

Celestia Data Push API

This API allows you to push data to the Celestia network using the /push_data/{data} endpoint. The data is submitted to the Celestia Devnet using a Celestia RPC node.

Prerequisites

	•	Rust: Make sure you have Rust installed on your machine. Install it from here.
	•	Celestia RPC Node: You need access to a Celestia RPC node. This can be a locally running node or a remote one.
	•	.env file: Set up environment variables to configure the RPC node and authentication token.

.env File Configuration

Create a .env file in the root of the project with the following content: