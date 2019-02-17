use super::super::database::database::establish_connection;
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
                        Some(generation_value) => match userid_value.is_string() {
                            true => match generation_value.is_string() {
                                true => {
                                    let conn = establish_connection();
                                    let result = conn.find(json!({
                                        "selector": {
                                            "_id": userid_value.as_str().unwrap().to_string(),
                                            "generation": generation_value.as_str().unwrap().to_string(),
                                        }
                                    })).unwrap();
                                    match result.total_rows {
                                        0 => Err(()),
                                        _ => Ok(()),
                                    }
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
