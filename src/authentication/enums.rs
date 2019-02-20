use std::fmt;

// response enum for the auth function (String after Success is for the user's group)
#[derive(Debug)]
pub enum AuthReponse {
    Success(String),
    GenerationInvalid,
    GenerationMissing,
    IdInvalid,
    IdMissing,
    JwtDecodeFailure,
    SignatureInvalid,
}

// implement format for the AuthResponse enum
impl fmt::Display for AuthReponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
