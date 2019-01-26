use super::authentication::auth;

#[derive(Clone, Debug)]
pub struct APIResource;

impl_web! {
    impl APIResource {
        #[get("/api")]
        fn api_index(&self) -> Result<&'static str, ()> {
            Ok("Welcome to the iodine_storage API!")
        }

        #[get("/authentication_check")]
        fn authentication_check(&self, authentication: String) -> Result<&'static str, ()> {
            match auth(authentication) {
                Ok(_) => Ok("Authentication successful"),
                Err(_) => Err(()),
            }
        }
    }
}
