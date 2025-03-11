# PumpFun - Solana Smart Contract Forked for Meteora
## Contact

   If you need help or upgraded version, plz contact here: 
   
   [Telegram](https://t.me/sealdevnut)

**PumpFun Bonding Curve Protocol** is a Solana protocol that builds upon the **Meteora** Dex protocol to implement advanced functionalities for token rewards, liquidity management, and decentralized finance mechanics. This project is designed to integrate seamlessly with the Solana ecosystem, ensuring performance, scalability, and security.

## Core Features

### Bonding Curve Mechanism

The protocol implements a constant product bonding curve (x * y = k) with the following initial parameters:

- Initial Virtual Token Reserves: 1,073,000,000,000,000
- Initial Virtual SOL Reserves: 15,000,000,000
- Initial Real Token Reserves: 793,100,000,000,000
- Total Token Supply: 1,000,000,000,000,000

The bonding curve ensures price discovery and continuous liquidity for the token.

All fees are directed to the protocol's multisig wallet: `Br4NUsLoHRgAcxTBsDwgnejnjqMe5bkyio1YCrM3gWM2`

### Automated Liquidity Management

When the bonding curve accumulates 42.5 SOL:
1. X SOL is sent to the protocol multisig
2. Remaining SOL is used to seed a Meteora constant product liquidity pool
3. LP tokens are locked with claim authority assigned to the protocol multisig

## Administrative Roles

### Curve Creator
- Can initialize new bonding curves
- Sets initial parameters and optional whitelist
- Configures launch timing and initial purchases

### Admin
- Can modify protocol parameters
- Manages fee settings
- Controls whitelist status

### Fee Recipients
- Protocol Multisig (`Br4NUsLoHRgAcxTBsDwgnejnjqMe5bkyio1YCrM3gWM2`)
  - Receives trading fees
  - Has authority over locked LP tokens
  - Receives swapped USDC from liquidity migrations

## Creating a Bonding Curve

To create a new bonding curve:

1. Initialize curve parameters
2. Optional: Enable whitelist
3. Set launch timing
4. Configure initial purchases

Trading is enabled along the bonding curve until 42.5 SOL are raised and all 793,100,000 tokens are sold.

## Migration
Migration is a critical process that occurs once the bonding curve has completed and the tokens are empty. It involves:

1. Minting the remaining 206,900,000 tokens
2. Sending the experiment fee to the multisig wallet
3. A CPI (Cross-Program Invocation) call to create a Meteora Dynamic AMM (Automated Market Maker). It uses 206,900,000 matched with 42.5SOL - migration fee

4. A separate instruction CPI call that locks the liquidity in the AMM and creates an escrow from which the multisig can claim trading fees.

## 🛠 Installation

1. Clone the repository:

2. Install dependencies:
   ```
    - anchor : v0.31.0
    - solana : v1.18.18
    - rustc : v1.75.0 
   ```
## 📂 Project Structure

- `programs/pumpfun`: Contains the main smart contract code.
- `tests`: Automated test scripts for contract functionality.
- `migrations`: Deployment scripts for managing updates.
- `scripts`: Useful utilities for interacting with the contract.

## 🔧 Configuration

- Update the `Anchor.toml` file to set your Solana cluster (e.g., Devnet or Mainnet) and wallet details:

   ```
    [provider]
    cluster = "https://api.devnet.solana.com"
    wallet = "~/.config/solana/id.json"
   ```
- Ensure you have Solana CLI installed and configured:
   ```
    solana config set --url https://api.devnet.solana.com
   ```

## 📜 Usage
- Interact with the smart contract: Use the provided scripts or integrate with a frontend to interact with the PumpFun smart contract.
- Testing: Run unit tests to validate the functionality 
- Deploy to Mainnet: Ensure all tests pass, and then update the deployment configuration to target the mainnet cluster.

## 🤝 Proof of work & Collaboration

Mainnet CA: [`9MHPjXpZXgJrB4NiJVFStE5qy7Nqp7yaYpaqNe5jNfMw`](https://solscan.io/account/9MHPjXpZXgJrB4NiJVFStE5qy7Nqp7yaYpaqNe5jNfMw)

Devnet CA: [`9MHPjXpZXgJrB4NiJVFStE5qy7Nqp7yaYpaqNe5jNfMw`](https://solscan.io/account/9MHPjXpZXgJrB4NiJVFStE5qy7Nqp7yaYpaqNe5jNfMw?cluster=devnet)

Create Bonding Curve Tx: [`32JZPjV9JAakkvJyqQMpFhW8ziNqZV3QN2jbsY1W6GRsuDtq69HPtXsQE1RpAyUE2rw1DW5usozh4oueWqb1aM2S`](https://solscan.io/tx/32JZPjV9JAakkvJyqQMpFhW8ziNqZV3QN2jbsY1W6GRsuDtq69HPtXsQE1RpAyUE2rw1DW5usozh4oueWqb1aM2S?cluster=devnet)

Swap (involved team fee) Tx: [`TrzFL5aMu6vGEdFHhpHiDQhYrCYEmo4swSDLrN7avxSQwP3LrweSMPFTDZYmVYMwVCsTHSKWRxJkt9xXboQAwi8`](https://solscan.io/tx/TrzFL5aMu6vGEdFHhpHiDQhYrCYEmo4swSDLrN7avxSQwP3LrweSMPFTDZYmVYMwVCsTHSKWRxJkt9xXboQAwi8?cluster=devnet) 

Meteora Migration Tx: [`496nHgdFz5QYR4ah1pTsmnkQJSMiF8Z3FaJhSNgQKaHp5UFL1rQekdEoUG7vWRozSKg3YHJivQsHZk7b8TxfnUVU`](https://solscan.io/tx/496nHgdFz5QYR4ah1pTsmnkQJSMiF8Z3FaJhSNgQKaHp5UFL1rQekdEoUG7vWRozSKg3YHJivQsHZk7b8TxfnUVU?cluster=devnet)








