// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(uint32, tag="1")]
    pub request_id: u32,
    #[prost(oneof="request::Subsystem", tags="3, 4, 5")]
    pub subsystem: ::core::option::Option<request::Subsystem>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subsystem {
        #[prost(message, tag="3")]
        Core(super::super::core::Request),
        #[prost(message, tag="4")]
        Behaviors(super::super::behaviors::Request),
        #[prost(message, tag="5")]
        Keymap(super::super::keymap::Request),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<response::Type>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        RequestResponse(super::RequestResponse),
        #[prost(message, tag="2")]
        Notification(super::Notification),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestResponse {
    #[prost(uint32, tag="1")]
    pub request_id: u32,
    #[prost(oneof="request_response::Subsystem", tags="2, 3, 4, 5")]
    pub subsystem: ::core::option::Option<request_response::Subsystem>,
}
/// Nested message and enum types in `RequestResponse`.
pub mod request_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subsystem {
        #[prost(message, tag="2")]
        Meta(super::super::meta::Response),
        #[prost(message, tag="3")]
        Core(super::super::core::Response),
        #[prost(message, tag="4")]
        Behaviors(super::super::behaviors::Response),
        #[prost(message, tag="5")]
        Keymap(super::super::keymap::Response),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(oneof="notification::Subsystem", tags="2, 5")]
    pub subsystem: ::core::option::Option<notification::Subsystem>,
}
/// Nested message and enum types in `Notification`.
pub mod notification {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subsystem {
        #[prost(message, tag="2")]
        Core(super::super::core::Notification),
        #[prost(message, tag="5")]
        Keymap(super::super::keymap::Notification),
    }
}
// @@protoc_insertion_point(module)
