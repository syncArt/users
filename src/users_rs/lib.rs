use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{query, update};

mod services;
mod structures;
mod utilities;

use crate::services::users;
use crate::structures::user::User;

#[query]
fn get_user_from_principal(id: Principal) -> Result<User, String> {
    users::get_user_from_principal(id)
}

#[query]
fn get_user_from_nickname(nickname: String) -> Result<User, String> {
    users::get_user_from_nickname(nickname)
}

#[query]
fn get_principal_from_nickname(nickname: String) -> Result<Option<Principal>, String> {
    users::get_principal_from_nickname(nickname)
}

#[query]
fn get_all_users() -> Result<Vec<User>, String> {
    users::get_all_users()
}

#[update]
fn update(id: Principal, user: User) -> Result<User, String> {
    users::update(id, user)
}

#[update]
fn remove(id: Principal) -> Result<String, String> {
    users::remove(id)
}

// Enable Candid export
ic_cdk::export_candid!();
