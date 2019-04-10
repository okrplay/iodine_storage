// response enum for all routes, success enums sorted alphabetically first, error enums sorted alphabetically second
#[derive(Serialize)]
pub enum ResponseEnum {
    Success(Option<String>),
    InviteInvalid,
    InviteMissing,
    InternalError,
    JwtDecodeFailure,
    JwtInvalid,
    PasswordInvalid,
    UserInvalid,
}
