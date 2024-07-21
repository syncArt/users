use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::ManualReply, query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type UsersStore = BTreeMap<Principal, User>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct SocialMedia {
    pub name: String,
    pub nickname: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct User {
    pub nickname: String,
    pub description: String,
    pub social_media: Vec<SocialMedia>,
}

thread_local! {
    static USERS_STORE: RefCell<UsersStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
}

#[query(name = "getSelf", manual_reply = true)]
fn get_self(id: Principal) -> ManualReply<User> {
    USERS_STORE.with(|users_store| {
        if let Some(user) = users_store.borrow().get(&id) {
            ManualReply::one(user)
        } else {
            ManualReply::one(User::default())
        }
    })
}

#[query(manual_reply = true)]
fn get(id: Principal, name: String) -> ManualReply<User> {
    ID_STORE.with(|id_store| {
        USERS_STORE.with(|users_store| {
            let users_store = users_store.borrow();
            if let Some(user) = id_store
                .borrow()
                .get(&name)
                .and_then(|id| users_store.get(id))
            {
                ManualReply::one(user)
            } else {
                ManualReply::one(User::default())
            }
        })
    })
}

#[update]
fn update(id: Principal, user: User) {
    ID_STORE.with(|id_store| {
        id_store
            .borrow_mut()
            .insert(user.name.clone(), id);
    });



    USERS_STORE.with(|users_store| {
        users_store.borrow_mut().insert(id, user);
    });
}

#[query(manual_reply = true)]
fn search(id: Principal, text: String) -> ManualReply<Option<User>> {
    let text = text.to_lowercase();
    USERS_STORE.with(|users_store| {
        for (_, p) in users_store.borrow().iter() {
            if p.name.to_lowercase().contains(&text) || p.description.to_lowercase().contains(&text)
            {
                return ManualReply::one(Some(p));
            }

            for x in p.social_media.iter() {
                if x.name.to_lowercase() == text || x.nickname.to_lowercase() == text {
                    return ManualReply::one(Some(p));
                }
            }
        }
        ManualReply::one(None::<User>)
    })
}

// Enable Candid export
ic_cdk::export_candid!();