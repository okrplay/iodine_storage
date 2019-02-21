// response enum for all routes, success enums sorted alphabetically first, error enums sorted alphabetically second
#[derive(Serialize)]
pub enum ResponseEnum {
    Success(String),
    GenerationInvalid,
    GenerationMissing,
    IdInvalid,
    IdMissing,
    InternalError,
    JwtDecodeFailure,
    PasswordInvalid,
    SignatureInvalid,
    UserInvalid,
}
