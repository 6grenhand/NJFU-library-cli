use super::dev::Dev;
use crate::def;
use crate::utils::time;
use serde::{Deserialize, Serialize};

/// Status struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resv {
    pub uuid: String,
    #[serde(rename = "resvId")]
    pub resv_id: u32,
    #[serde(rename = "appAccNo")]
    pub appacc_no: u32,
    #[serde(rename = "resvBeginTime")]
    pub resv_begin_time: u64,
    #[serde(rename = "resvEndTime")]
    pub resv_end_time: u64,
    #[serde(rename = "resvDevInfoList")]
    pub resv_dev_info_list: Option<Vec<Dev>>,
}

impl std::fmt::Display for Resv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        if let Some(devs) = &self.resv_dev_info_list {
            for dev in devs {
                write!(f, "{}", dev)?;
            }
        }
        writeln!(
            f,
            "begin time: {}",
            time::get_date_with_time_stamp(self.resv_begin_time / 1000)
        )?;
        writeln!(
            f,
            "end time: {}",
            time::get_date_with_time_stamp(self.resv_end_time / 1000)
        )?;
        writeln!(f, "uuid: {}", self.uuid)?;
        Ok(())
    }
}

impl Resv {
    pub fn new(
        uuid: String,
        resv_id: u32,
        appacc_no: u32,
        resv_begin_time: u64,
        resv_end_time: u64,
        resv_dev_info_list: Option<Vec<Dev>>,
    ) -> Self {
        Resv {
            uuid,
            resv_id,
            appacc_no,
            resv_begin_time,
            resv_end_time,
            resv_dev_info_list,
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn resv_id(&self) -> u32 {
        self.resv_id
    }

    pub fn appacc_no(&self) -> u32 {
        self.appacc_no
    }

    pub fn resv_begin_time(&self) -> u64 {
        self.resv_begin_time
    }

    pub fn resv_end_time(&self) -> u64 {
        self.resv_end_time
    }

    pub fn resv_dev_info_list(&self) -> Option<&Vec<Dev>> {
        self.resv_dev_info_list.as_ref()
    }
}
