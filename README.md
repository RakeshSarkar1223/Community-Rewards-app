<img width="1920" height="1080" alt="Screenshot 2026-03-27 134423" src="https://github.com/user-attachments/assets/f0400994-1bca-4045-a943-2edc75f687cf" />

# 🌟 Community Rewards - Soroban Smart Contract

## 📌 Project Description

Community Rewards is a simple decentralized smart contract built on the Stellar Soroban platform. It enables communities to reward users with points for their contributions, participation, or achievements in a transparent and tamper-proof way.

This contract is designed as a foundational system that can be extended into full reward ecosystems like loyalty programs, DAO incentives, or gamified platforms.

---

## ⚙️ What It Does

* Allows an admin to assign reward points to users
* Stores user rewards securely on-chain
* Enables anyone to check reward balances
* Ensures only authorized admin can distribute rewards

---

## ✨ Features

### 🔐 Admin-Controlled Rewards

* Only the authorized admin can assign reward points
* Prevents unauthorized reward manipulation

### 👤 User Reward Tracking

* Each user has a reward balance stored on-chain
* Easy retrieval of reward points

### ⚡ Lightweight & Efficient

* Minimal logic for fast execution on Soroban
* Uses efficient storage with `Map<Address, u32>`

### 🔍 Transparent System

* All reward allocations are verifiable on-chain
* Promotes trust in community systems

### 🚀 Extensible Design

This contract can be extended with:

* Token redemption systems
* NFT rewards
* Tier-based rewards
* Expiry mechanisms
* Multiple admins or DAO governance

---

## 🛠️ Future Improvements

* Add role-based access control (RBAC)
* Introduce reward history logs
* Add reward claiming or redeeming functionality
* Integrate with frontend dashboard

---

## 📦 Tech Stack

* Rust
* Soroban SDK (Stellar Smart Contracts)

---

## 📜 License

MIT License




