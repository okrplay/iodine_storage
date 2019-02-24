// imports
use super::super::responses::enums::ResponseEnum::{self, *};
use sofa::Database;

// check if invite code is valid  and delete it function
pub fn check(invite: String, conn: Database) -> Result<ResponseEnum, ResponseEnum> {
    // search for invite in database
    let result = conn
        .find(json!({
            "selector": {
                "category": "invite",
                "code": invite,
            }
        }))
        .unwrap();
    // check if invite was found
    match result.total_rows {
        // invite not found
        0 => Err(InviteInvalid),
        // one (or more) invite(s) found
        _ => {
            // get invite and delete it
            let found_invite = &result.rows[0];
            if conn.remove(found_invite.to_owned().doc) {
                Ok(Success(None))
            } else {
                Err(InternalError)
            }
        }
    }
}
