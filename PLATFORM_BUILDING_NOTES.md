# Key Findings: Building on Internet Computer (ICP) Platform
## Analysis of Bit-Canister Codebase for Satoshi Vault Development

---

## 📋 Table of Contents
1. [Platform Overview](#platform-overview)
2. [Project Structure](#project-structure)
3. [Backend Architecture (Rust Canister)](#backend-architecture-rust-canister)
4. [Frontend Architecture (React + TypeScript)](#frontend-architecture-react--typescript)
5. [Bitcoin Integration Patterns](#bitcoin-integration-patterns)
6. [Authentication & Identity](#authentication--identity)
7. [State Management & Persistence](#state-management--persistence)
8. [Key Patterns & Best Practices](#key-patterns--best-practices)
9. [Critical Considerations for Satoshi Vault](#critical-considerations-for-satoshi-vault)
10. [Missing Components to Implement](#missing-components-to-implement)

---

## 🏗️ Platform Overview

### Internet Computer (ICP) Fundamentals
- **Canisters**: Smart contracts that run on ICP, written in Rust (backend) or Motoko
- **Network Support**: Mainnet, Testnet, Regtest (local development)
- **Update Calls**: Take 2-3 seconds to complete - important for UX design
- **Query Calls**: Fast, read-only operations (no state changes)
- **Cycles**: Cost model for computation (similar to gas in Ethereum)

### Development Environment
- **DFX**: Command-line tool for developing, deploying, and managing canisters
- **Local Development**: `dfx start --clean --enable-bitcoin --bitcoin-node 127.0.0.1:18444`
- **Bitcoin Node**: Required for local regtest network (Bitcoin Core)
- **Build Target**: `wasm32-unknown-unknown` for Rust canisters

---

## 📁 Project Structure

```
rust-bitcoinwallet/
├── src/
│   ├── backend/          # Rust canister (smart contract)
│   │   ├── src/
│   │   │   ├── lib.rs    # Main canister entry point
│   │   │   ├── btc.rs    # Bitcoin utilities (UTXO selection, fee estimation)
│   │   │   ├── p2tr.rs   # Taproot transaction building
│   │   │   ├── schnorr.rs # Schnorr signature handling
│   │   │   └── service/  # API endpoints
│   │   └── backend.did   # Candid interface definition
│   ├── frontend/         # React + TypeScript frontend
│   │   ├── components/   # UI components
│   │   ├── hooks/        # Custom React hooks
│   │   ├── actor.tsx     # Canister actor setup
│   │   └── App.tsx       # Main app component
│   └── internet_identity/ # Internet Identity declarations
├── dfx.json              # DFX configuration
├── Cargo.toml            # Rust workspace config
└── package.json          # Frontend dependencies
```

### Key Configuration Files

**dfx.json**:
- Defines canisters (backend, frontend, internet_identity)
- Backend: Custom build with `build.sh` script
- Frontend: Asset canister type with build step
- Init args: Network variant (mainnet/testnet/regtest)

**Cargo.toml** (Backend):
- Dependencies: `ic-cdk`, `candid`, `bitcoin`, `serde`
- Crate type: `cdylib` for WebAssembly
- Edition: 2021

**package.json** (Frontend):
- React 18 with Vite
- `@dfinity/agent`, `@dfinity/auth-client` for ICP integration
- `ic-use-actor`, `ic-use-internet-identity` for React hooks
- `@tanstack/react-query` for data fetching
- Tailwind CSS for styling

---

## 🔧 Backend Architecture (Rust Canister)

### Core Components

#### 1. **Initialization** (`lib.rs`)
```rust
#[init]
pub fn init(network: Network) {
    init_upgrade(network);
}

#[post_upgrade]
fn upgrade(network: Network) {
    init_upgrade(network);
}
```
- Canisters have init and post_upgrade hooks
- Network configuration passed at deployment time
- Thread-local storage for global state (non-persistent across upgrades)

#### 2. **Bitcoin Context** (`BitcoinContext`)
```rust
pub struct BitcoinContext {
    pub network: Network,              // ICP network enum
    pub bitcoin_network: bitcoin::Network, // Bitcoin library network enum
    pub key_name: &'static str,        // ECDSA key name for signing
}
```
- Different key names for local vs. deployed: `"dfx_test_key"` (regtest) vs `"test_key_1"` (mainnet/testnet)
- Stored in thread-local `Cell` (not persistent across upgrades)

#### 3. **State Management**
- **Thread-Local Storage**: `thread_local!` macro for in-memory state
  - Non-persistent: Lost on canister upgrade
  - Used for: Caching (Schnorr keys), runtime configuration
- **No Persistent Storage**: Current codebase doesn't use `stable` memory
  - For Satoshi Vault: Will need persistent storage for vault configurations

#### 4. **Bitcoin Integration APIs**

**Address Generation**:
- Uses Principal-derived derivation paths: `vec![principal.as_slice().to_vec()]`
- Taproot (P2TR) addresses with key-path spending only
- Schnorr signatures via ICP's Threshold ECDSA API

**Transaction Building** (`btc.rs`):
- UTXO selection: Greedy algorithm (oldest last) or single UTXO
- Fee estimation: Queries Bitcoin network for fee percentiles (50th percentile = median)
- Fallback: 2000 msat/vB (2 sat/vB) for regtest
- Change outputs: Only if above dust threshold (1000 satoshis)

**Transaction Signing** (`p2tr.rs`):
- Key-path spending for Taproot
- Iterative fee calculation (chicken-egg problem: need fee to build tx, need tx size to calculate fee)
- Mock signing for size estimation, real signing for broadcast

#### 5. **Service Endpoints** (`service/`)

**get_address**:
- Derives Schnorr public key from Principal
- Creates P2TR address (key-path only, no script path)
- Returns address as string

**get_balance**:
- Queries Bitcoin network via `bitcoin_get_balance` API
- Optional min_confirmations parameter
- Returns balance in satoshis

**send_btc**:
- Authenticated endpoint (requires non-anonymous principal)
- Gets UTXOs, builds transaction, signs, broadcasts
- Returns transaction ID

### Key Rust Patterns

#### Error Handling
- `Result<T, String>` return types
- String error messages (not structured errors)
- Candid serialization for cross-canister calls

#### Async/Await
- All Bitcoin API calls are async
- Uses `ic_cdk::` async functions for canister APIs
- `await` for management canister calls (Schnorr API, Bitcoin API)

#### Authentication
```rust
fn auth_guard() -> Result<(), String> {
    match ic_cdk::api::msg_caller() {
        caller if caller == Principal::anonymous() => {
            Err("Calls with the anonymous principal are not allowed.".to_string())
        }
        _ => Ok(()),
    }
}
```

---

## 🎨 Frontend Architecture (React + TypeScript)

### Component Structure

#### 1. **Authentication Flow**
- Internet Identity provider: `InternetIdentityProvider` from `ic-use-internet-identity`
- Login component: Calls `login()` function
- Identity state: Managed by provider, accessible via `useInternetIdentity()`
- Principal: `identity?.getPrincipal()` for user identification

#### 2. **Canister Actor Setup** (`actor.tsx`)
```typescript
const actorContext = createActorContext<_SERVICE>();
export const useActor = createUseActorHook<_SERVICE>(actorContext);

<ActorProvider
  canisterId={canisterId}
  context={actorContext}
  identity={identity}
  idlFactory={idlFactory}
>
```
- Actor pattern: Type-safe canister client
- Auto-generated from Candid interface
- Identity automatically attached to calls

#### 3. **Data Fetching** (React Query)
```typescript
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retryOnMount: false,
      retry: false,
      gcTime: Infinity,
      staleTime: Infinity
    }
  }
});
```
- **Important**: Disables retries and caching refresh
- Reason: Update calls are expensive (2-3 seconds), avoid unnecessary calls
- LocalStorage caching: Address cached in browser storage

#### 4. **Custom Hooks Pattern**

**useBtcAddress**:
- Caches address in localStorage: `btcAddress-${principal.toText()}`
- Only fetches if not cached
- Returns cached value immediately for better UX

**useBtcBalance**:
- No caching (fresh data needed)
- Polling not implemented (would be expensive)

**useHandleAgentError**:
- Handles delegation expiration
- Clears identity and shows toast on error
- Pattern: Centralized error handling

#### 5. **UI Components**
- Radix UI components (Dialog, Toast, etc.)
- Tailwind CSS for styling
- Lucide React for icons
- shadcn/ui pattern (components.json)

### Frontend Build Configuration

**vite.config.ts**:
- Alias: `@` -> `./src/frontend`
- Environment variables: `CANISTER_*`, `DFX_*`, `II_URL`
- Proxy: `/api` -> `http://127.0.0.1:4943` for local development
- Internet Identity URL: Different for local vs. mainnet

---

## ₿ Bitcoin Integration Patterns

### Current Implementation
- **Direct Bitcoin Integration**: Uses ICP's Bitcoin API directly
- **No ckBTC**: Current codebase doesn't use ckBTC (chain-key Bitcoin)
- **Taproot Addresses**: P2TR (Pay-to-Taproot) only
- **Key Path Spending**: Single Schnorr signature, no script paths

### ICP Bitcoin API Functions Used

1. **bitcoin_get_balance**: Query balance for an address
2. **bitcoin_get_utxos**: Get UTXOs for an address (with pagination support)
3. **bitcoin_get_current_fee_percentiles**: Get network fee data
4. **bitcoin_send_transaction**: Broadcast transaction to Bitcoin network

### Threshold ECDSA (Schnorr) API

1. **schnorr_public_key**: Derive public key from derivation path
   - Key ID: `{ name: "test_key_1", algorithm: Bip340secp256k1 }`
   - Derivation path: `Vec<Vec<u8>>` (Principal bytes)
2. **sign_with_schnorr**: Sign message with private key
   - BIP-341 support: Optional merkle root hash for script paths
   - Key-path spending: No merkle root

### Transaction Flow

1. **Get UTXOs**: Query Bitcoin network for spendable outputs
2. **Select UTXOs**: Greedy or single UTXO selection
3. **Estimate Fee**: Query fee percentiles, use median
4. **Build Transaction**: Create unsigned transaction with change output
5. **Calculate Fee Iteratively**: Build tx, estimate size, recalculate fee until stable
6. **Sign Transaction**: Use Schnorr API to sign each input
7. **Broadcast**: Send signed transaction to Bitcoin network

### Important Bitcoin Considerations

- **Coinbase Maturity**: 100 blocks before spending (regtest)
- **Manual Block Mining**: Regtest doesn't auto-mine blocks
- **Dust Threshold**: 1000 satoshis (outputs below this are uneconomical)
- **Transaction Version**: Version 2 for modern features
- **Lock Time**: Zero (no absolute timelock)
- **Sequence**: `ENABLE_RBF_NO_LOCKTIME` for Replace-By-Fee

---

## 🔐 Authentication & Identity

### Internet Identity
- **Provider**: `ic-use-internet-identity` package
- **Local Development**: `http://${CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`
- **Mainnet**: `https://identity.ic0.app`
- **Delegation Expiration**: Handled automatically, errors caught and user logged out

### Principal-Based Identification
- Each user has a unique Principal
- Principal used for:
  - Bitcoin address derivation
  - Authentication checks
  - State isolation (multi-user wallet)

### Authentication Guard
- Anonymous principal rejected for sensitive operations
- `msg_caller()` returns the calling principal
- Pattern: Check at start of update methods

---

## 💾 State Management & Persistence

### Current State (Non-Persistent)
- **Thread-Local Storage**: Runtime state only
- **Schnorr Key Cache**: In-memory HashMap (lost on upgrade)
- **Bitcoin Context**: Runtime configuration (lost on upgrade)
- **Frontend Caching**: localStorage for addresses (browser-side)

### For Satoshi Vault: Need Persistent Storage

#### Required Data Structures:
1. **Vault Configuration**:
   - Principal -> VaultConfig mapping
   - Backup wallet address
   - Inactivity period (days)
   - Last activity timestamp
   - Encrypted message (optional, using vetKeys)

2. **Activity Tracking**:
   - Principal -> LastActivityTimestamp mapping
   - Update on each user interaction

3. **Timer State**:
   - Principal -> TimerExpiryTimestamp mapping
   - Calculated: LastActivity + InactivityPeriod

#### Implementation Options:

**Option 1: Stable Memory (Recommended)**
```rust
use ic_stable_structures::{
    StableBTreeMap, StableCell, memory_manager::MemoryManager,
};

thread_local! {
    static VAULTS: RefCell<StableBTreeMap<Principal, VaultConfig>> = ...;
}
```
- Persistent across upgrades
- Requires `ic-stable-structures` crate
- Need to implement `pre_upgrade` and `post_upgrade` hooks

**Option 2: Canister Storage (Simpler)**
```rust
thread_local! {
    static VAULTS: RefCell<HashMap<Principal, VaultConfig>> = ...;
}

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize and store in stable memory
}

#[post_upgrade]
fn post_upgrade() {
    // Deserialize from stable memory
}
```
- Simpler but requires manual serialization
- Use `candid` for serialization

---

## 🎯 Key Patterns & Best Practices

### 1. **Update Calls are Slow (2-3 seconds)**
- Minimize update calls in UI
- Use query calls for read operations when possible
- Cache results in frontend (localStorage)
- Optimistic UI updates where appropriate

### 2. **Error Handling**
- Structured error types in Candid (variant { Ok; Err })
- Frontend: Check for `Err` variant before using `Ok`
- Agent errors: Handle delegation expiration gracefully

### 3. **Network Configuration**
- Different behavior for regtest/testnet/mainnet
- Key names differ by network
- Address validation: Ensure address matches network

### 4. **Fee Estimation**
- Iterative approach for accurate fees
- Use median fee percentile (50th) for balance
- Fallback for regtest (no fee data available)

### 5. **UTXO Management**
- Greedy selection: Consolidate older UTXOs
- Single UTXO: For specific use cases (inscriptions, etc.)
- Handle pagination for addresses with many UTXOs

### 6. **Principal Derivation**
- Use Principal bytes as derivation path
- Ensures unique addresses per user
- Deterministic: Same Principal = Same address

### 7. **Candid Interface**
- Define in `.did` file
- Auto-generate TypeScript types
- Version control the interface

---

## 🚨 Critical Considerations for Satoshi Vault

### 1. **Timer Mechanism (Dead Man Switch)**

**Challenge**: ICP canisters don't have native cron jobs or scheduled tasks.

**Solutions**:

**Option A: Heartbeat Pattern (Recommended)**
- External service calls `check_vaults()` periodically
- Canister checks all vaults, transfers if expired
- Requires external trigger (oracle, cron job, user action)

**Option B: Canister Timers (New Feature)**
- ICP recently added `set_timer` API
- Can set per-canister timers
- Check if available in your ICP version

**Option C: User-Driven Checks**
- Check on every user interaction
- Check when user deposits/withdraws
- Less reliable (requires user activity)

**Implementation**:
```rust
#[update]
async fn check_and_transfer_expired_vaults() -> Result<Vec<String>, String> {
    // Iterate through all vaults
    // Check if last_activity + inactivity_period < now
    // Transfer BTC to backup wallet
    // Return list of transferred vaults
}
```

### 2. **Activity Tracking**

**What Counts as Activity?**
- Login (Internet Identity authentication)
- Deposit BTC
- Withdraw BTC
- Update vault configuration
- View vault status (maybe?)

**Implementation**:
```rust
fn update_activity(principal: Principal) {
    let now = ic_cdk::api::time(); // Nanoseconds since epoch
    // Update last_activity timestamp
    // Recalculate timer expiry
}
```

### 3. **ckBTC Integration (Required for Satoshi Vault)**

**Current Codebase**: Uses direct Bitcoin integration, NOT ckBTC.

**ckBTC vs Direct Bitcoin**:
- **ckBTC**: Wrapped Bitcoin on ICP, faster transfers, lower fees
- **Direct Bitcoin**: Native Bitcoin, slower, higher fees
- **Satoshi Vault**: Should use ckBTC for better UX

**Implementation**:
- Use `ckBTC` ledger canister
- Transfer ckBTC instead of BTC
- Convert ckBTC to BTC when transferring to backup wallet (if needed)
- Or: Keep as ckBTC and transfer ckBTC to backup wallet

**ckBTC Ledger Canister**:
- Mainnet: `mxzaz-hqaaa-aaaar-qaada-cai`
- Methods: `icrc1_transfer`, `icrc1_balance_of`, etc.
- ICRC-2: Approved transfers (for auto-transfers)

### 4. **vetKeys Integration (Optional)**

**Purpose**: Encrypt private messages for heirs.

**Implementation**:
- Use `vetKeys` canister for encryption
- Store encrypted message in vault configuration
- Heir can decrypt using their identity
- Pattern: Encrypt with heir's public key

**vetKeys API**:
- Encrypt message with recipient's public key
- Store encrypted blob in canister
- Recipient decrypts with their private key

### 5. **Backup Wallet Validation**

**Requirements**:
- Validate Bitcoin address format
- Ensure address is valid for network (mainnet/testnet)
- Store address securely (no encryption needed, it's public)

**Implementation**:
```rust
fn validate_backup_address(address: &str, network: bitcoin::Network) -> Result<Address, String> {
    Address::from_str(address)
        .map_err(|e| format!("Invalid address: {}", e))?
        .require_network(network)
        .map_err(|e| format!("Address not valid for network: {}", e))
}
```

### 6. **Inactivity Period Configuration**

**Options**:
- Predefined periods: 30, 60, 90 days
- Custom period: User-defined days
- Validation: Minimum period (e.g., 7 days), maximum period (e.g., 365 days)

**Storage**:
- Store as nanoseconds: `inactivity_period_ns: u64`
- Calculate: `period_days * 24 * 60 * 60 * 1_000_000_000`

### 7. **Auto-Transfer Logic**

**Flow**:
1. Check if timer expired: `now > last_activity + inactivity_period`
2. Get vault balance (ckBTC or BTC)
3. Validate backup wallet address
4. Build transfer transaction
5. Sign and broadcast
6. Update vault state (mark as transferred, or delete)

**Error Handling**:
- What if transfer fails? (Retry mechanism?)
- What if backup address is invalid? (Notify user beforehand)
- What if balance is zero? (Skip transfer, but mark as expired)

### 8. **Security Considerations**

**Access Control**:
- Only vault owner can update configuration
- Only vault owner can withdraw (before timer expires)
- Auto-transfer: No user interaction required (canister-initiated)

**Principal Validation**:
- Always validate `msg_caller()` matches vault owner
- Reject anonymous principal for sensitive operations

**Backup Wallet**:
- User-provided address (trust assumption)
- Consider: Multi-sig backup wallet? (Complex, maybe v2 feature)

### 9. **Frontend Requirements**

**Dashboard**:
- Vault balance (ckBTC)
- Timer countdown (days/hours remaining)
- Last activity timestamp
- Backup wallet address
- Encrypted message status

**Actions**:
- Deposit ckBTC
- Withdraw ckBTC (before timer expires)
- Update inactivity period
- Update backup wallet
- Add/update encrypted message
- Manually trigger activity update

**UI Components**:
- Countdown timer (real-time updates)
- Deposit/withdraw dialogs
- Configuration form
- Activity log (optional)

### 10. **Testing Strategy**

**Local Development**:
- Use regtest network
- Mine blocks manually for confirmations
- Test timer expiration (mock time or fast-forward)
- Test transfer failures

**Testnet**:
- Deploy to testnet for integration testing
- Use testnet ckBTC
- Test with real Internet Identity

**Mainnet**:
- Security audit before mainnet deployment
- Start with small amounts
- Monitor for issues

---

## 🔨 Missing Components to Implement

### Backend (Rust Canister)

1. **Persistent Storage**
   - [ ] Install `ic-stable-structures` or implement manual serialization
   - [ ] Define `VaultConfig` struct
   - [ ] Implement `pre_upgrade` and `post_upgrade` hooks
   - [ ] Create vault storage (StableBTreeMap or HashMap)

2. **Vault Management**
   - [ ] `create_vault(backup_address, inactivity_period_days)` - Create new vault
   - [ ] `update_vault_config(...)` - Update configuration
   - [ ] `get_vault_status(principal)` - Get vault info and timer status
   - [ ] `update_activity()` - Update last activity timestamp

3. **Timer & Auto-Transfer**
   - [ ] `check_and_transfer_expired_vaults()` - Check all vaults, transfer if expired
   - [ ] `transfer_vault_to_backup(principal)` - Transfer single vault
   - [ ] Timer calculation logic
   - [ ] Transfer failure handling

4. **ckBTC Integration**
   - [ ] Add ckBTC ledger canister dependency
   - [ ] `deposit_ckbtc(amount)` - Accept ckBTC deposits
   - [ ] `withdraw_ckbtc(amount)` - Withdraw ckBTC (before timer expires)
   - [ ] `transfer_ckbtc_to_backup(principal)` - Auto-transfer on expiration
   - [ ] ICRC-2 approval for auto-transfers (if needed)

5. **vetKeys Integration** (Optional)
   - [ ] Add vetKeys canister dependency
   - [ ] `store_encrypted_message(encrypted_blob)` - Store encrypted message
   - [ ] `get_encrypted_message(principal)` - Retrieve encrypted message
   - [ ] Encryption/decryption helpers

### Frontend (React + TypeScript)

1. **Vault Dashboard**
   - [ ] Vault status component (balance, timer, last activity)
   - [ ] Countdown timer component (real-time updates)
   - [ ] Backup wallet display
   - [ ] Encrypted message status

2. **Vault Actions**
   - [ ] Create vault form
   - [ ] Deposit ckBTC dialog
   - [ ] Withdraw ckBTC dialog
   - [ ] Update configuration form
   - [ ] Add/update encrypted message form

3. **Hooks**
   - [ ] `useVaultStatus()` - Get vault status and timer
   - [ ] `useVaultBalance()` - Get ckBTC balance
   - [ ] `useCreateVault()` - Create new vault mutation
   - [ ] `useDepositCkBtc()` - Deposit mutation
   - [ ] `useWithdrawCkBtc()` - Withdraw mutation
   - [ ] `useUpdateVaultConfig()` - Update configuration mutation

4. **Timer Logic**
   - [ ] Calculate time remaining
   - [ ] Format countdown (days, hours, minutes)
   - [ ] Real-time updates (polling or WebSocket if available)
   - [ ] Expiration notifications

### Configuration

1. **DFX Configuration**
   - [ ] Add ckBTC ledger canister to `dfx.json`
   - [ ] Add vetKeys canister (if using)
   - [ ] Update init args if needed

2. **Dependencies**
   - [ ] Backend: Add `ic-stable-structures` (or manual serialization)
   - [ ] Backend: Add ckBTC ledger interface
   - [ ] Backend: Add vetKeys interface (if using)
   - [ ] Frontend: Add ckBTC UI components
   - [ ] Frontend: Add timer/countdown components

### Testing & Deployment

1. **Local Testing**
   - [ ] Test vault creation
   - [ ] Test deposit/withdraw
   - [ ] Test timer expiration (mock time)
   - [ ] Test auto-transfer
   - [ ] Test configuration updates

2. **Testnet Deployment**
   - [ ] Deploy to testnet
   - [ ] Test with testnet ckBTC
   - [ ] Test with real Internet Identity
   - [ ] Test timer with real time (long-running test)

3. **Mainnet Deployment**
   - [ ] Security audit
   - [ ] Deploy to mainnet
   - [ ] Monitor for issues
   - [ ] User acceptance testing

---

## 📚 Additional Resources

### ICP Documentation
- [ICP Bitcoin Integration](https://internetcomputer.org/docs/build-on-btc/)
- [ICP Bitcoin API Reference](https://internetcomputer.org/docs/current/references/bitcoin-api/)
- [Threshold ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key)
- [Stable Memory](https://internetcomputer.org/docs/current/developer-docs/build/cdks/motoko-dfinity/stability)
- [Canister Timers](https://internetcomputer.org/docs/current/developer-docs/build/cdks/motoko-dfinity/timers) (if available)

### Bitcoin Documentation
- [BIP-341: Taproot](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
- [BIP-340: Schnorr Signatures](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)
- [Bitcoin Core RPC](https://bitcoincore.org/en/doc/)

### ckBTC Documentation
- [ckBTC Overview](https://internetcomputer.org/docs/current/developer-docs/defi/ckbtc/)
- [ckBTC Ledger Canister](https://github.com/dfinity/ic)
- [ICRC-2 Standard](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-2/ICRC-2.md)

### vetKeys Documentation
- [vetKeys Overview](https://internetcomputer.org/docs/current/developer-docs/integrations/vetkeys/)
- [vetKeys API](https://github.com/dfinity/vet Keys)

---

## ✅ Summary Checklist for Satoshi Vault

### Must-Have Features
- [x] Study existing codebase structure
- [ ] Implement persistent storage for vault configurations
- [ ] Implement activity tracking (last login timestamp)
- [ ] Implement inactivity timer logic
- [ ] Implement auto-transfer on timer expiration
- [ ] Integrate ckBTC (deposit, withdraw, transfer)
- [ ] Implement backup wallet validation
- [ ] Create vault management API (create, update, get status)
- [ ] Build frontend dashboard with timer countdown
- [ ] Build deposit/withdraw UI
- [ ] Build configuration UI

### Nice-to-Have Features
- [ ] vetKeys integration for encrypted messages
- [ ] Multi-sig backup wallet support
- [ ] Activity log/history
- [ ] Email/SMS notifications (requires external service)
- [ ] Multiple backup wallets (failover)
- [ ] Partial withdrawals (keep some funds in vault)

### Security & Testing
- [ ] Security audit
- [ ] Unit tests for vault logic
- [ ] Integration tests for ckBTC transfers
- [ ] Timer expiration testing
- [ ] Failure scenario testing
- [ ] Mainnet deployment checklist

---

## 🎯 Key Takeaways

1. **Platform**: Internet Computer (ICP) with Rust canisters and React frontend
2. **Bitcoin Integration**: Currently uses direct Bitcoin API, but Satoshi Vault should use ckBTC
3. **State Management**: Need persistent storage (stable memory) for vault configurations
4. **Timer Mechanism**: No native cron jobs - need heartbeat pattern or canister timers
5. **Authentication**: Internet Identity for user authentication, Principal for identification
6. **Update Calls**: Slow (2-3 seconds) - minimize calls, cache in frontend
7. **Error Handling**: Use Result types, handle delegation expiration
8. **Network Configuration**: Different behavior for regtest/testnet/mainnet
9. **Fee Estimation**: Iterative approach for accurate Bitcoin transaction fees
10. **Principal Derivation**: Use Principal bytes for deterministic Bitcoin address generation

---

**Generated**: Based on analysis of Bit-Canister codebase
**Purpose**: Guide for building Satoshi Vault on ICP platform
**Next Steps**: Start implementing persistent storage and vault management API

