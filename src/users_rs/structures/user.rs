use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;
use candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum EventEnum {
    UpdateSmileyball(Smileyball),
    UpdateThruToday(ThruToday),
}

#[derive(Clone, Debug, CandidType, PartialEq, Deserialize)]
pub enum AppTypeEnum {
    Smileyball,
    ThruToday,
    General,

}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum AppDataEnum {
    Smileyball(Smileyball),
    ThruToday(ThruToday),
    General,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GeneralInfo {
    pub nickname: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AppsData {
    pub connected_apps: Vec<AppTypeEnum>,
    pub registry: HashMap<String, AppDataEnum>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub nickname: String,
    pub description: Option<String>,
    pub apps_data: AppsData,
}

impl User {
    pub fn new() -> Self {
        Self {
            nickname: String::new(),
            description: Option::from(String::new()),
            apps_data: Default::default(),
        }
    }

    pub fn create_account(
        &mut self,
        nickname: String,
        description: Option<String>,
        app_name: AppTypeEnum,
    ) -> &mut Self {
        self.nickname = nickname;
        self.description = description;

        let initial_app_data = match app_name {
            AppTypeEnum::Smileyball => AppDataEnum::Smileyball(Smileyball::default()),
            AppTypeEnum::ThruToday => AppDataEnum::ThruToday(ThruToday::default()),
            AppTypeEnum::General => AppDataEnum::General,
        };

        let mut registry = HashMap::new();
        registry.insert(format!("{:?}", app_name), initial_app_data);

        self.apps_data = AppsData {
            connected_apps: vec![app_name],
            registry,
        };

        self
    }

    pub fn update_general_info(
        &mut self,
        nickname: String,
        description: Option<String>,
    ) -> &mut Self {
        self.nickname = nickname;
        self.description = description;
        self
    }

    pub fn update_app_data(
        &mut self,
        update_data: AppDataEnum,
        app_type: AppTypeEnum,
    ) -> &mut Self {
        let app_key = format!("{:?}", app_type);

        if let Some(existing_data) = self.apps_data.registry.get_mut(&app_key) {
            match existing_data {
                AppDataEnum::Smileyball(existing_smileyball) => {
                    if let AppDataEnum::Smileyball(new_smileyball) = update_data {
                        existing_smileyball.update(&new_smileyball);
                    } else {
                        ic_cdk::println!("Mismatched app data type for Smileyball");
                    }
                }
                AppDataEnum::ThruToday(existing_thru_today) => {
                    if let AppDataEnum::ThruToday(new_thru_today) = update_data {
                        existing_thru_today.update(new_thru_today);
                    } else {
                        ic_cdk::println!("Mismatched app data type for ThruToday");
                    }
                }
                AppDataEnum::General => {
                    ic_cdk::println!("General data");
                }
            }
        } else {
            self.apps_data.registry.insert(app_key, update_data);
        }

        if !self.apps_data.connected_apps.contains(&app_type) {
            self.apps_data.connected_apps.push(app_type);
        }

        self
    }

    pub fn register_new_app(&mut self, app_type: AppTypeEnum) -> &mut Self {
        if !self.apps_data.connected_apps.contains(&app_type) {
            self.apps_data.connected_apps.push(app_type);
        }

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
