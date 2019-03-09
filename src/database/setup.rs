// imports
use sofa::Database;

// check setup function
pub fn check_setup(conn: Database) -> bool {
    // checks if there are settings available in the database
    let result = conn
        .find(json!({
            "selector": {
                "category":"system_setting"
            }
        }))
        .unwrap();
    match result.total_rows {
        0 => false,
        _ => true,
    }
}
