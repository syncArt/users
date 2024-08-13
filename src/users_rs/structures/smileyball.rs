use crate::structures::moodmoji::Moodmoji;
use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ContestRecord {
    pub song_id: String,
    pub moodmoji: Moodmoji,
    pub added_at: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ContestData {
    lobby: Vec<ContestRecord>,
    voted: Vec<ContestRecord>,
    historical_data: Vec<ContestRecord>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Smileyball {
    is_suspended: Option<bool>,
    moodmoji: Moodmoji,
    contest: Option<ContestData>,
}
