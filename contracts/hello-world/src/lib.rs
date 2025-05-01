#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, log};

#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub id: u64,
    pub owner: Address,
    pub location: String,
    pub size_sqft: u32,
    pub tokenized: bool,
}

#[contracttype]
pub enum PropertyKey {
    Property(u64),
    PropertyCount,
}

#[contract]
pub struct RealEstateContract;

#[contractimpl]
impl RealEstateContract {
    // Register a new property on-chain
    pub fn register_property(env: Env, owner: Address, location: String, size_sqft: u32) -> u64 {
        owner.require_auth();

        let mut count: u64 = env.storage().instance().get(&PropertyKey::PropertyCount).unwrap_or(0);
        count += 1;

        let property = Property {
            id: count,
            owner: owner.clone(),
            location,
            size_sqft,
            tokenized: false,
        };

        env.storage().instance().set(&PropertyKey::Property(count), &property);
        env.storage().instance().set(&PropertyKey::PropertyCount, &count);

        log!(&env, "Registered property {} by {}", count, owner);
        count
    }

    // Tokenize a property
    pub fn tokenize_property(env: Env, owner: Address, property_id: u64) -> bool {
        owner.require_auth();

        let mut property: Property = env.storage().instance().get(&PropertyKey::Property(property_id)).expect("Property not found");

        if property.owner != owner {
            log!(&env, "Only the owner can tokenize");
            return false;
        }

        property.tokenized = true;
        env.storage().instance().set(&PropertyKey::Property(property_id), &property);

        log!(&env, "Property {} tokenized", property_id);
        true
    }

    // Transfer property ownership
    pub fn transfer_property(env: Env, current_owner: Address, new_owner: Address, property_id: u64) -> bool {
        current_owner.require_auth();

        let mut property: Property = env.storage().instance().get(&PropertyKey::Property(property_id)).expect("Property not found");

        if property.owner != current_owner {
            return false;
        }

        property.owner = new_owner.clone();
        env.storage().instance().set(&PropertyKey::Property(property_id), &property);

        log!(&env, "Property {} transferred to {}", property_id, new_owner);
        true
    }

    // View property details
    pub fn get_property(env: Env, property_id: u64) -> Property {
        env.storage().instance().get(&PropertyKey::Property(property_id)).expect("Property not found")
    }
}
