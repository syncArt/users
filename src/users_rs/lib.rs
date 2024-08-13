use candid::{CandidType, Deserialize, Principal};
use dotenv::dotenv;
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;

mod queries;
mod structures;
mod updates;
mod utilities;

use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;
use crate::structures::user::User;

type IdStore = BTreeMap<String, Principal>;
type UsersStore = BTreeMap<Principal, User>;

thread_local! {
    static USERS_STORE: RefCell<UsersStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static ALLOWED_CALLERS: Vec<Principal> = vec![
        Principal::from_text(option_env!("CANISTER_ID_USERS_INTER_RS").expect("Error!")).unwrap(),
        Principal::from_text(option_env!("CANISTER_ID_USERS_RS").expect("Error!")).unwrap(),
        Principal::from_text("2vxsx-fae").unwrap()
    ];
}

fn check_if_unique_username(nickname: &str, id: Principal) -> Result<(), String> {
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

fn check_if_legal_caller() -> bool {
    let caller = ic_cdk::api::caller();
    ALLOWED_CALLERS.with(|allowed_callers| allowed_callers.iter().any(|&i| i == caller))
}

#[query]
fn get_user_from_principal(id: Principal) -> Result<User, String> {
    USERS_STORE.with(|users_store| match users_store.borrow().get(&id).cloned() {
        Some(user) => Ok(user),
        None => Err(String::from("user_not_found")),
    })
}

#[query]
fn get_user_from_nickname(name: String) -> Result<User, String> {
    if !check_if_legal_caller() {
        return Err(String::from("illegal_caller"));
    }

    ID_STORE.with(|id_store| {
        USERS_STORE.with(|users_store| {
            match id_store
                .borrow()
                .get(&name)
                .and_then(|id| users_store.borrow().get(id).cloned())
            {
                Some(user) => Ok(user),
                None => Err(String::from("user_not_found")),
            }
        })
    })
}

fn create_new_user(id: Principal, user: User) -> Result<User, String> {
    USERS_STORE.with(|users_store| {
        ID_STORE.with(
            |id_store| match check_if_unique_username(user.nickname.as_str(), id) {
                Ok(_) => {
                    users_store.borrow_mut().insert(id, user.clone());
                    id_store.borrow_mut().insert(user.nickname.clone(), id);
                    Ok(user)
                }
                Err(e) => Err(format!("nickname_exists. {}", e)),
            },
        )
    })
}
fn update_user(id: Principal, existing_user: User, user: User) -> Result<User, String> {
    USERS_STORE.with(|users_store| {
        ID_STORE.with(
            |id_store| match check_if_unique_username(user.nickname.as_str(), id) {
                Ok(_) => {
                    let mut id_store_borrowed = id_store.borrow_mut();
                    let mut users_store_borrowed = users_store.borrow_mut();

                    id_store_borrowed.remove(&existing_user.nickname);
                    id_store_borrowed.insert(user.nickname.clone(), id);

                    users_store_borrowed.insert(id, user.clone());

                    Ok(user.clone())
                }
                Err(e) => Err(format!("nickname_exists. {}", e)),
            },
        )
    })
}

#[update]
fn update(id: Principal, user: User) -> Result<User, String> {
    if !check_if_legal_caller() {
        return Err(String::from("illegal_caller"));
    }
    if user.nickname.is_empty() {
        return Err(String::from("you need to provide nickname"));
    }

    match get_user_from_principal(id) {
        Ok(existing_user) => {
            ic_cdk::println!("User found, updating: {:?}", existing_user);
            update_user(id, existing_user.clone(), user.clone())
        }
        Err(e) => {
            if e == "user_not_found" {
                ic_cdk::println!("User not found, creating new user: {:?}", user);

                match create_new_user(id, user.clone()) {
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

#[update]
fn remove(id: Principal) -> Result<String, String> {
    if !check_if_legal_caller() {
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

#[query]
fn get_principal_from_nickname(nickname: String) -> Result<Option<Principal>, String> {
    if !check_if_legal_caller() {
        return Err(String::from("illegal_caller"));
    }

    ID_STORE.with(|user_store| {
        let mut store = user_store.borrow_mut();
        Ok(store.get(&nickname).cloned())
    })
}

#[query]
fn get_all_users() -> Result<Vec<User>, String> {
    if !check_if_legal_caller() {
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

// Enable Candid export
ic_cdk::export_candid!();
