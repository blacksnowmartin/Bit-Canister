use ic_cdk::api::time;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Vault {
    owner: String,
    backup_wallet: String,
    last_check_in: u64,
    timeout_days: u64,
    balance_satoshi: u64,
    encrypted_note: Option<String>,
}

thread_local! {
    static VAULTS: std::cell::RefCell<HashMap<String, Vault>> = Default::default();
}

#[update]
fn create_vault(owner: String, backup_wallet: String, timeout_days: u64, encrypted_note: Option<String>) {
    let vault = Vault {
        owner: owner.clone(),
        backup_wallet,
        timeout_days,
        encrypted_note,
        last_check_in: time(),
        balance_satoshi: 0,
    };
    VAULTS.with(|v| v.borrow_mut().insert(owner, vault));
}

#[update]
fn deposit(owner: String, amount: u64) {
    VAULTS.with(|v| {
        if let Some(vault) = v.borrow_mut().get_mut(&owner) {
            vault.balance_satoshi += amount;
            vault.last_check_in = time();
        }
    });
}

#[update]
fn check_in(owner: String) {
    VAULTS.with(|v| {
        if let Some(vault) = v.borrow_mut().get_mut(&owner) {
            vault.last_check_in = time();
        }
    });
}

#[query]
fn get_vault(owner: String) -> Option<Vault> {
    VAULTS.with(|v| v.borrow().get(&owner).cloned())
}

// Periodic check (simulate cron)
#[update]
fn trigger_expired_transfers() {
    let now = time();
    VAULTS.with(|v| {
        for (_, vault) in v.borrow_mut().iter_mut() {
            let elapsed_days = (now - vault.last_check_in) / 86_400_000_000;
            if elapsed_days > vault.timeout_days {
                ic_cdk::println!(
                    "Triggering BTC transfer to backup wallet: {}",
                    vault.backup_wallet
                );
                // Here you’d call ckBTC ledger or Threshold ECDSA signing
            }
        }
    });
}
