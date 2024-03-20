use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageType {
    message_id: String,
    scaling: String,
    exhaustion: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    effects: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    death_message_type: Option<String>,
}