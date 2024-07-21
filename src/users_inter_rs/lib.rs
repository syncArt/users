use ic_cdk::update;
use ic_cdk::query;
use candid::{Principal};

mod declarations;
use declarations::users_rs::{users_rs, User};

#[update(name = "getSelf")]
async fn get_self() -> User {
    let id = ic_cdk::api::caller();
    users_rs.get_self(id).await.unwrap().0
}

#[update]
async fn get(name: String) -> User {
    let id = ic_cdk::api::caller();
    users_rs.get(id, name).await.unwrap().0
}

#[update]
async fn update(user: User) {
    let id = ic_cdk::api::caller();
    users_rs.update(id, user).await.unwrap()
}

#[update]
async fn search(text: String) -> Option<User> {
    let id = ic_cdk::api::caller();
    users_rs.search(id, text).await.unwrap().0
}

// Enable Candid export
ic_cdk::export_candid!();