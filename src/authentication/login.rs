// imports
use super::super::database::database::establish_connection;
use bcrypt::verify;
use frank_jwt::{encode, Algorithm};
use std::env;

pub fn get_jwt(username: String, password: String) -> Result<String, &'static str> {
    let conn = establish_connection();
    let result = conn
        .find(json!({
            "selector": {
                "kind": "user",
                "username": username,
            },
        }))
        .unwrap();
    match result.total_rows {
        0 => Err("USER_INVALID"),
        _ => {
            let user_value = &result.get_data()[0];
            let hashed_password = user_value.get("password").unwrap().as_str().unwrap();
            match verify(password, hashed_password) {
                Ok(status) => match status {
                    true => {
                        let mut header = json!({});
                        let mut keypath = env::current_dir().unwrap();
                        keypath.push(
                            env::var("iodine_private_key")
                                .expect("Environment variable iodine_private_key not set"),
                        );
                        let mut payload = json!({
                            "username": user_value.get("username").unwrap().as_str().unwrap(),
                            "id": user_value.get("_id").unwrap().as_str().unwrap(),
                            "generation": user_value.get("generation").unwrap().as_str().unwrap(),
                        });
                        match encode(header, &keypath, &payload, Algorithm::RS256) {
                            Ok(jwt) => Ok(jwt),
                            Err(_) => Err("INTERNAL_ERROR"),
                        }
                    }
                    false => Err("PASSWORD_INVALID"),
                },
                Err(_) => Err("PASSWORD_INVALID"),
            }
        }
    }
}
