use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::data_types::{identifier::Identifier, text_component::Json, TextComponent};



#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
/// Fields do not use snake_case to match Minecraft's default NBT tag naming scheme
pub struct EntityData {
    Air: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    CustomName: Option<TextComponent<Json>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    CustomNameVisible: Option<bool>,
    FallDistance: f32,
    Fire: i16,
    Glowing: bool,
    HasVisualFire: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    Invulnerable: bool,
    Motion: Vec<f64>, //always of size 3 -- dX, dY, dZ in blocks/tick
    NoGravity: bool,
    OnGround: bool,
    Passengers: Vec<EntityData>,
    PortalCooldown: i32,
    Pos: Vec<f64>, //always of size 3 -- x, y, and x coordinates of entity
    Rotation: Vec<f32>, //Always of size 2. Angles, rotation of entity from -180 to 180. May want to modify the Angle struct to use for this. Yaw first, then pitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    Silent: Option<bool>,
    Tags: Vec<TempEmptyTag>, //Scoreboard tags of entity, for our implementation for now we'll leave this as a list of size 0 for obvious reasons.
    #[serde(skip_serializing_if = "Option::is_none")]
    TicksFrozen: Option<i32>, //ONly used by mobs that are not `freeze_imune_entity_types`,
    entity_uuid: Vec<i32>, //Stored as 4 integers forming the Uuid
}

impl EntityData {
    pub fn new() -> Self {
        EntityData {
            Air : 0,
            CustomName : None,
            CustomNameVisible : None,
            FallDistance : 0.0,
            Fire : -20,
            Glowing : false,
            HasVisualFire : false,
            id : None,
            Invulnerable : false,
            Motion: vec![0.0, 0.0, 0.0],
            NoGravity : false,
            OnGround : true,
            Passengers : vec![],
            PortalCooldown : 300,
            Pos : vec![0.0, 0.0, 0.0],
            Rotation : vec![0.0f32, 0.0f32],
            Silent : None,
            Tags : vec![],
            TicksFrozen : None,
            entity_uuid : Uuid::new_v4()
                            .as_bytes()
                            .chunks(4)
                            .map(
                                |bytes| i32::from_ne_bytes(bytes[0..4].try_into().unwrap())
                            )
                            .collect::<Vec<i32>>(),
        }
    }

    pub fn set_air(mut self, air: i32) {
        self.Air = air;
    }

    pub fn set_CustomName(mut self, custom_name: Option<TextComponent<Json>>) {
        self.CustomName = custom_name;
    }

    pub fn set_CustomNameVisible(mut self, custom_name_visible: Option<bool>) {
        self.CustomNameVisible = custom_name_visible;
    }

    pub fn set_FallDistance(mut self, fall_distance: f32) {
        self.FallDistance = fall_distance;
    }

    /// Number of ticks entity is on fire
    pub fn set_Fire(mut self, fire_ticks: i16) {
        self.Fire = fire_ticks;
    }

    pub fn set_Glowing(mut self, glowing: bool) {
        self.Glowing = glowing;
    }

    //...
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempEmptyTag;