use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};

mod declarations;
use crate::declarations::users_rs::{Result2, Smileyball, ThruToday};
use declarations::users_rs::{users_rs, User};

enum AppData {
    Smileyball(Smileyball),
    ThruToday(ThruToday),
}

#[update(name = "getSelf")]
async fn get_self() -> Result<User, String> {
    let id = ic_cdk::api::caller();
    match users_rs.get_user_by_principal(id).await.unwrap().0 {
        Result2::Ok(user) => Ok(user),
        Result2::Err(e) => Err(e),
    }
}

#[update]
async fn get_user_by_nickname(name: String) -> Result<User, String> {
    match users_rs.get_user_by_nickname(name).await.unwrap().0 {
        Result2::Ok(user) => Ok(user),
        Result2::Err(e) => Err(e),
    }
}

#[update]
async fn update(app_name: AppData) -> String {
    let id = ic_cdk::api::caller();
    match app_name {
        AppData::Smileyball(user) => {}
        AppData::ThruToday(_) => {}
    }
}

// Enable Candid export
ic_cdk::export_candid!();
