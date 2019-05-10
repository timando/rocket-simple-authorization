/// To let a struct which implements the `SimpleAuthorization<E>` trait become an authorizer. The default `<E>` is `<String>`.
#[macro_export]
macro_rules! authorizer {
    ( $name:ty ) => {
        authorizer!($name, &'a str);
    };
    ( $name:ty, $tpy:ty ) => {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for $name {
            type Error = ();

            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, Self::Error> {
                let keys: Vec<&str> = request.headers().get("authorization").collect();

                let key = keys.into_iter().next();

                match <$name as ::rocket_simple_authorization::SimpleAuthorization<$tpy>>::has_authority(key) {
                    Some(key) => ::rocket::Outcome::Success(<$name as ::rocket_simple_authorization::SimpleAuthorization<$tpy>>::create_auth(key)),
                    None => ::rocket::Outcome::Forward(())
                }
            }
        }
    }
}