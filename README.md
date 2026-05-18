# Stellar Family Tree DApp

**Stellar Family Tree DApp** - Blockchain-Based Decentralized Family Lineage Management System

---

## Project Description

Stellar Family Tree DApp is a decentralized smart contract application built on the Stellar blockchain using the Soroban SDK. The system is designed to securely manage family lineage data, enabling users to create, search, update, and maintain family relationships directly on-chain.

The smart contract stores family members as unique individuals identified by IDs and names, while also preserving important personal information such as birth dates, gender, and life status (alive or deceased). Parent-child relationships are permanently recorded to build a transparent and verifiable family tree structure.

This decentralized approach removes reliance on centralized databases and introduces immutable family lineage documentation that can potentially support future government-scale digital family registry systems.

The system allows users to:

- Create family members
- Track parent-child relationships
- Search family members by ID or name
- Update member life status
- Remove members from the family tree
- Retrieve the full family lineage dataset

All data is securely stored within Soroban smart contract instance storage on the Stellar blockchain.

---

## Project Vision

Our vision is to modernize family lineage documentation and identity systems through decentralized blockchain technology by:

- **Decentralizing Family Records**  
  Eliminating dependency on centralized registry systems by storing lineage data on a distributed blockchain.

- **Ensuring Data Integrity**  
  Creating immutable family records that cannot be manipulated or altered without authorization.

- **Preserving Lineage History**  
  Building a permanent historical archive of family relationships across generations.

- **Enhancing Transparency**  
  Allowing verifiable and auditable lineage tracking while maintaining trustless infrastructure.

- **Supporting Government Digitalization**  
  Preparing a scalable foundation for future blockchain-based family card systems and population management.

- **Empowering Digital Identity**  
  Enabling secure family-based identity verification systems integrated with decentralized identity protocols.

We envision a future where lineage records become globally accessible, transparent, secure, and permanently preserved through blockchain technology.

---

## Key Features

### 1. **Family Member Registration**

- Create family members with:
  - Name
  - Birth date
  - Gender
- Automatic unique ID generation
- Persistent blockchain storage

---

### 2. **Parent-Child Relationship Management**

- Link members as parents or children
- Support multi-generation lineage tracking
- Transparent relationship structure
- Immutable ancestry records

---

### 3. **Life Status Tracking**

- Mark members as:
  - Alive
  - Deceased
- Store official death dates
- Update life status dynamically

---

### 4. **Family Member Search**

- Search members by:
  - Unique ID
  - Name
- Efficient on-chain retrieval
- Structured data for frontend integration

---

### 5. **Member Removal**

- Remove members by:
  - ID
  - Name
- Immediate storage update
- Simplified family data management

---

### 6. **Blockchain Transparency & Security**

- Immutable lineage documentation
- Transparent relationship tracking
- Blockchain-based verification
- Resistant to unauthorized modifications

---

### 7. **Stellar Network Integration**

- Powered by the Stellar blockchain
- Built using Soroban Smart Contract SDK
- Fast and low-cost transactions
- Scalable decentralized architecture

---

## Contract Details

- ID smartcontact testnet
CCL4JMDKEU5GKHR7QWFGIEY2YAFJSKNHEX3R7XX4WZTLXPDDJYVRWPAC

<img width="1723" height="738" alt="Image" src="https://github.com/user-attachments/assets/b907c2f6-5ea8-4bb1-894a-d40b94c36a3b" />

---

## Smart Contract Functions

### Member Management

- `create_member()`
  - Create a new family member

- `get_members()`
  - Retrieve all family members

- `search_by_id()`
  - Search a member using unique ID

- `search_by_name()`
  - Search a member using name

- `remove_member_by_id()`
  - Delete member using ID

- `remove_member_by_name()`
  - Delete member using name

---

### Relationship Management

- `add_parent_child_relation()`
  - Link parent and child relationships

---

### Life Status Management

- `mark_as_deceased()`
  - Mark member as deceased with death date

- `mark_as_alive()`
  - Restore member status to alive

---

## Example Family Member Structure

```rust
FamilyMember {
    id: 12345,
    name: "John Doe",
    birth_date: "1990-05-20",
    gender: "Male",
    is_alive: true,
    death_date: "",
    parent_ids: [...],
    child_ids: [...],
}
