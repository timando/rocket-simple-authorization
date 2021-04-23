/// To let a struct which implements the `SimpleAuthorization` trait become an authorizer.
#[macro_export]
macro_rules! authorizer {
    ($name:ty) => {
        impl<'a, 'r> $crate::rocket::request::FromRequest<'a, 'r> for $name {
            type Error = ();

            fn from_request(
                request: &'a $crate::rocket::request::Request<'r>,
            ) -> $crate::rocket::request::Outcome<Self, Self::Error> {
                let key: Option<&str> = request.headers().get("authorization").next();

                match <$name as $crate::SimpleAuthorization>::authorizing(request, key) {
                    Some(ins) => $crate::rocket::Outcome::Success(ins),
                    None => $crate::rocket::Outcome::Forward(()),
                }
            }
        }
    };
    (ref $name:ty) => {
        $crate::authorizer!($name);

        impl<'a, 'r> $crate::rocket::request::FromRequest<'a, 'r> for &'a $name {
            type Error = ();

            fn from_request(
                request: &'a $crate::rocket::request::Request<'r>,
            ) -> $crate::rocket::request::Outcome<Self, Self::Error> {
                let cache = request.local_cache(|| {
                    let key: Option<&str> = request.headers().get("authorization").next();

                    match <$name as $crate::SimpleAuthorization>::authorizing(request, key) {
                        Some(ins) => Some(ins),
                        None => None,
                    }
                });

                match cache.as_ref() {
                    Some(cache) => $crate::rocket::Outcome::Success(cache),
                    None => $crate::rocket::Outcome::Forward(()),
                }
            }
        }
    };
}
