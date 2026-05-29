use std::collections::HashMap;

// Name Assignment (variables and constants)
// TODO: Assign the current bitcoin mining reward
pub const MINING_REWARD: f64 = 0.0;
// TODO: Assign the current block height
pub const CURRENT_BLOCK_HEIGHT: u64 = 0;
// TODO: Assign the number of satoshis in one Bitcoin
pub const BTC_TO_SATS: u64 = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    //casting the type of u64 to f64 using as key word
    let result = blocks_mined as f64 * MINING_REWARD;
    return result;
} //blocks_mined as f64 * MINING_REWARD could also return a value which is f64. without a full colon it will be an expression returning a value

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    fee >= 0.00001 && fee <= 0.01
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    balance > 50.0
    //since its a float the decimals are key
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    // TODO: Calculate fee rate (fee_btc / size_bytes) and use if/else if/else
    // High: > 0.00005, Medium: > 0.00001, otherwise Low
    let fee_rate = fee_btc / size_bytes as f64;
    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
    //s = "mainnet" -> borrowed string slice (&str).
    network.to_lowercase == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    //for value in 100..=200 true -> loops don’t return values
    value >= 100 && value <= 200
} //(100..=200).contains(&value) because the contains function expects a pointer

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    std::ptr::eq(wallet1, wallet2) //two raw pointers point to the exact same memory address.
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(address: &str) -> String {
    address.trim().to_lowercase()
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(mut utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    //adding mut to be able to modify
    utxos.push(new_utxo);
    utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    fee_list
        .iter()
        .enumerate()
        .find(|(_, fee)| **fee > 0.005)
        .map(|(i, fee)| (i, *fee))
} //**
  //enumerate() gives you (index, value). The double **fee dereferences through the iterator and the slice. .map() converts the found reference into an owned value.

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    ("Louis_wallet".to_string(), 12)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    tx_pool
        .get(txid)
        .cloned()
        .unwrap_or_else(|| "not found".to_string())
} //**

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    // TODO: Destructure the tuple into (name, balance) and format the result
    // Expected format: "Wallet <name> has balance: <balance> BTC"
    let (name, balance) = wallet_info;
    format!("Wallet {} has balance: {} BTC", name, balance as u64)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    // TODO: Multiply btc by BTC_TO_SATS and return as u64
    (btc * BTC_TO_SATS as f64) as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    // TODO: Build a random suffix of (32 - prefix.len()) chars from [a-z0-9]
    // TODO: Concatenate prefix + suffix and return
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let suffix_len = 32 - prefix.len();
    let suffix: String = (0..suffix_len)
        .map(|i| charset[i % charset.len()])
        .collect();
    format!("{}{}", prefix, suffix)
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
    // TODO: Check that height is not negative
    // TODO: Check that height is within a realistic range (<= 800_000)
    // TODO: Return (true, "Valid block height") otherwise
    if height < 0 {
        return (false, "Block height cannot be negative".to_string());
    }
    if height > 800_000 {
        return (false, "Block height is unrealistic".to_string());
    }
    (true, "Valid block height".to_string())
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    // TODO: Base reward is 50 * 100_000_000 sats; halving interval is 210_000 blocks
    // TODO: For each block: halvings = block / 210_000; reward = base >> halvings
    // TODO: Insert (block, reward) into the result HashMap
    let base_reward: u64 = 50 * 100_000_000;
    let halving_interval: u64 = 210_000;
    let mut result = HashMap::new();
    for &block in blocks {
        let halvings = block / halving_interval;
        let reward = base_reward >> halvings;
        result.insert(block, reward);
    }
    result
} //shifting by 1 halves the value, so >> halvings applies all halvings at once.

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    // TODO: Filter UTXOs to those with value >= target
    // TODO: Return the one with the smallest value, or None if none qualify
    utxos
        .iter()
        .filter(|u| u.value >= target)
        .min_by_key(|u| u.value)
        .cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    // TODO: Build a base map with "txid" and "vout" (as string)
    // TODO: Merge extra into the base map and return
    let mut map = HashMap::new();
    map.insert("txid".to_string(), txid.to_string());
    map.insert("vout".to_string(), vout.to_string());
    for (k, v) in extra {
        map.insert(k, v);
    }
    map
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    if bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }
    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)
}
