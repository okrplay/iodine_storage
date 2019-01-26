use frank_jwt::{validate_signature, Algorithm};
use std::env;

pub fn auth(jwt: String) -> Result<(), ()> {
    let mut keypath = env::current_dir().unwrap();
    keypath.push(
        env::var("iodine_public_key").expect("Environment variable iodine_public_key not set"),
    );

    match validate_signature(&jwt, &keypath, Algorithm::RS256) {
        Ok(success) => match success {
            true => {
                // Generation verification logic, to be implemented when the DB is added
                Ok(())
            }
            false => Err(()),
        },
        Err(_) => Err(()),
    }
}
