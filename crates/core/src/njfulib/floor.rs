use crate::def;
use crate::error::ClientError;
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// # Floor information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Floor {
    pub room_id: u32,
    pub room_name: String,
    pub dev_start: u32,
    pub dev_end: u32,
    pub site_num: u32,
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, "roomId: {}", self.room_id)?;
        writeln!(f, "roomName: {}", self.room_name)?;
        writeln!(f, "devStart: {}", self.dev_start)?;
        writeln!(f, "devEnd: {}", self.dev_end)?;
        writeln!(f, "siteNum: {}", self.site_num)?;
        Ok(())
    }
}

impl Floor {
    pub fn new(
        room_id: u32,
        room_name: String,
        dev_start: u32,
        dev_end: u32,
        site_num: u32,
    ) -> Self {
        Floor {
            room_id,
            room_name,
            dev_start,
            dev_end,
            site_num,
        }
    }
}

pub fn get_floor_by_room_id(room_id: u32) -> Result<Floor> {
    for floor in def::FLOORS.iter() {
        if floor.room_id == room_id {
            return Ok(floor.clone());
        }
    }
    Err(anyhow!(ClientError::RoomError))
}

pub fn get_floor_by_room_name(room_name: &str) -> Result<Floor> {
    for floor in def::FLOORS.iter() {
        if floor.room_name == room_name {
            return Ok(floor.clone());
        }
    }
    Err(anyhow!(ClientError::RoomError))
}
