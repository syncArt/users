use crate::utilities::helpers::generate_random_id;
use candid::{CandidType, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct SmileyballInput {
    pub is_suspended: Option<bool>,
    pub created_contests: HashSet<u64>,
    pub voted_songs: HashMap<u64, VotedSongRecord>,
    pub won_badges: Vec<BadgeRecord>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Smileyball {
    pub is_suspended: bool,
    pub created_contests: HashSet<u64>,             // contest_id
    pub voted_songs: HashMap<u64, VotedSongRecord>, // <contest_id, data>
    pub won_badges: HashMap<u64, BadgeRecord>,      // <badge_id, data>
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct VotedSongRecord {
    pub song_id: u64,
    pub vote: u8,
    pub voted_at: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BadgeRecord {
    pub contest_id: u64,
    pub song_id: u64,
    pub awarded_at: u64,
}

impl Smileyball {
    pub async fn update(&mut self, new_data: &Smileyball) -> Result<(), String> {
        self.is_suspended = new_data.is_suspended;

        self.created_contests
            .extend(new_data.created_contests.clone());

        self.voted_songs.extend(new_data.voted_songs.clone());

        for badge_record in new_data.won_badges.values() {
            self.add_badge(badge_record.clone()).await?;
        }

        Ok(())
    }
    pub async fn add_badge(&mut self, badge_record: BadgeRecord) -> Result<u64, String> {
        let badge_id = generate_random_id().await?;
        self.won_badges.insert(badge_id, badge_record);
        Ok(badge_id)
    }

    pub async fn from_input(input: SmileyballInput) -> Result<Self, String> {
        let mut smileyball = Smileyball::default();

        smileyball.is_suspended = input.is_suspended.unwrap_or(false);
        smileyball.created_contests = input.created_contests;
        smileyball.voted_songs = input.voted_songs;

        for badge in input.won_badges {
            let badge_id = generate_random_id()
                .await
                .map_err(|e| format!("Failed to generate ID: {}", e))?;
            smileyball.won_badges.insert(badge_id, badge);
        }

        Ok(smileyball)
    }

    pub fn to_input(&self) -> SmileyballInput {
        SmileyballInput {
            is_suspended: Some(self.is_suspended),
            created_contests: self.created_contests.clone(),
            voted_songs: self.voted_songs.clone(),
            won_badges: self.won_badges.values().cloned().collect(),
        }
    }
}
