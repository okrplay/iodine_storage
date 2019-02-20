use sofa::Database;

pub fn check_setup(conn: Database) -> bool {
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
