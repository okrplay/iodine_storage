// imports
use super::super::responses::enums::ResponseEnum::{self, *};
use bcrypt::verify;
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, RegisteredClaims, JWT,
};
use sofa::Database;

// define jwt private claims
#[derive(Serialize, Deserialize)]
struct PrivateClaims {
    username: String,
    id: String,
    generation: String,
    group: String,
}

// jwt generation function
pub fn login(
    username: String,
    password: String,
    conn: Database,
) -> Result<ResponseEnum, ResponseEnum> {
    // search for user in database
    let result = conn
        .find(json!({
            "selector": {
                "category": "user",
                "username": username,
            },
        }))
        .unwrap();

    // check if user was found
    match result.total_rows {
        // no user found
        0 => Err(UserInvalid),

        // user found
        _ => {
            // get password from result
            let user_value = &result.get_data()[0];
            let hashed_password = user_value.get("password").unwrap().as_str().unwrap();

            // verify password
            match verify(password, hashed_password) {
                // password verification successful
                Ok(status) => {
                    // verify if password is correct
                    if status {
                        // password is correct, proceed
                        // get the keys
                        let secret_key = Secret::rsa_keypair_from_file("keys/private_key.der")
                            .expect("Couldn't get private key from file");

                        // set the jwt claims
                        let claims = ClaimsSet::<PrivateClaims> {
                            registered: RegisteredClaims {
                                ..Default::default()
                            },
                            private: PrivateClaims {
                                username: user_value
                                    .get("username")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                                id: user_value.get("_id").unwrap().as_str().unwrap().to_string(),
                                generation: user_value
                                    .get("generation")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                                group: user_value
                                    .get("group")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                            },
                        };

                        // generate jwt return it
                        match JWT::new_decoded(
                            From::from(RegisteredHeader {
                                algorithm: SignatureAlgorithm::RS256,
                                ..Default::default()
                            }),
                            claims,
                        )
                        .into_encoded(&secret_key)
                        {
                            Ok(jwt) => Ok(Success(Some(jwt.unwrap_encoded().to_string()))),
                            Err(_) => Err(InternalError),
                        }
                    } else {
                        // password incorrect
                        Err(PasswordInvalid)
                    }
                }

                // password verification not successful
                Err(_) => Err(PasswordInvalid),
            }
        }
    }
}
