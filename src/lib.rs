/*!
# `simple-authorization` Request Guard for Rocket Framework

This crate provides a request guard builder used for authorization.

See `examples`.
*/

mod macros;

extern crate rocket;

use rocket::request::Request;

/// The trait for an authorizer.
pub trait SimpleAuthorization<'a, 'r>: where Self: Sized {
    /// Check whether the value in the `Authorization` header is valid or not. If it is valid, create a new instance of `Self`.
    fn authorizing(request: &'a Request<'r>, authorization: Option<&'a str>) -> Option<Self>;
}