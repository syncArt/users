use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;
use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum EventEnum {
    UpdateSmileyball(Smileyball),
    UpdateThruToday(ThruToday),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum AppTypes {
    Smileyball,
    ThruToday,
}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum AllowedAppsData {
    Smileyball(Smileyball),
    ThruToday(ThruToday),
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AppsData {
    connected_apps: Option<Vec<String>>,
    pub registry: Option<HashMap<String, AllowedAppsData>>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub nickname: String,
    pub description: Option<String>,
    pub apps_data: Option<AppsData>,
}

impl User {
    pub fn new() -> Self {
        User {
            nickname: "test".to_string(),
            description: None,
            apps_data: None,
        }
    }

    pub fn create_account(
        mut self,
        nickname: String,
        description: Option<String>,
        app_name: String,
    ) -> Self {
        self.nickname = nickname;
        self.description = description;
        self.apps_data = Some(AppsData {
            connected_apps: Some(vec![app_name]),
            registry: None,
        });
        self
    }

    pub fn update_user(mut self, update_data: User) -> Self {
        self.nickname = update_data.nickname;
        self.description = update_data.description;
        self
    }

    pub fn build(self) -> Option<User> {
        if let nickname = self.nickname {
            Some(User {
                nickname,
                description: self.description,
                apps_data: self.apps_data,
            })
        } else {
            None
        }
    }
}
