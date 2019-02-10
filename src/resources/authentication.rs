use frank_jwt::{decode, validate_signature, Algorithm};
use std::env;

pub fn auth(jwt: String) -> Result<(), ()> {
    let mut keypath = env::current_dir().unwrap();
    keypath.push(
        env::var("iodine_public_key").expect("Environment variable iodine_public_key not set"),
    );

    match validate_signature(&jwt, &keypath, Algorithm::RS256) {
        Ok(success) => match success {
            true => match decode(&jwt, &keypath, Algorithm::RS256) {
                Ok((_, payload)) => match payload.get("id") {
                    Some(userid_value) => match payload.get("generation") {
                        Some(generation_value) => match userid_value.is_i64() {
                            true => match generation_value.is_string() {
                                true => {
                                    // TODO: check generation value
                                    Ok(())
                                }
                                false => Err(()),
                            },
                            false => Err(()),
                        },
                        None => Err(()),
                    },
                    None => Err(()),
                },
                Err(_) => Err(()),
            },
            false => Err(()),
        },
        Err(_) => Err(()),
    }
}
