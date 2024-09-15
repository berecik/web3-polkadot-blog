# Building a Web3 Application with Substrate and Rust

*Unlock the potential of decentralized applications with your own custom blockchain.*

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Node](#running-the-node)
- [Developing Smart Contracts](#developing-smart-contracts)
- [Integrating the Frontend](#integrating-the-frontend)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

---

## Introduction

This project demonstrates how to build a Web3 application using [Substrate](https://substrate.dev/) and [Rust](https://www.rust-lang.org/). It includes:

- A custom Substrate blockchain node.
- Smart contracts developed with [ink!](https://paritytech.github.io/ink-docs/).
- A frontend application interacting with the blockchain via the [Polkadot.js API](https://polkadot.js.org/api/).

By following this guide, you'll set up a local blockchain network, deploy smart contracts, and integrate them with a frontend application.

## Features

- **Customizable Blockchain Node:** Modify and extend your blockchain's runtime.
- **Smart Contract Support:** Write and deploy smart contracts using ink!.
- **Frontend Integration:** Interact with your blockchain through a user-friendly interface.
- **Modular Design:** Easily add new pallets and functionalities.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust and Cargo:** [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js and npm:** [Install Node.js](https://nodejs.org/en/download/)
- **Git:** [Install Git](https://git-scm.com/downloads)
- **cargo-contract:** For ink! smart contract development

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/yourproject.git
cd yourproject
```

### 2. Install Rust Toolchain

Ensure that you have the latest stable Rust toolchain:

```bash
rustup install stable
rustup update
```

### 3. Install Substrate Prerequisites

Follow the official [Substrate installation guide](https://docs.substrate.io/install/) for your operating system.

### 4. Install cargo-contract

```bash
cargo install cargo-contract --force
```

### 5. Build the Substrate Node

```bash
cd backend
cargo build --release
```

## Running the Node

Start your local blockchain node:

```bash
./target/release/backend-template --dev
```

This command runs the node in development mode with temporary state.

## Developing Smart Contracts

### 1. Navigate to the Contracts Directory

```bash
cd contracts/my_contract
```

### 2. Build the Smart Contract

```bash
cargo +nightly contract build
```

### 3. Deploy the Contract

Use the [Polkadot.js Apps UI](https://polkadot.js.org/apps/#/contracts) to deploy your contract:

1. Navigate to the Contracts tab.
2. Upload your contract's `.contract` metadata file from the `target` directory.
3. Follow the on-screen instructions to deploy.

## Integrating the Frontend

### 1. Navigate to the Frontend Directory

```bash
cd frontend
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Configure the Connection

Ensure the frontend is configured to connect to your local node. Update the WebSocket endpoint in your configuration file (e.g., `config.js`):

```javascript
module.exports = {
  providerEndpoint: 'ws://localhost:9944',
};
```

### 4. Start the Frontend Application

```bash
npm start
```

Visit `http://localhost:3000` in your browser to interact with your application.

## Project Structure

```
yourproject/
├── node/                   # Custom Substrate node
│   ├── src/
│   └── Cargo.toml
├── runtime/                # Runtime modules (pallets)
│   ├── src/
│   └── Cargo.toml
├── contracts/              # ink! smart contracts
│   └── my_contract/
│       ├── src/
│       └── Cargo.toml
├── frontend/               # Frontend application
│   ├── src/
│   ├── package.json
│   └── ...
├── scripts/                # Deployment and utility scripts
├── README.md
└── LICENSE
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/your-feature-name`.
3. Commit your changes: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature/your-feature-name`.
5. Open a pull request.

Please make sure to update tests as appropriate.