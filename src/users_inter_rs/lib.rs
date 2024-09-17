use candid::{CandidType, Deserialize};
use ic_cdk::update;

mod declarations;
use crate::declarations::users_rs::{
    GeneralInfo, Result1, Result2, Result4, Smileyball, ThruToday,
};
use declarations::users_rs::{users_rs, AppDataEnum, AppTypeEnum, UpdateOrCreateUserInput, User};

enum AppData {
    Smileyball(Smileyball),
    ThruToday(ThruToday),
}

#[update(name = "getSelf")]
async fn get_self() -> Result<User, String> {
    let id = ic_cdk::api::caller();

    match users_rs.get_user_from_principal(id).await {
        Ok((result3,)) => match result3 {
            Result4::Ok(user) => Ok(user),
            Result4::Err(e) => Err(e),
        },
        Err((_, e)) => Err(format!("Error fetching user: {}", e)),
    }
}

#[update]
async fn get_user_by_nickname(name: String) -> Result<User, String> {
    match users_rs.get_user_from_nickname(name).await.unwrap().0 {
        Result4::Ok(user) => Ok(user),
        Result4::Err(e) => Err(e),
    }
}

#[update]
async fn get_general_info_from_user() -> Result<GeneralInfo, String> {
    let principal = ic_cdk::api::caller();

    match users_rs.get_general_info_from_user(principal).await {
        Ok((Result2::Ok(user),)) => Ok(user),
        Ok((Result2::Err(e),)) => Err(e),
        Err((_, e)) => Err("Unexpected error occurred.".to_string()),
    }
}

#[update]
async fn get_app_data_from_user(
    app_type: AppTypeEnum,
) -> Result<Option<AppDataEnum>, String> {
    let principal = ic_cdk::api::caller();

    match users_rs.get_app_data_from_user(principal, app_type).await {
        Ok((Result1::Ok(user),)) => Ok(user),
        Ok((Result1::Err(e),)) => Err(e),
        Err((_, e)) => Err(format!("Failed to fetch app data: {:?}", e)),
    }
}

#[update]
async fn update(user: UpdateOrCreateUserInput) -> Result<User, String> {
    let principal = ic_cdk::api::caller();

    match user.app_type {
        AppTypeEnum::Smileyball => match users_rs.update(principal, user).await {
            Ok((Result4::Ok(updated_user),)) => Ok(updated_user),
            Ok((Result4::Err(e),)) => Err(format!("Failed to update user for Smileyball: {}", e)),
            Err((_, e)) => Err(format!("Update failed due to rejection: {}", e)),
        },
        AppTypeEnum::ThruToday => match users_rs.update(principal, user).await {
            Ok((Result4::Ok(updated_user),)) => Ok(updated_user),
            Ok((Result4::Err(e),)) => Err(format!("Failed to update user for ThruToday: {}", e)),
            Err((_, e)) => Err(format!("Update failed due to rejection: {}", e)),
        },
        AppTypeEnum::General => match users_rs.update(principal, user).await {
            Ok((Result4::Ok(updated_user),)) => Ok(updated_user),
            Ok((Result4::Err(e),)) => Err(format!("Failed to update user for General: {}", e)),
            Err((_, e)) => Err(format!("Update failed due to rejection: {}", e)),
        },
        _ => Err("Unexpected error occurred.".to_string()),
    }
}

// Enable Candid export
ic_cdk::export_candid!();
