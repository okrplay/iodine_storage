use super::authentication::auth;

#[derive(Clone, Debug)]
pub struct APIResource;

#[derive(Response)]
struct CustomResponse {
    message: &'static str,
    #[web(status)]
    status: u16,
}

impl_web! {
    impl APIResource {
        #[get("/api")]
        #[content_type("json")]
        fn api_index(&self) -> Result<CustomResponse, ()> {
            Ok(CustomResponse { message: "Welcome to the iodine_storage API!", status: 200 })
        }

        #[get("/api/authentication_check")]
        #[content_type("json")]
        fn authentication_check(&self, authentication: String) -> Result<CustomResponse, ()> {
            match auth(authentication) {
                Ok(_) => Ok(CustomResponse { message: "Authentication successful", status: 200 }),
                Err(_) => Ok(CustomResponse { message: "Error", status: 400 }),
            }
        }
    }
}
