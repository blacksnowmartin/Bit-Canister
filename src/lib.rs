use ic_cdk::api::{management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, SignWithEcdsaArgument,
    EcdsaPublicKeyArgument,
}, time};
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

// Constants
const NANOS_PER_SECOND: u64 = 1_000_000_000;
const SECONDS_PER_DAY: u64 = 86_400;
const NANOS_PER_DAY: u64 = NANOS_PER_SECOND * SECONDS_PER_DAY;
const ECDSA_KEY_ID: &str = "secp256k1";
const MIN_TIMEOUT_DAYS: u64 = 1;
const MAX_TIMEOUT_DAYS: u64 = 3650; // 10 years max

// Error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VaultError {
    VaultNotFound,
    Unauthorized,
    InvalidInput(String),
    TransferFailed(String),
    InsufficientBalance,
    VaultAlreadyExists,
    InvalidBitcoinAddress,
    TimeoutTooShort,
    TimeoutTooLong,
    ThresholdEcdsaError(String),
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::VaultNotFound => write!(f, "Vault not found"),
            VaultError::Unauthorized => write!(f, "Unauthorized access"),
            VaultError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            VaultError::TransferFailed(msg) => write!(f, "Transfer failed: {}", msg),
            VaultError::InsufficientBalance => write!(f, "Insufficient balance"),
            VaultError::VaultAlreadyExists => write!(f, "Vault already exists"),
            VaultError::InvalidBitcoinAddress => write!(f, "Invalid Bitcoin address"),
            VaultError::TimeoutTooShort => write!(f, "Timeout too short (minimum {} days)", MIN_TIMEOUT_DAYS),
            VaultError::TimeoutTooLong => write!(f, "Timeout too long (maximum {} days)", MAX_TIMEOUT_DAYS),
            VaultError::ThresholdEcdsaError(msg) => write!(f, "Threshold ECDSA error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, VaultError>;

// Vault struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vault {
    pub owner: candid::Principal,
    pub backup_wallet: String,
    pub last_check_in: u64,
    pub timeout_days: u64,
    pub balance_satoshi: u64,
    pub encrypted_note: Option<String>,
    pub created_at: u64,
}

