#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_simple_authorization;

extern crate chrono;

use rocket::http::Status;

use rocket_simple_authorization::SimpleAuthorization;

use chrono::prelude::*;

pub struct Auth;

impl SimpleAuthorization for Auth {
    fn has_authority<S: AsRef<str>>(key: Option<S>) -> bool {
        match key {
            Some(key) => key.as_ref().eq("magickey"),
            None => false
        }
    }

    fn create_auth<S: AsRef<str>>(_key: Option<S>) -> Auth {
        Auth
    }
}

authorizer!(Auth);

#[get("/time")]
fn system_time(_auth: Auth) -> String {
    let utc: DateTime<Utc> = Utc::now();

    utc.format("%Y-%m-%d-%H-%M-%S").to_string()
}

#[get("/time", rank = 2)]
fn system_time_401() -> Status {
    Status::Unauthorized
}

fn main() {
    rocket::ignite().mount("/", routes![system_time]).mount("/", routes![system_time_401]).launch();
}