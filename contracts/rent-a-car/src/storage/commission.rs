use soroban_sdk::Env;

use crate::storage::types::{error::Error, storage::DataKey};

pub fn read_commission_rate(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::CommissionRate)
        .unwrap_or(0)
}

pub fn write_commission_rate(env: &Env, rate: &i128) {
    env.storage()
        .instance()
        .set(&DataKey::CommissionRate, rate);
}

pub fn read_admin_commission_balance(env: &Env) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::AdminCommissionBalance)
        .unwrap_or(0)
}

pub fn write_admin_commission_balance(env: &Env, amount: &i128) {
    env.storage()
        .persistent()
        .set(&DataKey::AdminCommissionBalance, amount);
}

pub fn increment_admin_commission_balance(env: &Env, amount: i128) -> Result<i128, Error> {
    let current = read_admin_commission_balance(env);
    let new_balance = current.checked_add(amount).ok_or(Error::MathOverflow)?;
    write_admin_commission_balance(env, &new_balance);
    Ok(new_balance)
}

pub fn decrement_admin_commission_balance(env: &Env, amount: i128) -> Result<i128, Error> {
    let current = read_admin_commission_balance(env);
    let new_balance = current.checked_sub(amount).ok_or(Error::UnderFlow)?;
    write_admin_commission_balance(env, &new_balance);
    Ok(new_balance)
}
