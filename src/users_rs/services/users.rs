use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;
use crate::structures::user::{AllowedAppsData, AppTypes, AppsData, User};
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
    static ALLOWED_APPS: HashMap<String, AppTypes> = HashMap::from([
        ("eo3rd-7qaaa-aaaan-qmn6q-cai".to_string(), AppTypes::Smileyball),
        ("11111-7qaaa-aaaan-qmn6q-cai".to_string(), AppTypes::ThruToday),
    ]);
}

fn create_new_user(id: Principal, user: User) -> Result<User, String> {
    USERS_STORE.with(|users_store| {
        ID_STORE.with(|id_store| {
            match validators::check_if_unique_username(user.nickname.as_str(), id) {
                Ok(_) => {
                    users_store.borrow_mut().insert(id, user.clone());
                    id_store.borrow_mut().insert(user.nickname.clone(), id);
                    Ok(user)
                }
                Err(e) => Err(format!("nickname_exists. {}", e)),
            }
        })
    })
}
fn update_app_data_factory(existing_user: &mut User, update_data: User, app_name: AppTypes) {
    match app_name {
        AppTypes::Smileyball => {
            let apps_data = existing_user.apps_data.get_or_insert_with(Default::default);
            let registry = apps_data.registry.get_or_insert_with(Default::default);

            let smileyball_data = match registry.get("Smileyball") {
                Some(AllowedAppsData::Smileyball(data)) => data.clone(),
                _ => Smileyball::default(),
            };

            let mut updated_smileyball_data = smileyball_data;

            if let Some(AllowedAppsData::Smileyball(update_smileyball_data)) = update_data
                .apps_data
                .as_ref()
                .and_then(|apps_data| apps_data.registry.as_ref())
                .and_then(|registry| registry.get("Smileyball"))
            {
                updated_smileyball_data.update_from_user(update_smileyball_data);
            }

            registry.insert(
                "Smileyball".to_string(),
                AllowedAppsData::Smileyball(updated_smileyball_data),
            );
        }
        AppTypes::ThruToday => {
            let apps_data = existing_user.apps_data.get_or_insert_with(Default::default);
            let registry = apps_data.registry.get_or_insert_with(Default::default);

            let thru_today_data = match registry.get("ThruToday") {
                Some(AllowedAppsData::ThruToday(data)) => data.clone(),
                _ => ThruToday::default(),
            };

            let mut updated_thru_today_data = thru_today_data;

            if let Some(AllowedAppsData::ThruToday(update_thru_today_data)) = update_data
                .apps_data
                .as_ref()
                .and_then(|apps_data| apps_data.registry.as_ref())
                .and_then(|registry| registry.get("ThruToday"))
            {
                // Aktualizacja pól ThruToday
                updated_thru_today_data.is_suspended = update_thru_today_data.is_suspended;

                if !update_thru_today_data.live_data.is_empty() {
                    updated_thru_today_data.live_data = update_thru_today_data.live_data.clone();
                }

                if !update_thru_today_data.historical_data.is_empty() {
                    updated_thru_today_data.historical_data =
                        update_thru_today_data.historical_data.clone();
                }
            }

            registry.insert(
                "ThruToday".to_string(),
                AllowedAppsData::ThruToday(updated_thru_today_data),
            );
        }
        _ => {}
    }
}

fn update_user(
    id: Principal,
    app_name: AppTypes,
    app_canister_id: Principal,
    existing_user: User,
    update_data: User,
) -> Result<User, String> {
    USERS_STORE.with(|users_store| {
        ID_STORE.with(|id_store| {
            match validators::check_if_unique_username(update_data.nickname.as_str(), id) {
                Ok(_) => {
                    let mut id_store_borrowed = id_store.borrow_mut();
                    let mut users_store_borrowed = users_store.borrow_mut();

                    id_store_borrowed.remove(&existing_user.nickname);
                    id_store_borrowed.insert(update_data.nickname.clone(), id);

                    let mut updated_user = existing_user.clone();
                    update_app_data_factory(&mut updated_user, update_data.clone(), app_name);
                    users_store_borrowed.insert(id, updated_user.clone());

                    Ok(updated_user)
                }
                Err(e) => Err(format!("nickname_exists. {}", e)),
            }
        })
    })
}

pub fn update_or_create(
    calling_canister: Principal,
    principal: Principal,
    app_canister_id: Principal,
    user: User,
) -> Result<User, String> {
    if !check_if_legal_caller(calling_canister) {
        return Err(String::from("illegal_calling_canister"));
    }

    let app_name = match check_if_legal_app(app_canister_id.to_string()) {
        Some(value) => value,
        None => {
            return Err(String::from("illegal_app"));
        }
    };

    if user.nickname.is_empty() {
        return Err(String::from("you need to provide nickname"));
    }

    match crate::get_user_from_principal(principal) {
        Ok(existing_user) => {
            ic_cdk::println!("User found, updating: {:?}", existing_user);
            update_user(
                principal,
                app_name,
                app_canister_id,
                existing_user.clone(),
                user.clone(),
            )
        }
        Err(e) => {
            if e == "user_not_found" {
                ic_cdk::println!("User not found, creating new user: {:?}", user);

                match create_new_user(principal, user.clone()) {
                    Ok(new_user) => {
                        ic_cdk::println!("New user created successfully: {:?}", new_user);
                        Ok(new_user)
                    }
                    Err(e) => {
                        ic_cdk::println!("Failed to create new user: {}", e);
                        Err(format!("Failed to create new user: {}", e))
                    }
                }
            } else {
                ic_cdk::println!("An error occurred: {}", e);
                Err(format!("An error occurred: {}", e))
            }
        }
    }
}

//todo replace with management canister method
fn check_if_legal_caller(calling_canister: Principal) -> bool {
    ALLOWED_CALLERS.with(|allowed_callers| allowed_callers.keys().any(|&i| i == calling_canister))
}

//todo replace with management canister method
fn check_if_legal_app(app_canister_id: String) -> Option<AppTypes> {
    ALLOWED_APPS.with(|allowed_apps| allowed_apps.get(&app_canister_id).cloned())
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
