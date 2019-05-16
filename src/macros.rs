/// To let a struct which implements the `SimpleAuthorization<E>` trait become an authorizer. The default `<E>` is `<&'a str>`.
#[macro_export]
macro_rules! authorizer {
    ( $name:ty ) => {
        authorizer!($name, &'a str);
    };
    ( $name:ty, $typ:ty ) => {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for $name {
            type Error = ();

            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, Self::Error> {
                let keys: Vec<&str> = request.headers().get("authorization").collect();

                let key = keys.into_iter().next();

                match <$name as ::rocket_simple_authorization::SimpleAuthorization<$typ>>::has_authority(key) {
                    Some(key) => ::rocket::Outcome::Success(<$name as ::rocket_simple_authorization::SimpleAuthorization<$typ>>::create_auth(key)),
                    None => ::rocket::Outcome::Forward(())
                }
            }
        }
    };
    ( ref $name:ty, $typ:ty ) => {
        authorizer!($name, $typ);
    
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for &'a $name {
            type Error = ();

            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, Self::Error> {
                let cache = request.local_cache(|| {
                    let keys: Vec<&str> = request.headers().get("authorization").collect();

                    let key = keys.into_iter().next();

                    match <$name as ::rocket_simple_authorization::SimpleAuthorization<$typ>>::has_authority(key) {
                        Some(key) => Some(<$name as ::rocket_simple_authorization::SimpleAuthorization<$typ>>::create_auth(key)),
                        None => None
                    }
                });

                match cache.as_ref() {
                    Some(client_addr) => ::rocket::Outcome::Success(client_addr),
                    None => ::rocket::Outcome::Forward(())
                }
            }
        }
    }
}