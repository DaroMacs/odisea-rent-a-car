use soroban_sdk::{Env, Address};
use crate::storage::car::{read_car};
use crate::storage::types::error::Error;
use crate::storage::types::car_status::CarStatus;

pub fn get_car_status(env: &Env, owner: &Address) -> Result<CarStatus, Error> {
          let car = read_car(env, owner)?;

        Ok(car.car_status)
    }
