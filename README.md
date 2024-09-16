# celestia-server-poc

Celestia Data Push API

This API allows you to push data to the Celestia network using the /push_data/{data} endpoint. The data is submitted to the Celestia Devnet using a Celestia RPC node.

Prerequisites

•	Rust: Make sure you have Rust installed on your machine. Install it from here.
•	Celestia RPC Node: You need access to a Celestia RPC node. This can be a locally running node or a remote one.
•	.env file: Set up environment variables to configure the RPC node and authentication token.

.env File Configuration

Create a .env file in the root of the project with the following content:

RPC_NODE_CELESTIA=http://your-celestia-rpc-url
AUTH_TOKEN_CELESTIA=your-auth-token

•	Replace http://your-celestia-rpc-url with your actual Celestia node URL.
•	Replace your-auth-token with the appropriate authentication token if required by your Celestia RPC node.

Getting Started

1. Clone the Repository

git clone https://github.com/your-repository.git
cd your-repository

API Endpoints

1. Push Data to Celestia Network

URL