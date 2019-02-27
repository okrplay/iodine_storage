// imports
use super::super::responses::enums::ResponseEnum::{self, *};
use bcrypt::verify;
use frank_jwt::{encode, Algorithm};
use sofa::Database;
use std::env;

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
                Ok(status) => {
                    // verify if password is correct
                    if status {
                        // generate jwt and return it
                        let header = json!({});
                        let mut keypath = env::current_dir().unwrap();
                        keypath.push(
                            env::var("iodine_private_key")
                                .expect("Environment variable iodine_private_key not set"),
                        );
                        let payload = json!({
                            "username": user_value.get("username").unwrap().as_str().unwrap(),
                            "id": user_value.get("_id").unwrap().as_str().unwrap(),
                            "generation": user_value.get("generation").unwrap().as_str().unwrap(),
                            "group": user_value.get("group").unwrap().as_str().unwrap(),
                        });
                        match encode(header, &keypath, &payload, Algorithm::RS256) {
                            Ok(jwt) => Ok(Success(Some(jwt))),
                            Err(_) => Err(InternalError),
                        }
                    } else {
                        Err(PasswordInvalid)
                    }
                }
                Err(_) => Err(PasswordInvalid),
            }
        }
    }
}
