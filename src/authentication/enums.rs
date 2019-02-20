// response enum for the auth function (String after Success is for the user's group)
#[derive(Serialize)]
pub enum AuthResponseEnum {
    Success(String),
    GenerationInvalid,
    GenerationMissing,
    IdInvalid,
    IdMissing,
    JwtDecodeFailure,
    SignatureInvalid,
}
