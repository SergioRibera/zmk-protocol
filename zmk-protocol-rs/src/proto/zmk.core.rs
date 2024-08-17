// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::RequestType", tags="1, 2, 3, 4")]
    pub request_type: ::core::option::Option<request::RequestType>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        #[prost(bool, tag="1")]
        GetDeviceInfo(bool),
        #[prost(bool, tag="2")]
        GetLockState(bool),
        #[prost(bool, tag="3")]
        Lock(bool),
        #[prost(bool, tag="4")]
        ResetSettings(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::ResponseType", tags="1, 2, 4")]
    pub response_type: ::core::option::Option<response::ResponseType>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        #[prost(message, tag="1")]
        GetDeviceInfo(super::GetDeviceInfoResponse),
        #[prost(enumeration="super::LockState", tag="2")]
        GetLockState(i32),
        #[prost(bool, tag="4")]
        ResetSettings(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeviceInfoResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub serial_number: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(oneof="notification::NotificationType", tags="1")]
    pub notification_type: ::core::option::Option<notification::NotificationType>,
}
/// Nested message and enum types in `Notification`.
pub mod notification {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NotificationType {
        #[prost(enumeration="super::LockState", tag="1")]
        LockStateChanged(i32),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockState {
    ZmkStudioCoreLockStateLocked = 0,
    ZmkStudioCoreLockStateUnlocked = 1,
}
impl LockState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LockState::ZmkStudioCoreLockStateLocked => "ZMK_STUDIO_CORE_LOCK_STATE_LOCKED",
            LockState::ZmkStudioCoreLockStateUnlocked => "ZMK_STUDIO_CORE_LOCK_STATE_UNLOCKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ZMK_STUDIO_CORE_LOCK_STATE_LOCKED" => Some(Self::ZmkStudioCoreLockStateLocked),
            "ZMK_STUDIO_CORE_LOCK_STATE_UNLOCKED" => Some(Self::ZmkStudioCoreLockStateUnlocked),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
