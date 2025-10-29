# 🪙 Satoshi Vault — The Bitcoin Dead Man Switch

## 💡 Overview
Satoshi Vault is a Bitcoin inheritance vault built on the **Internet Computer**.  
It uses **ckBTC** for native Bitcoin transfers and **Threshold ECDSA** for automated, secure transaction signing.  
If a user becomes inactive for a defined period, their funds are automatically transferred to a backup wallet — no intermediaries, no bridges, no wrapped BTC.

---

## 🚀 Features
- Secure deposit and withdrawal using ckBTC
- User inactivity timer
- Automated BTC transfer via Threshold ECDSA
- Optional encrypted notes using vetKeys
- Frontend built with React + Tailwind

---

## 🧠 Architecture
See `architecture.png` or diagram below:
![Satoshi Vault Architecture](satoshi_vault_architecture.png)

---

## ⚙️ Tech Stack
| Layer | Technology |
|-------|-------------|
| Backend | Rust ICP Canisters |
| Bitcoin Integration | ckBTC + Threshold ECDSA |
| Frontend | React + Tailwind |
| Storage | ICP Stable Memory |
| Privacy Layer | vetKeys (Encrypted Notes) |

---

## 🧩 Setup Instructions
```bash
dfx new satoshi_vault
cd satoshi_vault
dfx start
dfx deploy
