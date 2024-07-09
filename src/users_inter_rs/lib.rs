use ic_cdk::update;

mod declarations;
use declarations::users_rs::{users_rs, User};

#[update(name = "getSelf")]
async fn get_self() -> User {
    users_rs.get_self().await.unwrap().0
}

#[update]
async fn get(name: String) -> User {
    users_rs.get(name).await.unwrap().0
}

#[update]
async fn update(user: User) {
    users_rs.update(user).await.unwrap()
}

#[update]
async fn search(text: String) -> Option<User> {
    users_rs.search(text).await.unwrap().0
}

// Enable Candid export
ic_cdk::export_candid!();