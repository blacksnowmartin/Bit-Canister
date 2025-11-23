# Satoshi Vault - Bitcoin Inheritance and Recovery Vault

## Overview
Satoshi Vault is a fully on-chain Bitcoin inheritance and recovery system that allows users to create secure vaults with automatic inheritance features using ckBTC and threshold ECDSA technology.

## Authentication
- Users authenticate via Internet Identity
- All vault operations require authenticated access

## Core Features

### Vault Management
- Users can create and manage Bitcoin inheritance vaults
- Each vault contains:
  - Primary Bitcoin address (custody managed on-chain)
  - Backup wallet address for emergency transfers
  - Configurable inactivity period (30, 90, or 180 days)
  - ckBTC balance tracking

### Inheritance System
- Backend automatically tracks user activity timestamps
- When inactivity period is exceeded, vault automatically transfers ckBTC to designated backup wallet
- Uses Threshold ECDSA for secure on-chain Bitcoin transaction signing
- All transfers are executed without external custodians

### Encrypted Messages (vetKeys Integration)
- Users can create encrypted messages or instructions tied to their vault
- Messages are only decryptable by the heir wallet when inheritance is triggered
- Encryption/decryption handled through vetKeys integration

### Dashboard Interface
- Display active vaults with current status
- Show countdown timer for remaining time before automatic transfer
- Activity log showing recent vault interactions
- Private encrypted messages section for heir communications

## Backend Data Storage
- User vault configurations (addresses, inactivity periods, creation dates)
- Activity timestamps for inactivity tracking
- ckBTC balance information
- Encrypted message data
- Transaction history and logs

## Backend Operations
- Monitor user activity and update timestamps
- Execute automatic ckBTC transfers when inactivity thresholds are met
- Generate and sign Bitcoin transactions using Threshold ECDSA
- Manage encrypted message storage and retrieval
- Handle ckBTC balance queries and updates

## Technical Integration
- ckBTC integration for Bitcoin balance management
- Threshold ECDSA for secure transaction signing
- vetKeys for message encryption/decryption
- All logic and storage maintained on-chain without external dependencies
