/// To let a struct which implements the `SimpleAuthorization` trait become an authorizer.
#[macro_export]
macro_rules! authorizer {
    ( $name:ty ) => {
        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for $name {
            type Error = ();

            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, Self::Error> {
                let key: Option<&str> = request.headers().get("authorization").next();

                match <$name as ::rocket_simple_authorization::SimpleAuthorization>::authorizing(request, key) {
                    Some(ins) => ::rocket::Outcome::Success(ins),
                    None => ::rocket::Outcome::Forward(())
                }
            }
        }
    };
    ( ref $name:ty ) => {
        authorizer!($name);

        impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for &'a $name {
            type Error = ();

            fn from_request(request: &'a ::rocket::request::Request<'r>) -> ::rocket::request::Outcome<Self, Self::Error> {
                let cache = request.local_cache(|| {
                    let key: Option<&str> = request.headers().get("authorization").next();

                    match <$name as ::rocket_simple_authorization::SimpleAuthorization>::authorizing(request, key) {
                        Some(ins) => Some(ins),
                        None => None
                    }
                });

                match cache.as_ref() {
                    Some(cache) => ::rocket::Outcome::Success(cache),
                    None => ::rocket::Outcome::Forward(())
                }
            }
        }
    }
}
