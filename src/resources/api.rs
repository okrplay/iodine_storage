// imports
use super::super::authentication::{authentication::auth, login::get_jwt};
use serde_json::Value;

// resource struct
#[derive(Clone, Debug)]
pub struct APIResource;

// custom response for success and failure
#[derive(Response)]
struct CustomResponse {
    message: String,
    #[web(status)]
    status: u16,
}

#[derive(Extract)]
struct LoginUser {
    username: String,
    password: String,
}

// macro to implement resources
impl_web! {
    impl APIResource {
        // api index
        #[get("/api")]
        #[content_type("json")]
        fn api_index(&self) -> Result<CustomResponse, ()> {
            Ok(CustomResponse { message: "Welcome to the iodine_storage API!".to_string(), status: 200 })
        }

        // endpoint to check if authentication with jwt is working
        #[get("/api/authentication_check")]
        #[content_type("json")]
        fn authentication_check(&self, authentication: String) -> Result<CustomResponse, ()> {
            match auth(authentication) {
                Ok(msg) => Ok(CustomResponse { message: msg.to_string(), status: 200 }),
                Err(msg) => Ok(CustomResponse { message: msg.to_string(), status: 400 }),
            }
        }

        #[post("/api/login")]
        #[content_type("json")]
        fn login(&self, body: LoginUser) -> Result<CustomResponse, ()> {
            match get_jwt(body.username, body.password) {
                Ok(jwt) => Ok(CustomResponse { message: jwt, status: 200 }),
                Err(msg) => Ok(CustomResponse { message: msg.to_string(), status: 400 }),
            }
        }
    }
}
