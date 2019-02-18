// imports
use super::super::database::database::establish_connection;
use frank_jwt::{decode, validate_signature, Algorithm};
use std::env;

// jwt authentication function
pub fn auth(jwt: String) -> Result<&'static str, &'static str> {
    // get public key filesystem path from environment or .env
    let mut keypath = env::current_dir().unwrap();
    keypath.push(
        env::var("iodine_public_key").expect("Environment variable iodine_public_key not set"),
    );

    // validate jwt signature
    match validate_signature(&jwt, &keypath, Algorithm::RS256) {
        Ok(success) => match success {
            // decode jwt to get payload
            true => match decode(&jwt, &keypath, Algorithm::RS256) {
                // get id from jwt payload
                Ok((_, payload)) => match payload.get("id") {
                    // get generation from jwt payload
                    Some(userid_value) => match payload.get("generation") {
                        // check if id is actually a string
                        Some(generation_value) => match userid_value.is_string() {
                            // check if generation is actually a string
                            true => match generation_value.is_string() {
                                true => {
                                    // get couchdb connection + database
                                    let conn = establish_connection();
                                    // search for a document with matching id and generation
                                    let result = conn.find(json!({
                                        "selector": {
                                            "kind": "user",
                                            "_id": userid_value.as_str().unwrap().to_string(),
                                            "generation": generation_value.as_str().unwrap().to_string(),
                                        }
                                    })).unwrap();
                                    // check amount of results
                                    match result.total_rows {
                                        // no results, the generation value is invalid
                                        0 => Err("GENERATION_INVALID"),
                                        // 1 (or more, probably won't happen) result, generation and jwt is completly valid
                                        _ => Ok("AUTH_SUCCESS"),
                                    }
                                }
                                false => Err("GENERATION_INVALID"),
                            },
                            false => Err("ID_INVALID"),
                        },
                        None => Err("GENERATION_MISSING"),
                    },
                    None => Err("ID_MISSING"),
                },
                Err(_) => Err("JWT_DECODE_FAILED"),
            },
            false => Err("SIGNATURE_INVALID"),
        },
        Err(_) => Err("SIGNATURE_INVALID"),
    }
}
