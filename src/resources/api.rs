use super::authentication::auth;

#[derive(Clone, Debug)]
pub struct APIResource;

#[derive(Response)]
#[web(status = "200")]
struct CustomSuccess {
    message: &'static str,
}

#[derive(Response)]
#[web(status = "400")]
struct CustomError {
    message: &'static str,
}

impl_web! {
    impl APIResource {
        #[get("/api")]
        fn api_index(&self) -> Result<&'static str, CustomError> {
            Ok("Welcome to the iodine_storage API!")
        }

        #[get("/authentication_check")]
        #[content_type("json")]
        fn authentication_check(&self, authentication: String) -> Result<CustomSuccess, CustomError> {
            match auth(authentication) {
                Ok(_) => Ok(CustomSuccess { message: "Authentication successful" }),
                Err(_) => Err(CustomError { message: "Error" }),
            }
        }
    }
}
