/*!
# `simple-authorization` Request Guard for Rocket Framework

This crate provides a request guard builder used for authorization.

See `examples`.
*/

mod macros;

#[macro_use]
#[doc(hidden)]
pub extern crate rocket;

use rocket::request::Request;

/// The trait for an authorizer.
#[async_trait]
pub trait SimpleAuthorization<'r>
where
    Self: Sized, {
    /// Check whether the value in the `Authorization` header is valid or not. If it is valid, create a new instance of `Self`.
    async fn authorizing(request: &'r Request<'_>, authorization: Option<&'r str>) -> Option<Self>;
}