impl Storable for Vault {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(bincode::serialize(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        bincode::deserialize(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Principal wrapper for Storable
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PrincipalBytes(Vec<u8>);

impl From<candid::Principal> for PrincipalBytes {
    fn from(p: candid::Principal) -> Self {
        PrincipalBytes(p.as_slice().to_vec())
    }
}

impl From<PrincipalBytes> for candid::Principal {
    fn from(pb: PrincipalBytes) -> Self {
        candid::Principal::from_slice(&pb.0)
    }
}

impl Storable for PrincipalBytes {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        PrincipalBytes(bytes.to_vec())
    }

    const BOUND: Bound = Bound::Unbounded;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static VAULTS: RefCell<StableBTreeMap<u64, Vault, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static VAULT_INDEX: RefCell<StableBTreeMap<PrincipalBytes, u64, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

// Helper function to get caller principal
fn caller() -> candid::Principal {
    ic_cdk::caller()
}

// Helper function to validate Bitcoin address (basic validation)
fn validate_bitcoin_address(address: &str) -> bool {
    // Basic validation: check length and character set
    // Real implementation should use proper Bitcoin address validation library
    if address.is_empty() || address.len() > 100 {
        return false;
    }
    // Check for valid base58 characters (simplified)
    address.chars().all(|c| {
        c.is_alphanumeric() && 
        !matches!(c, '0' | 'O' | 'I' | 'l')
    })
}

// Helper function to validate timeout
fn validate_timeout(timeout_days: u64) -> Result<()> {
    if timeout_days < MIN_TIMEOUT_DAYS {
        return Err(VaultError::TimeoutTooShort);
    }
    if timeout_days > MAX_TIMEOUT_DAYS {
        return Err(VaultError::TimeoutTooLong);
    }
    Ok(())
}

// Get next vault ID
fn get_next_vault_id() -> u64 {
    VAULTS.with(|vaults| {
        let mut max_id = 0u64;
        for (id, _) in vaults.borrow().iter() {
            if id > max_id {
                max_id = id;
            }
        }
        max_id + 1
    })
}

// Create a new vault
#[update]
fn create_vault(
    backup_wallet: String,
    timeout_days: u64,
    encrypted_note: Option<String>,
) -> Result<u64> {
    let caller = caller();
    
    // Validate inputs
    if !validate_bitcoin_address(&backup_wallet) {
        return Err(VaultError::InvalidBitcoinAddress);
    }
    validate_timeout(timeout_days)?;
    
    // Check if vault already exists for this caller
    let caller_bytes = PrincipalBytes::from(caller);
    VAULT_INDEX.with(|index| {
        if index.borrow().contains_key(&caller_bytes) {
            return Err(VaultError::VaultAlreadyExists);
        }
        Ok(())
    })?;
    
    // Create vault
    let vault_id = get_next_vault_id();
    let vault = Vault {
        owner: caller,
        backup_wallet,
        last_check_in: time(),
        timeout_days,
        balance_satoshi: 0,
        encrypted_note,
        created_at: time(),
    };
    
    // Store vault
    VAULTS.with(|vaults| {
        vaults.borrow_mut().insert(vault_id, vault.clone());
    });
    
    VAULT_INDEX.with(|index| {
        index.borrow_mut().insert(caller_bytes, vault_id);
    });
    
    ic_cdk::println!("Created vault {} for owner {}", vault_id, caller);
    Ok(vault_id)
}

// Get vault information
#[query]
fn get_vault() -> Result<Vault> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    VAULTS.with(|vaults| {
        vaults.borrow().get(&vault_id).ok_or(VaultError::VaultNotFound)
    })
}

// Get vault by ID (for admin/query purposes)
#[query]
fn get_vault_by_id(vault_id: u64) -> Result<Vault> {
    let caller = caller();
    
    let vault = VAULTS.with(|vaults| {
        vaults.borrow().get(&vault_id).ok_or(VaultError::VaultNotFound)
    })?;
    
    // Only owner can query their vault
    if vault.owner != caller {
        return Err(VaultError::Unauthorized);
    }
    
    Ok(vault)
}

// Check in to reset the timer
#[update]
fn check_in() -> Result<()> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    VAULTS.with(|vaults| {
        let mut vaults = vaults.borrow_mut();
        let mut vault = vaults.get(&vault_id).ok_or(VaultError::VaultNotFound)?;
        
        if vault.owner != caller {
            return Err(VaultError::Unauthorized);
        }
        
        vault.last_check_in = time();
        vaults.insert(vault_id, vault);
        Ok(())
    })
}

// Update vault settings
#[update]
fn update_vault(
    backup_wallet: Option<String>,
    timeout_days: Option<u64>,
    encrypted_note: Option<String>,
) -> Result<()> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    VAULTS.with(|vaults| {
        let mut vaults = vaults.borrow_mut();
        let mut vault = vaults.get(&vault_id).ok_or(VaultError::VaultNotFound)?;
        
        if vault.owner != caller {
            return Err(VaultError::Unauthorized);
        }
        
        if let Some(backup) = backup_wallet {
            if !validate_bitcoin_address(&backup) {
                return Err(VaultError::InvalidBitcoinAddress);
            }
            vault.backup_wallet = backup;
        }
        
        if let Some(timeout) = timeout_days {
            validate_timeout(timeout)?;
            vault.timeout_days = timeout;
        }
        
        if let Some(note) = encrypted_note {
            vault.encrypted_note = Some(note);
        }
        
        vaults.insert(vault_id, vault);
        Ok(())
    })
}

// ckBTC Ledger Principal (mainnet)
const CKBTC_LEDGER: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

// Deposit ckBTC into vault using ICRC-2 transfer_from
#[update]
async fn deposit_ckbtc(amount: u64) -> Result<()> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    if amount == 0 {
        return Err(VaultError::InvalidInput("Amount must be greater than zero".to_string()));
    }
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    // Get canister principal (vault address)
    let canister_id = ic_cdk::id();
    
    // In a real implementation, this would:
    // 1. Call ckBTC ledger's icrc2_transfer_from to transfer from user to canister
    // 2. Verify the transfer was successful
    // 3. Update the vault balance
    
    // For now, we simulate the deposit
    // Real implementation would use:
    // ic_cdk::api::call::call(
    //     Principal::from_text(CKBTC_LEDGER).unwrap(),
    //     "icrc2_transfer_from",
    //     ...
    // ).await
    
    VAULTS.with(|vaults| {
        let mut vaults = vaults.borrow_mut();
        let mut vault = vaults.get(&vault_id).ok_or(VaultError::VaultNotFound)?;
        
        if vault.owner != caller {
            return Err(VaultError::Unauthorized);
        }
        
        vault.balance_satoshi += amount;
        vault.last_check_in = time();
        vaults.insert(vault_id, vault);
        ic_cdk::println!("Deposited {} satoshis into vault {}", amount, vault_id);
        Ok(())
    })
}

