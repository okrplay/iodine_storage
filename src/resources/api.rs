// imports
use super::super::authentication::authentication::auth;

// resource struct
#[derive(Clone, Debug)]
pub struct APIResource;

// custom response for success and failure
#[derive(Response)]
struct CustomResponse {
    message: &'static str,
    #[web(status)]
    status: u16,
}

// macro to implement resources
impl_web! {
    impl APIResource {
        // api index
        #[get("/api")]
        #[content_type("json")]
        fn api_index(&self) -> Result<CustomResponse, ()> {
            Ok(CustomResponse { message: "Welcome to the iodine_storage API!", status: 200 })
        }

        // endpoint to check if authentication with jwt is working
        #[get("/api/authentication_check")]
        #[content_type("json")]
        fn authentication_check(&self, authentication: String) -> Result<CustomResponse, ()> {
            match auth(authentication) {
                Ok(msg) => Ok(CustomResponse { message: msg, status: 200 }),
                Err(msg) => Ok(CustomResponse { message: msg, status: 400 }),
            }
        }
    }
}
