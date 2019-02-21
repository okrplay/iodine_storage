// imports
use super::super::authentication::{jsonwebtoken::auth, login::get_jwt};
use super::super::database::connection::establish_connection;
use super::super::responses::enums::ResponseEnum;

// resource struct
#[derive(Clone, Debug)]
pub struct APIResource;

// generic response for success and failure
#[derive(Response)]
struct CustomResponse {
    message: String,
    #[web(status)]
    status: u16,
}

// generic response for routes with enums
#[derive(Response)]
struct EnumResponse {
    code: ResponseEnum,
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
        #[get("/api/auth")]
        #[content_type("json")]
        fn authentication_check(&self, authentication: String) -> Result<EnumResponse, ()> {
            let conn = establish_connection();
            match auth(authentication, conn) {
                Ok(response) => Ok(EnumResponse { code: response, status: 200 }),
                Err(response) => Ok(EnumResponse { code: response, status: 400 }),
            }
        }

        // login endpoint
        #[post("/api/login")]
        #[content_type("json")]
        fn login(&self, body: LoginUser) -> Result<EnumResponse, ()> {
            let conn = establish_connection();
            match get_jwt(body.username, body.password, conn) {
                Ok(response) => Ok(EnumResponse { code: response, status: 200 }),
                Err(response) => Ok(EnumResponse { code: response, status: 400 }),
            }
        }
    }
}