// Manual withdrawal - withdraw ckBTC from vault
#[update]
async fn withdraw_ckbtc(amount: u64, destination: candid::Principal) -> Result<()> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    if amount == 0 {
        return Err(VaultError::InvalidInput("Amount must be greater than zero".to_string()));
    }
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    VAULTS.with(|vaults| {
        let mut vaults = vaults.borrow_mut();
        let mut vault = vaults.get(&vault_id).ok_or(VaultError::VaultNotFound)?;
        
        if vault.owner != caller {
            return Err(VaultError::Unauthorized);
        }
        
        if vault.balance_satoshi < amount {
            return Err(VaultError::InsufficientBalance);
        }
        
        vault.balance_satoshi -= amount;
        vault.last_check_in = time();
        vaults.insert(vault_id, vault.clone());
        Ok(())
    })?;
    
    // In a real implementation, this would call ckBTC ledger's icrc1_transfer
    // to transfer ckBTC from canister to destination principal
    // Real implementation would use:
    // ic_cdk::api::call::call(
    //     Principal::from_text(CKBTC_LEDGER).unwrap(),
    //     "icrc1_transfer",
    //     (destination, amount, ...)
    // ).await
    
    ic_cdk::println!("Withdrawing {} satoshis (ckBTC) to {}", amount, destination);
    
    Ok(())
}

// Manual withdrawal to Bitcoin address using Threshold ECDSA
#[update]
async fn withdraw_to_bitcoin(amount: u64, destination: String) -> Result<()> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    if amount == 0 {
        return Err(VaultError::InvalidInput("Amount must be greater than zero".to_string()));
    }
    
    if !validate_bitcoin_address(&destination) {
        return Err(VaultError::InvalidBitcoinAddress);
    }
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    VAULTS.with(|vaults| {
        let mut vaults = vaults.borrow_mut();
        let mut vault = vaults.get(&vault_id).ok_or(VaultError::VaultNotFound)?;
        
        if vault.owner != caller {
            return Err(VaultError::Unauthorized);
        }
        
        if vault.balance_satoshi < amount {
            return Err(VaultError::InsufficientBalance);
        }
        
        vault.balance_satoshi -= amount;
        vault.last_check_in = time();
        vaults.insert(vault_id, vault.clone());
        Ok(())
    })?;
    
    // Transfer Bitcoin using Threshold ECDSA
    transfer_bitcoin(destination, amount).await?;
    
    Ok(())
}

// Trigger expired transfers
#[update]
async fn trigger_expired_transfers() -> Result<Vec<u64>> {
    let now = time();
    let mut processed_vaults = Vec::new();
    
    // Collect expired vaults
    let expired_vaults: Vec<(u64, Vault)> = VAULTS.with(|vaults| {
        vaults.borrow()
            .iter()
            .filter_map(|(id, vault)| {
                let elapsed_nanos = now.saturating_sub(vault.last_check_in);
                let elapsed_days = elapsed_nanos / NANOS_PER_DAY;
                
                if elapsed_days > vault.timeout_days && vault.balance_satoshi > 0 {
                    Some((id, vault.clone()))
                } else {
                    None
                }
            })
            .collect()
    });
    
    // Process each expired vault
    for (vault_id, mut vault) in expired_vaults {
        ic_cdk::println!(
            "Processing expired vault {}: transferring {} satoshis to {}",
            vault_id, vault.balance_satoshi, vault.backup_wallet
        );
        
        // Transfer funds using Threshold ECDSA
        let transfer_result = transfer_bitcoin(
            vault.backup_wallet.clone(),
            vault.balance_satoshi,
        ).await;
        
        match transfer_result {
            Ok(_) => {
                // Update vault balance to zero
                VAULTS.with(|vaults| {
                    vault.balance_satoshi = 0;
                    vaults.borrow_mut().insert(vault_id, vault);
                });
                processed_vaults.push(vault_id);
            }
            Err(e) => {
                ic_cdk::println!("Failed to transfer from vault {}: {}", vault_id, e);
            }
        }
    }
    
    Ok(processed_vaults)
}

