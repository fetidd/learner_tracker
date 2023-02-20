use proc_macro::*;
use quote::quote;
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
            let kinds = variants.iter().map(|f| &f.ident);
            quote! {
                #[derive(Clone, Debug, PartialEq)]
                pub struct Error {
                    pub kind: ErrorKind,
                    pub message: Option<String>
                }

                impl std::fmt::Display for Error {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match &self.message {
                            Some(msg) => write!(f, "[{}]::> {}", self.kind.as_string(), msg),
                            None => write!(f, "[{}]", self.kind.as_string()),
                        }
                    }
                }

                impl std::fmt::Display for ErrorKind {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.as_string())
                    }
                }

                #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
                struct ErrorResponse {
                    error: ErrorKind,
                    #[serde(skip_serializing_if="Option::is_none")]
                    details: Option<String>
                }
                impl #ident {
                    pub fn as_string(&self) -> String {
                        match self {
                            #(Self::#kinds => String::from(stringify!(#kinds))),*
                        }
                    }
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
