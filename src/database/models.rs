#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub generation: String,
}
