/*!
# `simple-authorization` Request Guard for Rocket Framework

This crate provides a request guard builder used for authorization.

See `examples`.
*/

mod macros;

extern crate rocket;

use rocket::request::Request;

/// The trait for an authorizer.
pub trait SimpleAuthorization<'a, 'r, E = &'a str> {
    /// Check whether the key is valid or not. And a generic type can also be returned to help create an auth instance.
    fn has_authority(request: &'a Request<'r>, key: Option<&'a str>) -> Option<Option<E>>;

    /// Create an auth instance.
    fn create_auth(key: Option<E>) -> Self;
}