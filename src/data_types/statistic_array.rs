use super::{ToProtocol, VarInt};

pub struct Statistic {
    category_id: VarInt,
    statistic_id: VarInt,
    value: VarInt,
}

impl ToProtocol for Statistic {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.category_id
        .to_protocol_bytes()
        .into_iter()
        .chain(
            self.statistic_id
            .to_protocol_bytes()
            .into_iter()
            .chain(
                self.value
                .to_protocol_bytes()
                .into_iter()
            )
        ).collect()
    }
}

pub type StatisticArray = Vec<Statistic>;

impl ToProtocol for StatisticArray {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        VarInt::new(self.len() as i32)
        .to_protocol_bytes()
        .into_iter()
        .chain(
            self
            .into_iter()
            .map(|statistic| {
                statistic
                .to_protocol_bytes()
                .into_iter()
            }).flatten()
        ).collect()
    }
}