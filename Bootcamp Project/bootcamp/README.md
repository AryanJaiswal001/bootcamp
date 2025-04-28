# Decentralized Art Gallery

## 📌 Project Title
**Decentralized Art Gallery**

## 🖼 Project Description
A blockchain-powered platform allowing artists to register and showcase digital artworks immutably. Each entry is tied to the artist's wallet address and timestamped on the Stellar Soroban blockchain.

## 🌐 Project Vision
To create a transparent, censorship-resistant, and globally accessible digital art space where creators can assert ownership and share their work securely and independently.

## ✨ Key Features
- Artist-authenticated artwork submissions
- Immutable and timestamped records of digital art
- Artwork browsing via ID
- Minimal and gas-efficient contract logic

## 🔍 Contract Details

### Contract Address: CCJNAKXG2JILFC7ZM2BHZSWEI3UCCFTENGT6EGKPEU4L4Z5665GXF353 

### `add_artwork(env, creator, title, description) -> u64`
Registers a new piece of art. Requires authentication from the creator. Returns the artwork ID.

### `get_artwork(env, id) -> Artwork`
Fetches artwork details by ID.

### `get_artwork_count(env) -> u64`
Returns the current total number of artworks in the gallery.

---

> Built with 💙 using [Soroban SDK](https://soroban.stellar.org/)

