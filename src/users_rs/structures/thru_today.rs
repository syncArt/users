use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ToDo {
    pub text: String,
    pub is_finished: bool,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Meals {
    pub breakfast: String,
    pub lunch: String,
    pub dinner: String,
    pub snacks: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct NoteRecord {
    pub top3goals: Vec<String>,
    pub mood: i8,
    pub score: i8,
    pub hydration: i8,
    pub notes: String,
    pub meals: Meals,
    pub to_do: Vec<ToDo>,
    pub schedule: Vec<String>,
    pub follow_ups: Vec<String>,
    pub notes_for_tomorrow: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ThruToday {
    pub is_suspended: bool,
    pub live_data: Vec<NoteRecord>,
    pub historical_data: Vec<NoteRecord>,
}

impl ThruToday {
    pub fn update(&mut self, update_data: ThruToday) {
        self.is_suspended = update_data.is_suspended;
        self.live_data = update_data.live_data;
        self.historical_data = update_data.historical_data;
    }
}
