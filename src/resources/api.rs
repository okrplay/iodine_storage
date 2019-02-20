// imports
use super::super::authentication::{jsonwebtoken::auth, login::get_jwt};
use super::super::database::connection::establish_connection;

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

// struct used in login route
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
            let conn = establish_connection();
            match auth(authentication, conn) {
                Ok(msg) => Ok(CustomResponse { message: msg.to_string(), status: 200 }),
                Err(msg) => Ok(CustomResponse { message: msg.to_string(), status: 400 }),
            }
        }

        // login endpoint
        #[post("/api/login")]
        #[content_type("json")]
        fn login(&self, body: LoginUser) -> Result<CustomResponse, ()> {
            let conn = establish_connection();
            match get_jwt(body.username, body.password, conn) {
                Ok(jwt) => Ok(CustomResponse { message: jwt, status: 200 }),
                Err(msg) => Ok(CustomResponse { message: msg.to_string(), status: 400 }),
            }
        }
    }
}
