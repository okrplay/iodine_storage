// imports
use super::super::responses::enums::ResponseEnum::{self, *};
use super::invites::{check, enabled};
use bcrypt::{hash, DEFAULT_COST};
use sofa::Database;

// user registration function
pub fn register(
    username: String,
    password: String,
    invite_option: Option<String>,
    conn: Database,
) -> Result<ResponseEnum, ResponseEnum> {
    if enabled(conn.clone()) {
        // with invite
        match invite_option {
            Some(invite) => {
                match check(invite, conn.clone()) {
                    // create user account
                    Ok(_) => match create_account(username, password, conn) {
                        Ok(_) => Ok(Success(None)),
                        Err(why) => Err(why),
                    },
                    Err(why) => Err(why),
                }
            }
            None => Err(InviteMissing),
        }
    } else {
        match create_account(username, password, conn) {
            Ok(_) => Ok(Success(None)),
            Err(why) => Err(why),
        }
    }
}

// user account creation function
fn create_account(
    username: String,
    password: String,
    conn: Database,
) -> Result<ResponseEnum, ResponseEnum> {
    // hash the password
    match hash(password, DEFAULT_COST) {
        Ok(hashed_password) => {
            // create the user
            match conn.create(json!({
                "category": "user",
                "username": username,
                "password": hashed_password,
                "generation": "XXXXXX", // todo random generation
                "group": "user",
            })) {
                Ok(_) => Ok(Success(None)),
                Err(_) => Err(InternalError),
            }
        }
        Err(_) => Err(InternalError),
    }
}
