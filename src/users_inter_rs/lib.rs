use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};

mod declarations;
use crate::declarations::users_rs::{Result3, Smileyball, ThruToday};
use declarations::users_rs::{users_rs, User, AppTypeEnum, AppDataEnum, UpdateGeneralInfoInput, UpdateOrCreateUserInput};

enum AppData {
    Smileyball(Smileyball),
    ThruToday(ThruToday),
}


#[update(name = "getSelf")]
async fn get_self() -> Result<User, String> {
    let id = ic_cdk::api::caller();

    match users_rs.get_user_from_principal(id).await {
        Ok((result3,)) => match result3 {
            Result3::Ok(user) => Ok(user),
            Result3::Err(e) => Err(e),
        },
        Err((_, e)) => Err(format!("Error fetching user: {}", e)),
    }
}



#[update]
async fn get_user_by_nickname(name: String) -> Result<User, String> {
    match users_rs.get_user_from_nickname(name).await.unwrap().0 {
        Result3::Ok(user) => Ok(user),
        Result3::Err(e) => Err(e),
    }
}

#[update]
async fn update(user: UpdateOrCreateUserInput) -> Result<User, String> {
    let principal = ic_cdk::api::caller();

    match user.app_type {
        AppTypeEnum::Smileyball => {
            match users_rs.update(principal, user).await {
                Ok((Result3::Ok(updated_user),)) => {
                    Ok(updated_user)
                },
                Ok((Result3::Err(e),)) => Err(format!("Failed to update user for Smileyball: {}", e)),
                Err((_, e)) => Err(format!("Update failed due to rejection: {}", e)),
            }
        }
        AppTypeEnum::ThruToday => {
            match users_rs.update(principal, user).await {
                Ok((Result3::Ok(updated_user),)) => {
                    Ok(updated_user)
                },
                Ok((Result3::Err(e),)) => Err(format!("Failed to update user for ThruToday: {}", e)),
                Err((_, e)) => Err(format!("Update failed due to rejection: {}", e)),
            }
        }
    }
}

// Enable Candid export
ic_cdk::export_candid!();
