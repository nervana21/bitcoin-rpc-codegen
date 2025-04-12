# Bitcoin RPC Code Generator

A tool for generating type-safe RPC client code for Bitcoin Core's JSON-RPC API.

## Features

- Generates TypeScript/JavaScript client code from Bitcoin Core's RPC documentation
- Full type safety with TypeScript definitions
- Support for all Bitcoin Core RPC methods
- Automatic parameter validation
- Promise-based API
- Comprehensive error handling

## Installation

```bash
npm install bitcoin-rpc-codegen
```

## Usage

```typescript
import { BitcoinRPC } from "bitcoin-rpc-codegen";

const rpc = new BitcoinRPC({
  url: "http://localhost:8332",
  username: "your_rpc_username",
  password: "your_rpc_password",
});

// Get blockchain info
const info = await rpc.getblockchaininfo();

// Send transaction
const txid = await rpc.sendrawtransaction({
  hexstring: "020000000...",
  allowhighfees: false,
});

// Get block
const block = await rpc.getblock({
  blockhash: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
});
```

## Development

1. Clone the repository:

```bash
git clone https://github.com/yourusername/bitcoin-rpc-codegen.git
cd bitcoin-rpc-codegen
```

2. Install dependencies:

```bash
npm install
```

3. Run tests:

```bash
npm test
```

4. Build the project:

```bash
npm run build
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
