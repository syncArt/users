use crate::services::entities::{
    UpdateAppDataInput, UpdateGeneralInfoInput, UpdateOrCreateUserInput,
};
use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;
use crate::structures::user::{AppDataEnum, AppTypeEnum, AppsData, EventEnum, GeneralInfo, User};
use crate::utilities::validators;
use candid::{CandidType, Deserialize, Principal};
use dotenv::dotenv;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::env;

type IdStore = BTreeMap<String, Principal>;
type UsersStore = BTreeMap<Principal, User>;

thread_local! {
    static USERS_STORE: RefCell<UsersStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    //todo: replace it with management canister data. Add smileyball canisters ids.
    static ALLOWED_CALLERS: HashMap<Principal, String> = HashMap::from([
        (Principal::from_text(option_env!("CANISTER_ID_USERS_INTER_RS").expect("Error!")).unwrap(), "CANISTER_ID_USERS_INTER_RS".to_string()),
        (Principal::from_text(option_env!("CANISTER_ID_USERS_RS").expect("Error!")).unwrap(), "CANISTER_ID_USERS_RS".to_string()),
        (Principal::from_text("2vxsx-fae").unwrap(), "default_user".to_string())
    ]);
}

fn validate_input(input: &UpdateOrCreateUserInput) -> Result<(), String> {
    if let Some(ref general_info) = input.general_info {
        if general_info.nickname.is_empty() {
            return Err("You should provide a nickname.".to_string());
        }
    } else {
        return Err("Err. No general info.".to_string());
    }
    Ok(())
}

fn handle_create_user(
    principal: Principal,
    general_info: UpdateGeneralInfoInput,
    app_type: AppTypeEnum,
) -> Result<User, String> {
    match create_new_user(principal, general_info, app_type) {
        Ok(new_user) => {
            ic_cdk::println!("Created user: {:?}", new_user);
            Ok(new_user)
        }
        Err(e) => {
            ic_cdk::println!("There is an error creating a user:: {}", e);
            Err(format!("There is an error creating a user: {}", e))
        }
    }
}

fn handle_update_user(
    principal: Principal,
    input: UpdateOrCreateUserInput,
    existing_user: &mut User,
) -> Result<(), String> {
    if let Some(general_info) = input.general_info.clone() {
        update_general_info(principal, general_info)?;
    }
    if let Some(app_data) = input.apps_data.clone() {
        update_app_data(principal, app_data, input.app_type.clone())?;
    }
    Ok(())
}

pub fn update_or_create(
    calling_canister: Principal,
    principal: Principal,
    input: UpdateOrCreateUserInput,
) -> Result<User, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err("You're not allowed to call this method.".to_string());
    }

    validate_input(&input)?;

    if let Ok(mut existing_user) = crate::get_user_from_principal(principal) {
        ic_cdk::println!("Found user and updating: {:?}", existing_user);
        handle_update_user(principal, input, &mut existing_user)?;
        Ok(existing_user)
    } else {
        ic_cdk::println!("Couldnt find a user, creating new one");
        let general_info = input.general_info.clone().unwrap();
        handle_create_user(principal, general_info, input.app_type.clone())
    }
}

fn create_new_user(
    id: Principal,
    general_info: UpdateGeneralInfoInput,
    app_type: AppTypeEnum,
) -> Result<User, String> {
    let mut new_user = User::new();

    new_user.create_account(
        general_info.nickname.clone(),
        general_info.description.clone(),
        app_type.clone(),
    );

    validators::check_if_unique_username(general_info.nickname.as_str(), id)?;

    USERS_STORE.with(|users_store| {
        users_store.borrow_mut().insert(id, new_user.clone());
    });
    ID_STORE.with(|id_store| {
        id_store.borrow_mut().insert(general_info.nickname.clone(), id);
    });

    Ok(new_user)
}
fn update_app_data(
    principal: Principal,
    update_data: AppDataEnum,
    app_type: AppTypeEnum,
) -> Result<User, String> {
    let mut existing_user = crate::get_user_from_principal(principal)?;

    existing_user.update_app_data(update_data, app_type);

    USERS_STORE.with(|users_store| {
        users_store.borrow_mut().insert(principal, existing_user.clone());
    });

    Ok(existing_user)
}
fn update_general_info(
    principal: Principal,
    update_data: UpdateGeneralInfoInput,
) -> Result<User, String> {
    let mut existing_user = crate::get_user_from_principal(principal)?;

    validators::check_if_unique_username(update_data.nickname.as_str(), principal)?;

    USERS_STORE.with(|users_store| {
        ID_STORE.with(|id_store| {
            let mut id_store_borrowed = id_store.borrow_mut();
            let mut users_store_borrowed = users_store.borrow_mut();

            id_store_borrowed.remove(&existing_user.nickname);
            id_store_borrowed.insert(update_data.nickname.clone(), principal);

            existing_user.update_general_info(
                update_data.nickname.clone(),
                update_data.description.clone(),
            );

            users_store_borrowed.insert(principal, existing_user.clone());
        })
    });

    Ok(existing_user)
}

