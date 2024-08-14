use crate::structures::smileyball::Smileyball;
use crate::structures::thru_today::ThruToday;

#[derive(Clone, Debug)]
pub enum AppDataInputEnum {
    Smileyball(Smileyball),
    ThruToday(ThruToday),
}

#[derive(Clone, Debug)]
pub struct CreateAccountInput {
    pub nickname: String,
    pub description: Option<String>,
    pub app_name: AppDataInputEnum,
}

#[derive(Clone, Debug)]
pub struct UpdateAppDataInput {
    pub nickname: Option<String>,
    pub description: Option<String>,
    pub app_data: AppDataInputEnum,
}

#[derive(Clone, Debug)]
pub struct DeleteAppDataInput {
    pub app_data: AppDataInputEnum,
}
