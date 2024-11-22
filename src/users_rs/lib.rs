use crate::structures::user::{AppDataEnum, AppTypeEnum, GeneralInfo};
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{query, update};

mod services;
mod structures;
mod utilities;

use crate::services::users;
use crate::structures::user::User;

#[query]
fn whoami() -> Principal {
    ic_cdk::api::caller()
}

#[query]
fn get_user_from_principal(id: Principal) -> Result<User, String> {
    let calling_canister = ic_cdk::api::caller();
    users::get_user_from_principal(calling_canister, id)
}

#[query]
fn get_user_from_nickname(nickname: String) -> Result<User, String> {
    let calling_canister = ic_cdk::api::caller();

    users::get_user_from_nickname(calling_canister, nickname)
}

#[query]
fn get_app_data_from_user(
    principal: Principal,
    app_type: AppTypeEnum,
) -> Result<Option<AppDataEnum>, String> {
    let calling_canister = ic_cdk::api::caller();
    users::get_app_data_from_user(calling_canister, principal, app_type)
}

#[query]
fn get_general_info_from_user(principal: Principal) -> Result<GeneralInfo, String> {
    let calling_canister = ic_cdk::api::caller();
    users::get_general_info_from_user(calling_canister, principal)
}

#[query]
fn get_principal_from_nickname(nickname: String) -> Result<Option<Principal>, String> {
    let calling_canister = ic_cdk::api::caller();
    users::get_principal_from_nickname(calling_canister, nickname)
}

#[query]
fn get_all_users() -> Result<Vec<User>, String> {
    let calling_canister = ic_cdk::api::caller();
    users::get_all_users(calling_canister)
}

#[update]
async fn update(
    principal: Principal,
    input: crate::services::entities::UpdateOrCreateUserInput,
) -> Result<User, String> {
    let calling_canister = ic_cdk::api::caller();
    users::update_or_create(calling_canister, principal, input).await
}

#[update]
fn remove(id: Principal) -> Result<String, String> {
    let calling_canister = ic_cdk::api::caller();
    users::remove(calling_canister, id)
}

// Enable Candid export
ic_cdk::export_candid!();
