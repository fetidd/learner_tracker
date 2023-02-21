use lazy_static::lazy_static;
use proc_macro::*;
use quote::quote;
use regex::Regex;
use syn::{self, parse_macro_input, DataEnum, DeriveInput};

#[proc_macro_derive(KindError, attributes(code, from))]
pub fn derive_kind_error(_in: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(_in);
    assert_eq!(
        ident.to_string(),
        "ErrorKind",
        "must be an enum called ErrorKind"
    );
    match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let strings = variants
                .iter()
                .map(|f| stringify_errorkind(&f.ident.to_string()));
            let kinds = variants.iter().map(|f| &f.ident);
            // let attrs = variants.iter().map(|f| &f.attrs);
            quote! {
                #[derive(Clone, Debug, PartialEq)]
                pub struct Error {
                    pub kind: ErrorKind,
                    pub message: Option<String>
                }

                impl std::fmt::Display for Error {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match &self.message {
                            Some(msg) => write!(f, "{}: {}", self.kind, msg),
                            None => write!(f, "{}", self.kind),
                        }
                    }
                }

                impl std::fmt::Display for ErrorKind {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let string = match self {
                            #(Self::#kinds => String::from(#strings)),*
                        };
                        write!(f, "{}", string)
                    }
                }

                #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
                struct ErrorResponse {
                    error: String,
                    #[serde(skip_serializing_if="Option::is_none")]
                    details: Option<String>
                }
            }
            .into()
        }
        _ => panic!("can only be derived on an enum called ErrorKind!"),
    }
}

// impl IntoResponse for Error {
//     fn into_response(self) -> Response {
//         let code = match self.kind {
//             ErrorKind::InvalidApiRequest
//             | ErrorKind::InvalidUserPassword
//             | ErrorKind::UserDoesNotExist
//             | ErrorKind::PupilDoesNotExist => StatusCode::BAD_REQUEST,
//             ErrorKind::MissingEnvVariable
//             | ErrorKind::AddrParseError
//             | ErrorKind::IoError
//             | ErrorKind::ParseIntError
//             | ErrorKind::DatabaseError
//             | ErrorKind::JWTTokenCreationError
//             | ErrorKind::SerializeError
//             | ErrorKind::DeserializeError
//             | ErrorKind::EncodeError
//             | ErrorKind::DecodeError
//             | ErrorKind::ParseError
//             | ErrorKind::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
//             ErrorKind::Unauthorised | ErrorKind::InvalidJwt => StatusCode::UNAUTHORIZED,
//         };
//         (
//             code,
//             Json(ErrorResponse {
//                 error: self.kind,
//                 details: self.message,
//             }),
//         )
//             .into_response()
//     }
// }

fn stringify_errorkind(var: &str) -> String {
    lazy_static! {
        static ref KIND_PATTERN: &'static str = r"[A-Z][a-z]+";
    }
    let re = Regex::new(&KIND_PATTERN).expect("KIND_PATTERN invalid");
    let caps = re.captures_iter(var);
    caps.map(|c| c[0].to_uppercase())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify_errorkind() {
        let tests: Vec<(&str, &str)> = vec![
            ("Error", "ERROR"),
            ("InvalidCredentials", "INVALID CREDENTIALS"),
        ];
        for (input, exp) in tests {
            assert_eq!(exp.to_string(), stringify_errorkind(input));
        }
    }
}
