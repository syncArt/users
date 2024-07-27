use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{ query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;
use dotenv::dotenv;
use std::env;

type IdStore = BTreeMap<String, Principal>;
type UsersStore = BTreeMap<Principal, User>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Mood {
    primary_emoji: String,
    secondary_emoji: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Moodmoji {
    mood: Mood,
    background: String
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct LiveVoting {
    pub song_id: String,
    pub moodmoji: Moodmoji,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct VoteRecord {
    pub song_id: String,
    pub moodmoji: Moodmoji,
    pub added_at: String
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct User {
    pub nickname: String,
    pub moodmojis: Moodmoji,
    pub live_voting: Vec<LiveVoting>,
    pub historical_data: Vec<VoteRecord>
}

thread_local! {
    static USERS_STORE: RefCell<UsersStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
}

fn check_if_legal_caller () -> bool {
    let caller = ic_cdk::api::caller().to_text();

    dotenv().ok();
    let gateway_principal = option_env!("CANISTER_ID_USERS_INTER_RS").expect("Error!");

    return caller == gateway_principal
}

fn return_empty_user () -> User {
    return User {
        moodmojis: Moodmoji {
            mood: Mood {
                secondary_emoji: String::from(""),
                primary_emoji: String::from("")
            },
            background: String::from("")
        },
        live_voting: vec! [],
        historical_data: vec! [],
        nickname: String::from("")
    };
}

#[query(name = "getSelf")]
fn get_self(id: Principal) -> User {
    let is_legal_caller = check_if_legal_caller();

    if is_legal_caller {
        USERS_STORE.with(|users_store| {
            users_store.borrow().get(&id).cloned().unwrap_or_default()
        })

    } else {
        let empty_user = return_empty_user();
        empty_user
    }
}

#[query]
fn get(name: String) -> User {
    let is_legal_caller = check_if_legal_caller();

    if is_legal_caller {
        ID_STORE.with(|id_store| {
            USERS_STORE.with(|users_store| {
                id_store
                    .borrow()
                    .get(&name)
                    .and_then(|id| users_store.borrow().get(id).cloned()).unwrap_or_default()
            })
        })
    } else {
        let empty_user = return_empty_user();
        empty_user
    }
}

#[update]
fn update(id: Principal, user: User) -> String {
    let is_legal_caller = check_if_legal_caller();

    if is_legal_caller {
        ID_STORE.with(|id_store| {
            id_store
                .borrow_mut()
                .insert(user.nickname.clone(), id);
        });

        USERS_STORE.with(|users_store| {
            users_store.borrow_mut().insert(id, user);
        });
        return String::from("Updated!");
    } else {
        return String::from("Not allowed");
    }
}

// Enable Candid export
ic_cdk::export_candid!();