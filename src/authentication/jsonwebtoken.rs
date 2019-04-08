// imports
use super::super::responses::enums::ResponseEnum::{self, *};
use biscuit::{jwa::SignatureAlgorithm, jws::Secret, ClaimsSet, JWT};
use sofa::Database;
use std::env;

// define jwt private claims
#[derive(Serialize, Deserialize)]
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
        Ok(jwt) => Ok(Success(None)),
        Err(_) => Err(InternalError),
    }
}
