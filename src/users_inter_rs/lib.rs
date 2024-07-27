use ic_cdk::{query, update};

mod declarations;
use declarations::users_rs::{users_rs, User};

#[update(name = "getSelf")]
async fn get_self() -> User {
    let id = ic_cdk::api::caller();
    users_rs.get_self(id).await.unwrap().0
}

#[update]
async fn get(name: String) -> User {
    users_rs.get(name).await.unwrap().0
}

#[update]
async fn update(user: User) -> String {
    let id = ic_cdk::api::caller();
    users_rs.update(id, user).await.unwrap().0
}

// Enable Candid export
ic_cdk::export_candid!();