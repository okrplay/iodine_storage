use super::authentication::foo;

#[derive(Clone, Debug)]
pub struct APIResource;

impl_web! {
    impl APIResource {
        #[get("/api")]
        fn api_index(&self) -> Result<&'static str, ()> {
            foo();
            Ok("Welcome to the iodine_storage API!")
        }
    }
}
