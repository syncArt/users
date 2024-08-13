use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;
use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AppsData {
    connected_apps: Option<Vec<String>>,
    smileyball: Option<Smileyball>,
    thru_today: Option<ThruToday>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {
    pub nickname: String,
    pub description: String,
    pub apps_data: Option<AppsData>,
}
