use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Mood {
    name: String,
    primary_emoji: String,
    secondary_emoji: String,
    background: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Moodmoji {
    mood: Mood,
    data: Vec<Vec<String>>,
}
