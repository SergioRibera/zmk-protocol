// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::ResponseType", tags="1, 2")]
    pub response_type: ::core::option::Option<response::ResponseType>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        #[prost(bool, tag="1")]
        NoResponse(bool),
        #[prost(enumeration="super::ErrorConditions", tag="2")]
        SimpleError(i32),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorConditions {
    Generic = 0,
    UnlockRequired = 1,
    RpcNotFound = 2,
    MsgDecodeFailed = 3,
    MsgEncodeFailed = 4,
}
impl ErrorConditions {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorConditions::Generic => "GENERIC",
            ErrorConditions::UnlockRequired => "UNLOCK_REQUIRED",
            ErrorConditions::RpcNotFound => "RPC_NOT_FOUND",
            ErrorConditions::MsgDecodeFailed => "MSG_DECODE_FAILED",
            ErrorConditions::MsgEncodeFailed => "MSG_ENCODE_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GENERIC" => Some(Self::Generic),
            "UNLOCK_REQUIRED" => Some(Self::UnlockRequired),
            "RPC_NOT_FOUND" => Some(Self::RpcNotFound),
            "MSG_DECODE_FAILED" => Some(Self::MsgDecodeFailed),
            "MSG_ENCODE_FAILED" => Some(Self::MsgEncodeFailed),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
