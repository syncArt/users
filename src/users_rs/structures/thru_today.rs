use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ToDo {
    text: String,
    is_finished: bool,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Meals {
    breakfast: String,
    lunch: String,
    dinner: String,
    snacks: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct NoteRecord {
    top3goals: Vec<String>,
    mood: i8,
    score: i8,
    hydration: i8,
    notes: String,
    meals: Meals,
    to_do: Vec<ToDo>,
    schedule: Vec<String>,
    follow_ups: Vec<String>,
    notes_for_tomorrow: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ThruToday {
    is_suspended: bool,
    live_data: Vec<NoteRecord>,
    historical_data: Vec<NoteRecord>,
}
