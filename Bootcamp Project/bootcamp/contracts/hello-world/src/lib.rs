#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Vec, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct Artwork {
    pub id: u64,
    pub creator: Address,
    pub title: String,
    pub description: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum GalleryKey {
    Artwork(u64),
    ArtworkCount,
}

#[contract]
pub struct ArtGalleryContract;

#[contractimpl]
impl ArtGalleryContract {
    pub fn add_artwork(env: Env, creator: Address, title: String, description: String) -> u64 {
        creator.require_auth();

        let mut count: u64 = env.storage().instance().get(&GalleryKey::ArtworkCount).unwrap_or(0);
        count += 1;

        let artwork = Artwork {
            id: count,
            creator,
            title,
            description,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&GalleryKey::Artwork(count), &artwork);
        env.storage().instance().set(&GalleryKey::ArtworkCount, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Artwork {} added to gallery", count);
        count
    }

    pub fn get_artwork(env: Env, id: u64) -> Artwork {
        env.storage().instance().get(&GalleryKey::Artwork(id)).expect("Artwork not found")
    }

    pub fn get_artwork_count(env: Env) -> u64 {
        env.storage().instance().get(&GalleryKey::ArtworkCount).unwrap_or(0)
    }
}
