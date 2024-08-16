use crate::structures::user::{AppDataEnum, AppTypeEnum};
use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct CreateAccountInput {
    pub nickname: String,
    pub description: Option<String>,
    pub app_canister_id: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateAppDataInput {
    pub app_data: Option<AppDataEnum>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpdateGeneralInfoInput {
    pub nickname: String,
    pub description: Option<String>,
}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UpdateOrCreateUserInput {
    pub general_info: Option<UpdateGeneralInfoInput>,
    pub apps_data: Option<AppDataEnum>,
    pub app_type: AppTypeEnum,
}
