#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_simple_authorization;

#[macro_use]
extern crate lazy_static;

extern crate short_crypt;
extern crate chrono;

use rocket::http::Status;

use rocket_simple_authorization::SimpleAuthorization;

use short_crypt::ShortCrypt;
use chrono::prelude::*;

const KEY: &'static str = "magickey";

lazy_static! {
    static ref SC: ShortCrypt = {
        ShortCrypt::new(KEY)
    };
}

// 1. Implement any struct you want for authorization.
pub struct Auth {
    auth_data: String
}

// 2. Implement `SimpleAuthorization<E>` for the auth struct.
impl<'a> SimpleAuthorization<'a, String> for Auth {
    fn has_authority(key: Option<&'a str>) -> Option<Option<String>> {
        match key {
            Some(key) => {
                match SC.decrypt_url_component(key) {
                    Ok(result) => {
                        match String::from_utf8(result) {
                            Ok(user_name) => Some(Some(user_name)),
                            Err(_) => None
                        }
                    }
                    Err(_) => None
                }
            }
            None => None
        }
    }

    fn create_auth(user_name: Option<String>) -> Auth {
        Auth {
            auth_data: user_name.unwrap()
        }
    }
}

// 3. Make the auth struct be a authorizer.
authorizer!(Auth, String);

// 4. Use the auth struct as a request guard.
#[get("/time")]
fn system_time(auth: Auth) -> Option<String> {
    // 5. Handle the auth struct.
    match auth.auth_data.as_str() {
        "magiclen.org" => (),
        _ => return None
    }

    let utc: DateTime<Utc> = Utc::now();

    Some(utc.format("%Y-%m-%d-%H-%M-%S").to_string())
}

#[get("/time", rank = 2)]
fn system_time_401() -> Status {
    Status::Unauthorized
}

fn main() {
    rocket::ignite().mount("/", routes![system_time, system_time_401]).launch();
}