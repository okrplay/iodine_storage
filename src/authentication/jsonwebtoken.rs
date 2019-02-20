// imports
use frank_jwt::{decode, validate_signature, Algorithm};
use sofa::Database;
use std::env;

// jwt authentication function
pub fn auth(jwt: String, conn: Database) -> Result<&'static str, &'static str> {
    // get public key filesystem path from environment or .env
    let mut keypath = env::current_dir().unwrap();
    keypath.push(
        env::var("iodine_public_key").expect("Environment variable iodine_public_key not set"),
    );

    // validate jwt signature
    match validate_signature(&jwt, &keypath, Algorithm::RS256) {
        Ok(success) => {
            if success {
                // decode jwt to get payload
                match decode(&jwt, &keypath, Algorithm::RS256) {
                    // get id from jwt payload
                    Ok((_, payload)) => match payload.get("id") {
                        // get generation from jwt payload
                        Some(userid_value) => match payload.get("generation") {
                            // check if id is actually a string
                            Some(generation_value) => {
                                if userid_value.is_string() {
                                    // check if generation is actually a string
                                    if generation_value.is_string() {
                                        // search for a document with matching id and generation
                                        let result = conn.find(json!({
                                        "selector": {
                                            "category": "user",
                                            "_id": userid_value.as_str().unwrap().to_string(),
                                            "generation": generation_value.as_str().unwrap().to_string(),
                                        }
                                    })).unwrap();
                                        // check amount of results
                                        match result.total_rows {
                                            // no results, the generation value is invalid
                                            0 => Err("GENERATION_INVALID"),
                                            // 1 (or more, probably won't happen) result, generation and jwt is completely valid
                                            _ => Ok("AUTH_SUCCESS"),
                                        }
                                    } else {
                                        Err("GENERATION_INVALID")
                                    }
                                } else {
                                    Err("ID_INVALID")
                                }
                            }
                            None => Err("GENERATION_MISSING"),
                        },
                        None => Err("ID_MISSING"),
                    },
                    Err(_) => Err("JWT_DECODE_FAILED"),
                }
            } else {
                Err("SIGNATURE_INVALID")
            }
        }
        Err(_) => Err("SIGNATURE_INVALID"),
    }
}
