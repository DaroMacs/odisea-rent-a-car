use soroban_sdk::{Env, Symbol};

pub(crate) fn commission_set(env: &Env, commission_rate: i128) {
    let topics = (Symbol::new(env, "commission_set"),);

    env.events().publish(
        topics,
        commission_rate,
    );
}

pub(crate) fn commission_collected(env: &Env, rental_amount: i128, commission: i128) {
    let topics = (Symbol::new(env, "commission_collected"),);

    env.events().publish(
        topics,
        (rental_amount, commission),
    );
}

pub(crate) fn commission_withdrawn(env: &Env, amount: i128) {
    let topics = (Symbol::new(env, "commission_withdrawn"),);

    env.events().publish(
        topics,
        amount,
    );
}
