# 🏠 Stellar Soroban Real Estate Smart Contract
<img width="1919" height="877" alt="image" src="https://github.com/user-attachments/assets/b940864d-ed68-4806-b6b3-9527941607b6" />

## 📌 Project Description

This project is a decentralized real estate management system built using Soroban smart contracts on the Stellar blockchain. It enables secure property registration and ownership transfer without relying on intermediaries, ensuring transparency and trust.

---

## ⚙️ What it does

* Allows users to register real estate properties on-chain
* Stores ownership details securely using blockchain technology
* Enables authorized transfer of property ownership
* Provides transparent and immutable property records

---

## ✨ Features

* 🏡 **Property Registration** — Register properties with unique IDs
* 🔐 **Ownership Verification** — Only owners can control their property
* 🔄 **Ownership Transfer** — Secure and permission-based transfer system
* 📜 **Immutable Records** — Data stored permanently on blockchain
* ⚡ **Low Cost & Fast Transactions** — Powered by Stellar network

---

## 🛠️ Tech Stack

* **Blockchain:** Stellar
* **Smart Contracts:** Soroban (Rust)
* **Language:** Rust

---

## 🚀 How It Works

1. A user registers a property with a unique ID and metadata
2. The contract stores ownership details on-chain
3. Only the registered owner can initiate a transfer
4. Ownership is updated securely and permanently

---

## 📦 Functions

### `register_property`

Registers a new property on the blockchain

### `get_property`

Fetches property details using property ID

### `transfer_property`

Transfers ownership to another user

---

## 🔗 Deployed Smart Contract Link

```
CAQI5ELW3LJIBI327KKDVLL2PGY367HHRYGMQXBNUBDGVJ4WQWTZ3H64
```

---

## 🧪 Build & Deploy

### Build Contract

```bash
stellar contract build
```

### Deploy Contract

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source <your-key>
```

---

## 🌱 Future Improvements

* 💰 Property buying/selling with payments
* 🧾 Smart escrow system
* 🪙 Tokenization (NFT-based ownership)
* 👥 Multi-owner support
* 🌐 Frontend dashboard integration

---

## 🤝 Contribution

Feel free to fork this project and improve it. Contributions are welcome!

---

## 📄 License

This project is open-source and available under the MIT License.
