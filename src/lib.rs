/*!
# `simple-authorization` Request Guard for Rocket Framework

This crate provides a request guard builder used for authorization.

See `examples`.
*/

mod macros;

/// The trait for an authorizer.
pub trait SimpleAuthorization {
    fn has_authority<S: AsRef<str>>(key: Option<S>) -> bool;

    fn create_auth<S: Into<String>>(key: Option<S>) -> Self;
}