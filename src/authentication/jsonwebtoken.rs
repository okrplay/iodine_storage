// imports
use super::super::responses::enums::ResponseEnum::{self, *};
use biscuit::{jwa::SignatureAlgorithm, jws::Secret, ClaimsSet, JWT};
use sofa::Database;

// define jwt private claims
#[derive(Serialize, Deserialize, Debug)]
struct PrivateClaims {
    username: String,
    id: String,
    generation: String,
    group: String,
}

// jwt authentication function
pub fn auth(jwt: String, conn: Database) -> Result<ResponseEnum, ResponseEnum> {
    // get the keys
    let secret_key = Secret::public_key_from_file("keys/public_key.der")
        .expect("Couldn't get public key from file");

    // get the decoded token
    // TODO: verify jwt claims
    let token_encoded = JWT::<ClaimsSet<PrivateClaims>, biscuit::Empty>::new_encoded(&jwt);
    match token_encoded.into_decoded(&secret_key, SignatureAlgorithm::RS256) {
        Ok(jwt) => {
            let claims = jwt.payload().unwrap();
            // search for a document with matching id and generation
            let result = conn
                .find(json!({
                    "selector": {
                        "category": "user",
                        "_id": claims.private.private.id,
                        "generation": claims.private.private.generation,
                    }
                }))
                .unwrap();
            // check amount of results
            match result.total_rows {
                // no results, the generation value is invalid
                0 => Err(JwtInvalid),
                // 1 (or more, probably won't happen) result, generation and jwt is completely valid
                _ => Ok(Success(None)),
            }
        }
        Err(_) => Err(JwtDecodeFailure),
    }
}
