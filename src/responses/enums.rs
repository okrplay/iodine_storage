// response enum for all routes, success enums sorted alphabetically first, error enums sorted alphabetically second
#[derive(Serialize)]
pub enum ResponseEnum {
    Success(Option<String>),
    GenerationInvalid,
    GenerationMissing,
    IdInvalid,
    IdMissing,
    InviteInvalid,
    InternalError,
    JwtDecodeFailure,
    PasswordInvalid,
    SignatureInvalid,
    UserInvalid,
}
