use crate::structures::moodmoji::Moodmoji;
use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ContestRecord {
    pub song_id: String,
    pub moodmoji: Option<Moodmoji>,
    pub added_at: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ContestData {
    lobby: Option<Vec<ContestRecord>>,
    voted: Option<Vec<ContestRecord>>,
    historical_data: Option<Vec<ContestRecord>>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Smileyball {
    is_suspended: Option<bool>,
    moodmoji: Option<Moodmoji>,
    contest: Option<ContestData>,
}

impl Smileyball {
    pub fn update(&mut self, update_data: &Smileyball) {
        if let Some(is_suspended) = &update_data.is_suspended {
            self.is_suspended = Some(is_suspended.clone());
        }
        if let Some(moodmoji) = &update_data.moodmoji {
            self.moodmoji = Some(moodmoji.clone());
        }
        if let Some(contest) = &update_data.contest {
            self.contest = Some(contest.clone());
        }
    }
}
