#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, String
};

#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub id: u64,
    pub owner: Address,
    pub metadata: String,
}

#[contracttype]
pub enum DataKey {
    Property(u64),
}

#[contract]
pub struct RealEstateContract;

#[contractimpl]
impl RealEstateContract {

    pub fn register_property(env: Env, id: u64, owner: Address, metadata: String) {
        owner.require_auth();

        let key = DataKey::Property(id);

        if env.storage().instance().has(&key) {
            panic!("Property already exists");
        }

        let property = Property { id, owner, metadata };
        env.storage().instance().set(&key, &property);
    }

    pub fn get_property(env: Env, id: u64) -> Property {
        let key = DataKey::Property(id);

        env.storage()
            .instance()
            .get(&key)
            .unwrap()
    }

    pub fn transfer_property(env: Env, id: u64, new_owner: Address) {
        let key = DataKey::Property(id);

        let mut property: Property = env.storage()
            .instance()
            .get(&key)
            .unwrap();

        property.owner.require_auth();

        property.owner = new_owner;

        env.storage().instance().set(&key, &property);
    }
}