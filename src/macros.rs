/// To let a struct which implements the `SimpleAuthorization` trait become an authorizer.
#[macro_export]
macro_rules! authorizer {
    ( $name:ident ) => {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for $name {
            type Error = ();

            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, Self::Error> {
                let keys: Vec<&str> = request.headers().get("authorization").collect();

                let key = keys.into_iter().next();

                match <$name as ::rocket_simple_authorization::SimpleAuthorization>::has_authority(key) {
                    Some(key) => ::rocket::Outcome::Success(<$name as ::rocket_simple_authorization::SimpleAuthorization>::create_auth(key)),
                    None => ::rocket::Outcome::Forward(())
                }
            }
        }
    }
}