use ic_cdk::api::time; // Importing time API from ic_cdk
use ic_cdk_macros::*; // Importing macros for defining update and query functions
use serde::{Deserialize, Serialize}; // Importing serialization/deserialization traits
use std::collections::HashMap; // Importing HashMap for storing vaults

// Defining the Vault struct with necessary fields
#[derive(Serialize, Deserialize, Clone)]
struct Vault {
    owner: String, // Owner of the vault
    backup_wallet: String, // Backup wallet address
    last_check_in: u64, // Timestamp of the last check-in
    timeout_days: u64, // Timeout duration in days
    balance_satoshi: u64, // Balance in satoshis
    encrypted_note: Option<String>, // Optional encrypted note
}

// Thread-local storage for vaults
thread_local! {
    static VAULTS: std::cell::RefCell<HashMap<String, Vault>> = Default::default(); // HashMap to store vaults
}

// Function to create a new vault
#[update]
fn create_vault(owner: String, backup_wallet: String, timeout_days: u64, encrypted_note: Option<String>) {
    let vault = Vault {
        owner: owner.clone(), // Cloning owner string
        backup_wallet, // Setting backup wallet
        timeout_days, // Setting timeout days
        encrypted_note, // Setting optional encrypted note
        last_check_in: time(), // Setting current time as last check-in
        balance_satoshi: 0, // Initializing balance to zero
    };
    // Inserting the new vault into the thread-local storage
    VAULTS.with(|v| v.borrow_mut().insert(owner, vault));
}

// Function to deposit funds into a vault
#[update]
fn deposit(owner: String, amount: u64) {
    VAULTS.with(|v| {
        // Attempting to get mutable reference to the vault
        if let Some(vault) = v.borrow_mut().get_mut(&owner) {
            vault.balance_satoshi += amount; // Adding amount to balance
            vault.last_check_in = time(); // Updating last check-in time
        }
    });
}

// Function to check in to a vault
#[update]
fn check_in(owner: String) {
    VAULTS.with(|v| {
        // Attempting to get mutable reference to the vault
        if let Some(vault) = v.borrow_mut().get_mut(&owner) {
            vault.last_check_in = time(); // Updating last check-in time
        }
    });
}

// Function to retrieve vault information
#[query]
fn get_vault(owner: String) -> Option<Vault> {
    VAULTS.with(|v| v.borrow().get(&owner).cloned()) // Returning a clone of the vault if it exists
}

// Function to trigger actions for expired transfers
#[update]
fn trigger_expired_transfers() {
    let now = time(); // Getting the current time
    VAULTS.with(|v| {
        // Iterating through all vaults
        for (_, vault) in v.borrow_mut().iter_mut() {
            let elapsed_days = (now - vault.last_check_in) / 86_400_000_000; // Calculating elapsed days
            if elapsed_days > vault.timeout_days { // Checking if timeout has occurred
                ic_cdk::println!(
                    "Triggering BTC transfer to backup wallet: {}",
                    vault.backup_wallet // Logging the backup wallet for transfer
                );
                // Here you’d call ckBTC ledger or Threshold ECDSA signing
            }
        }
    });
}