//todo replace with management canister method
fn check_if_legal_caller(calling_canister: Principal) -> bool {
    ALLOWED_CALLERS.with(|allowed_callers| allowed_callers.keys().any(|&i| i == calling_canister))
}

pub fn get_user_from_principal(calling_canister: Principal, id: Principal) -> Result<User, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }

    USERS_STORE.with(|users_store| match users_store.borrow().get(&id).cloned() {
        Some(user) => Ok(user),
        None => Err(String::from("user_not_found")),
    })
}

pub fn get_user_from_nickname(
    calling_canister: Principal,
    nickname: String,
) -> Result<User, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }

    ID_STORE.with(|id_store| {
        USERS_STORE.with(|users_store| {
            match id_store
                .borrow()
                .get(&nickname)
                .and_then(|id| users_store.borrow().get(id).cloned())
            {
                Some(user) => Ok(user),
                None => Err(String::from("user_not_found")),
            }
        })
    })
}
pub fn get_app_data_from_user(
    calling_canister: Principal,
    principal: Principal,
    app_type: AppTypeEnum,
) -> Result<Option<AppDataEnum>, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }

    USERS_STORE.with(|users_store| {
        match users_store.borrow().get(&principal).cloned() {
            Some(user) => {
                let app_key = format!("{:?}", app_type);
                Ok(user.apps_data.registry.get(&app_key).cloned())
            }
            None => Err(String::from("user_not_found")),
        }
    })
}


pub fn get_principal_from_nickname(
    calling_canister: Principal,
    nickname: String,
) -> Result<Option<Principal>, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }

    ID_STORE.with(|user_store| {
        let mut store = user_store.borrow_mut();
        Ok(store.get(&nickname).cloned())
    })
}

pub fn get_all_users(calling_canister: Principal) -> Result<Vec<User>, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }
    ID_STORE.with(|user_store| {
        let mut store = user_store.borrow();
        let x = store.values().cloned();
        ic_cdk::println!("{:?}", x);
    });

    USERS_STORE.with(|user_store| {
        let mut store = user_store.borrow();
        Ok(store.values().cloned().collect())
    })
}

pub fn get_general_info_from_user(
    calling_canister: Principal,
    principal: Principal,
) -> Result<GeneralInfo, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }

    USERS_STORE.with(|users_store| {
        match users_store.borrow().get(&principal).cloned() {
            Some(user) => Ok(GeneralInfo {
                nickname: user.nickname.clone(),
                description: user.description.clone(),
            }),
            None => Err(String::from("user_not_found")),
        }
    })
}

pub fn remove(calling_canister: Principal, id: Principal) -> Result<String, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_caller"));
    }

    USERS_STORE.with(|user_store| {
        let mut users = user_store.borrow_mut();
        if users.remove(&id).is_some() {
            ID_STORE.with(|id_store| {
                let mut ids = id_store.borrow_mut();
                let id_key = id.to_text();
                ic_cdk::println!("Trying to remove ID: {}", id_key);
                ic_cdk::println!("IDS before removal: {:?}", *ids);
                if ids.remove(&id_key).is_some() {
                    Ok(String::from("User removed!"))
                } else {
                    ic_cdk::println!("Failed to remove ID: {}", id_key);

                    Err(String::from(
                        "Cannot remove user ID, that ID doesn't exist.",
                    ))
                }
            })
        } else {
            ic_cdk::println!("Failed to remove user with ID: {:?}", id);
            Err(String::from("Cannot remove user, that user doesn't exist."))
        }
    })
}
