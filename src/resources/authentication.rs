use super::super::database::database::establish_connection;
use super::super::database::models::*;
use diesel::prelude::*;
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
                                    use super::super::database::schema::users::dsl::*;

                                    let conn = establish_connection();
                                    let results = users
                                        .filter(userid.eq(userid_value.as_i64().unwrap()))
                                        .limit(1)
                                        .load::<User>(&conn)
                                        .expect("Error loading users");

                                    match results.len() {
                                        1 => {
                                            if results[0].generation
                                                == generation_value.as_str().unwrap()
                                            {
                                                Ok(())
                                            } else {
                                                Err(())
                                            }
                                        }
                                        _ => Err(()),
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
