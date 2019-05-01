/*!
# `simple-authorization` Request Guard for Rocket Framework

This crate provides a request guard builder used for authorization.

See `examples`.
*/

mod macros;

/// The trait for an authorizer.
pub trait SimpleAuthorization<E = String> {
    /// Check whether the key is valid or not. And a generic type can also be returned to help create an auth instance.
    fn has_authority<S: AsRef<str>>(key: Option<S>) -> Option<Option<E>>;

    /// Create an auth instance.
    fn create_auth(key: Option<E>) -> Self;
}