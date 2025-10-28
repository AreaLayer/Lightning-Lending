# ⚡ Lightning Lending (Archived)

> **Status:** 🗄️ *This project is no longer actively maintained.*
> The repository is kept for reference and educational purposes only.
> Use at your own risk.

---

## 🧩 Overview

**Lightning Lending** was an experimental project exploring **lending mechanisms directly on the Bitcoin Lightning Network** using **Nostr** and **Discreet Log Contracts (DLCs)**.
It enabled opening and closing Lightning channels through programmable contracts and relays, paving the way for non-custodial lending protocols built on Lightning.

---

## ⚙️ How It Worked

1. A lending channel was opened between two peers (A and B) using **DLCs** and **Nostr relays**.
2. The protocol defined **HTLC** and **CLTV** parameters through DLC agreements.
3. Once the counterparties agreed and paid in sats, the channel was created or settled automatically.
4. Closing followed the same trustless mechanism, ensuring on-chain enforcement when needed.

---

## 🔗 Protocols & Standards

The project was designed to integrate with emerging **Nostr Improvement Proposals (NIPs)** and **Bitcoin Lightning specifications**, including experimental work with:

* **NIP-88** crates
* **DLCs (Discreet Log Contracts)** for on-chain enforcement
* **Lightning HTLC/CLTV logic**

*(Originally listed references under “Check here” are no longer active.)*

---

## 🧪 Development Notes

**Lightning Lending** was a proof-of-concept and **beta software**.
It was never intended for production use or large funds.
Testing was ongoing with the following roadmap:

* ✅ Basic channel open/close logic
* 🧩 DLC integration (Work in Progress)
* 🧪 More testing and NIP compatibility
* 🔄 Transition from beta
* 🌀 Future: Ark swaps integration

---

## 🧠 Concepts Explored

* **Bitcoin Lightning Network**
* **Nostr protocol**
* **Non-custodial lending**
* **DLC-based financial contracts**

---

## 👥 Contributors

See the original repository contributors page:
👉 [github.com/AreaLayer/Lightning-Lending](https://github.com/AreaLayer/Lightning-Lending)

---

## 📜 License

**MIT License** — Free to use, modify, and distribute with attribution.

---
