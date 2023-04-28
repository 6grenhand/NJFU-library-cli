use super::cli::day::Day;
use crate::role::login::Login;
use crate::role::site::*;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::def;
use crate::utils::json;
use crate::utils::*;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use std::collections::HashMap;

/// The context of the program.
#[derive(Debug)]
pub struct Context {}

impl Context {
    /// # Context constructor.
    /// Create a new context.
    pub fn new() -> Self {
        Self {}
    }

    /// # Query the information of a student.
    pub fn query_by_name(&self, day: Day, name: String) -> Result<Vec<Site>> {
        let mut body = HashMap::new();
        body.insert("byType", "devcls");
        body.insert("classkind", "8");
        body.insert("cld_name", "default");
        body.insert("act", "get_rsv_sta");
        let date = match day {
            Day::Today => time::get_date_today("%Y-%m-%d"),
            Day::Tomorrow => time::get_date_tomorrow("%Y-%m-%d"),
        };
        body.insert("date", date.as_str());

        let mut ret: Vec<Site> = Vec::new();

        for (room_name, floor) in def::ROOMS.iter() {
            let room_id = floor.room_id().to_string();
            let mut data = body.clone();
            data.insert("room_id", room_id.as_str());

            let resp = http::post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), data)
                .unwrap_or_else(|err| {
                    panic!(
                        "parse error when scan student in {} \n detail: {}",
                        room_name, err
                    )
                });

            match json::get_name_info(resp, name.clone()) {
                Ok(info) => {
                    ret.append(info.into_iter().collect::<Vec<Site>>().as_mut());
                }
                Err(err) => return Err(err),
            }
        }
        Ok(ret)
    }

    /// # Query the information of a site.
    pub fn query_by_site(&self, day: Day, site: String) -> Result<Site> {
        let dev_id = get_site_id(site.clone());
        match dev_id {
            Ok(dev_id) => {
                let mut body = HashMap::new();
                body.insert("dev_id", dev_id.as_str());
                body.insert("act", "get_rsv_sta");
                let date = match day {
                    Day::Today => time::get_date_today("%Y-%m-%d"),
                    Day::Tomorrow => time::get_date_tomorrow("%Y-%m-%d"),
                };
                body.insert("date", date.as_str());

                let resp = http::post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), body)
                    .expect("net error when querying site");

                json::get_site_info(resp)
            }
            Err(e) => Err(e),
        }
    }

    /// # handle actual login to the server.
    fn handle_login(&self) -> Result<Student> {
        let mut login = Login::new("".to_string(), "".to_string());
        login.read_from_file().expect("read student info failed");

        let mut body = HashMap::new();
        body.insert("act", "login");
        body.insert("id", login.username());
        body.insert("pwd", login.password());
        let resp = http::post(def::LOGIN_URL.as_str(), def::HEADERMAP.clone(), body);
        match resp {
            Ok(resp) => json::get_login_info(resp),
            Err(e) => Err(e),
        }
    }

    /// # login to the server.
    pub fn login(&self, username: String, password: String) -> Result<Student> {
        let student = Login::new(username.clone(), password.clone());
        student.save_to_file().expect("save student info failed");
        self.handle_login()
    }

    /// # query the user status.
    pub fn status(&self) -> Result<Vec<State>> {
        //login
        self.handle_login()?;

        let mut body = HashMap::new();
        body.insert("act", "get_History_resv");
        body.insert("strat", "90");
        body.insert("StatFlag", "New");

        let resp = http::post(def::CENTER_URL.as_str(), def::HEADERMAP.clone(), body);
        match resp {
            Ok(resp) => json::get_state_info(resp),
            Err(e) => Err(e),
        }
    }

    /// # cancel the reservation.
    pub fn cancel(&self, id: String) -> Result<String> {
        //login
        self.handle_login()?;

        let mut body = HashMap::new();
        body.insert("act", "del_resv");
        body.insert("id", id.as_str());

        let resp = http::post(def::RESERVE_URL.as_str(), def::HEADERMAP.clone(), body);
        match resp {
            Ok(resp) => json::get_cancel_info(resp),
            Err(e) => Err(e),
        }
    }

    fn handle_reserve(&self, site: String, day: Day, start: String, end: String) -> Result<String> {
        let id = get_site_id(site)?;

        let mut body = HashMap::new();
        body.insert("act", "set_resv");
        body.insert("dev_id", id.as_str());
        let day = match day {
            Day::Today => time::get_date_today("%Y-%m-%d"),
            Day::Tomorrow => time::get_date_tomorrow("%Y-%m-%d"),
        };
        let start_time = format!("{} {}", day, start);
        let end_time = format!("{} {}", day, end);
        body.insert("start", start_time.as_str());
        body.insert("end", end_time.as_str());

        let resp = http::post(def::RESERVE_URL.as_str(), def::HEADERMAP.clone(), body);
        match resp {
            Ok(resp) => json::get_reserve_info(resp),
            Err(e) => Err(e),
        }
    }

    /// # reserve the site.
    pub fn reserve(
        &self,
        sites: Vec<String>,
        day: Day,
        start: String,
        end: String,
    ) -> Result<String> {
        //login
        self.handle_login()?;

        let mut ret = String::new();

        for site in sites {
            let resp = self.handle_reserve(site.clone(), day.clone(), start.clone(), end.clone());
            match resp {
                Ok(resp) => {
                    ret.push_str(format!("{}: {}\n", site, resp).as_str());
                    if resp.contains("成功") {
                        return Ok(ret);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Err(anyhow!("no site from put in can be reserved"))
    }

    /// check out site
    pub fn check_out(&self, id: String) -> Result<String> {
        //login
        self.handle_login()?;

        let mut body = HashMap::new();
        body.insert("act", "resv_leave");
        body.insert("type", "2");
        body.insert("resv_id", id.as_str());

        let resp = http::post(def::RESERVE_URL.as_str(), def::HEADERMAP.clone(), body);
        match resp {
            Ok(resp) => json::get_check_out_info(resp),
            Err(e) => Err(e),
        }
    }

    /// get all floors
    pub fn floor_state(&self) -> Result<Vec<&'static str>> {
        Ok(vec![
            "2F-A", "2F-B", "3F-A", "3F-B", "3F-C", "3FA-", "4F-A", "4FA-", "5F-A", "6F-A", "7F-A",
        ])
    }

    /// get author info
    pub fn author_state(&self) -> Result<String> {
        Ok(String::from("author: 蒋雨峰\n南京林业大学20级本科生\n"))
    }

    /// get user info
    pub fn user_state(&self) -> Result<Login> {
        let mut login = Login::new("".to_string(), "".to_string());
        login.read_from_file()?;
        Ok(login)
    }
}
