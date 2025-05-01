# Simple Real Estate Tokenizer

## 📌 Project Title
**82. Simple Real Estate Tokenizer**

## 📖 Project Description
A lightweight Soroban smart contract for registering, tokenizing, and transferring ownership of real estate properties on the blockchain. Each property is uniquely registered and can be tokenized by the rightful owner.

## 🌟 Project Vision
To simplify real estate management and ownership tracking by leveraging the transparency and immutability of blockchain technology. This project aims to make real estate assets more accessible and tradable in a decentralized environment.

## ✨ Key Features
- **Register Property**: Add new properties with basic details to the blockchain.
- **Tokenize Property**: Mark a property as tokenized, enabling future fractionalization or sale.
- **Transfer Ownership**: Easily change ownership from one user to another.
- **Property Lookup**: Fetch property metadata using its unique ID.

## ⚙️ Contract Details

### Contract Address: CBRG334FDZUPK7X73BVWOT7LMY6TWHHGHHYD3WYADUEAKYA343T4Q6LK

### 1. `register_property(owner, location, size_sqft) -> property_id`
Creates a new property record on the blockchain.

### 2. `tokenize_property(owner, property_id) -> bool`
Marks a property as tokenized. Only callable by the property's owner.

### 3. `transfer_property(current_owner, new_owner, property_id) -> bool`
Transfers the property to a new owner. Only callable by the current owner.

### 4. `get_property(property_id) -> Property`
Fetches the full record of the given property.

---

🛠 Built with [Soroban SDK](https://soroban.stellar.org/docs)
