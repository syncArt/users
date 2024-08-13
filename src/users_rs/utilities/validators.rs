use crate::{get_principal_from_nickname, get_user_from_nickname};
use candid::Principal;

pub fn check_if_unique_username(nickname: &str, id: Principal) -> Result<(), String> {
    let principal_result = get_principal_from_nickname(nickname.to_string());

    let nickname_owner = match principal_result {
        Ok(Some(owner)) => owner,
        _ => return Ok(()),
    };

    match get_user_from_nickname(nickname.to_string()) {
        Ok(_) => {
            if nickname_owner == id {
                Ok(())
            } else {
                Err(String::from("nickname_already_exists"))
            }
        }
        Err(e) => {
            if e == "user_not_found" {
                Ok(())
            } else {
                Err(format!("An error occurred: {}", e))
            }
        }
    }
}
