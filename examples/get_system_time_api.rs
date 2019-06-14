#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_simple_authorization;

extern crate short_crypt;
extern crate chrono;

use rocket::State;
use rocket::request::Request;
use rocket::http::Status;

use rocket_simple_authorization::SimpleAuthorization;

use short_crypt::ShortCrypt;
use chrono::prelude::*;

const KEY: &'static str = "magickey";

// 1. Implement any struct you want for authorization.
pub struct Auth {
    auth_data: String
}

impl Auth {
    pub fn as_str(&self) -> &str {
        self.auth_data.as_str()
    }
}

// 2. Implement `SimpleAuthorization` for the auth struct.
impl<'a, 'r> SimpleAuthorization<'a, 'r> for Auth {
    fn authorizing(request: &'a Request<'r>, authorization: Option<&'a str>) -> Option<Self> {
        let sc = request.guard::<State<ShortCrypt>>().unwrap();

        match authorization {
            Some(authorization) => {
                match sc.decrypt_url_component(authorization) {
                    Ok(result) => {
                        match String::from_utf8(result) {
                            Ok(user_name) => Some( Auth {
                                auth_data: user_name
                            }),
                            Err(_) => None
                        }
                    }
                    Err(_) => None
                }
            }
            None => None
        }
    }
}

// 3. Make the auth struct be an authorizer(cacheable).
authorizer!(ref Auth);

// 4. Use the auth struct as a request guard.
#[get("/time")]
fn system_time(auth: &Auth) -> Option<String> {
    // 5. Handle the auth struct.
    match auth.as_str() {
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
    rocket::ignite().manage(ShortCrypt::new(KEY)).mount("/", routes![system_time, system_time_401]).launch();
}