#[derive(Queryable)]
pub struct User {
    pub userid: i64,
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub generation: String,
}