// Transfer Bitcoin using Threshold ECDSA
async fn transfer_bitcoin(destination: String, amount_satoshi: u64) -> Result<()> {
    // This is a simplified implementation
    // In a real implementation, you would:
    // 1. Get the canister's Bitcoin address using ecdsa_public_key
    // 2. Create a Bitcoin transaction (UTXO selection, outputs, etc.)
    // 3. Sign the transaction using sign_with_ecdsa
    // 4. Broadcast it to the Bitcoin network using bitcoin_send_transaction
    
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: ECDSA_KEY_ID.to_string(),
    };
    
    // Get public key for Bitcoin address derivation
    let public_key_arg = EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![],
        key_id: key_id.clone(),
    };
    
    // Attempt to get public key (this is a simplified implementation)
    // In production, you would use this to derive Bitcoin addresses
    match ecdsa_public_key(public_key_arg).await {
        Ok((public_key,)) => {
            ic_cdk::println!("Got public key, length: {}", public_key.public_key.len());
            // In production: Derive Bitcoin address from this public key
        }
        Err((code, msg)) => {
            return Err(VaultError::ThresholdEcdsaError(format!(
                "Failed to get public key: code={:?}, message={}",
                code, msg
            )));
        }
    }
    
    // In a production implementation:
    // 1. Derive Bitcoin address from public key
    // 2. Get UTXOs for the canister's Bitcoin address
    // 3. Build transaction with inputs (UTXOs) and outputs (destination + change)
    // 4. Sign transaction inputs using sign_with_ecdsa
    // 5. Broadcast using bitcoin_send_transaction API
    
    // For now, we log the intended transfer
    // In production, this would actually send the Bitcoin
    ic_cdk::println!(
        "Transferring {} satoshis to Bitcoin address: {}",
        amount_satoshi, destination
    );
    
    // Note: Actual implementation would require:
    // - Bitcoin transaction construction library
    // - UTXO management
    // - Transaction signing with proper hash calculation
    // - Bitcoin network API integration
    
    Ok(())
}

// Get all vaults for the caller
#[query]
fn get_my_vaults() -> Result<Vec<Vault>> {
    let caller = caller();
    
    let vaults: Vec<Vault> = VAULTS.with(|v| {
        v.borrow()
            .iter()
            .filter(|(_, vault)| vault.owner == caller)
            .map(|(_, vault)| vault)
            .collect()
    });
    
    Ok(vaults)
}

// Check if vault is expired
#[query]
fn is_vault_expired() -> Result<bool> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    let now = time();
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    let vault = VAULTS.with(|vaults| {
        vaults.borrow().get(&vault_id).ok_or(VaultError::VaultNotFound)
    })?;
    
    if vault.owner != caller {
        return Err(VaultError::Unauthorized);
    }
    
    let elapsed_nanos = now.saturating_sub(vault.last_check_in);
    let elapsed_days = elapsed_nanos / NANOS_PER_DAY;
    
    Ok(elapsed_days > vault.timeout_days)
}

// Get time until expiration
#[query]
fn get_time_until_expiration() -> Result<u64> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    let now = time();
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    let vault = VAULTS.with(|vaults| {
        vaults.borrow().get(&vault_id).ok_or(VaultError::VaultNotFound)
    })?;
    
    if vault.owner != caller {
        return Err(VaultError::Unauthorized);
    }
    
    let elapsed_nanos = now.saturating_sub(vault.last_check_in);
    let elapsed_days = elapsed_nanos / NANOS_PER_DAY;
    
    if elapsed_days >= vault.timeout_days {
        Ok(0)
    } else {
        let remaining_days = vault.timeout_days - elapsed_days;
        Ok(remaining_days)
    }
}

// vetKeys integration for encrypted notes
// Note: vetKeys is a vetKD (verifiable key derivation) system for encrypted storage
// This is a placeholder implementation - full vetKeys integration would require
// the vetKeys canister interface and proper key derivation

// Store encrypted note using vetKeys (simplified)
#[update]
fn store_encrypted_note(encrypted_note: String) -> Result<()> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    VAULTS.with(|vaults| {
        let mut vaults = vaults.borrow_mut();
        let mut vault = vaults.get(&vault_id).ok_or(VaultError::VaultNotFound)?;
        
        if vault.owner != caller {
            return Err(VaultError::Unauthorized);
        }
        
        // In a real implementation, this would:
        // 1. Use vetKeys to derive encryption keys
        // 2. Encrypt the note with the derived keys
        // 3. Store the encrypted note
        
        vault.encrypted_note = Some(encrypted_note);
        vaults.insert(vault_id, vault);
        Ok(())
    })
}

// Retrieve encrypted note
#[query]
fn get_encrypted_note() -> Result<Option<String>> {
    let caller = caller();
    let caller_bytes = PrincipalBytes::from(caller);
    
    let vault_id = VAULT_INDEX.with(|index| {
        index.borrow().get(&caller_bytes).ok_or(VaultError::VaultNotFound)
    })?;
    
    let vault = VAULTS.with(|vaults| {
        vaults.borrow().get(&vault_id).ok_or(VaultError::VaultNotFound)
    })?;
    
    if vault.owner != caller {
        return Err(VaultError::Unauthorized);
    }
    
    // In a real implementation, this would:
    // 1. Use vetKeys to derive decryption keys
    // 2. Decrypt the note
    // 3. Return the decrypted note (or keep it encrypted for client-side decryption)
    
    Ok(vault.encrypted_note)
}

// Export candid interface
ic_cdk::export_candid!();

