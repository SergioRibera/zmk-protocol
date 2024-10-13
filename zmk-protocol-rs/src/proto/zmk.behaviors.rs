// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::RequestType", tags="1, 2")]
    pub request_type: ::core::option::Option<request::RequestType>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        #[prost(bool, tag="1")]
        ListAllBehaviors(bool),
        #[prost(message, tag="2")]
        GetBehaviorDetails(super::GetBehaviorDetailsRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBehaviorDetailsRequest {
    #[prost(uint32, tag="1")]
    pub behavior_id: u32,
}
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
        #[prost(message, tag="1")]
        ListAllBehaviors(super::ListAllBehaviorsResponse),
        #[prost(message, tag="2")]
        GetBehaviorDetails(super::GetBehaviorDetailsResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllBehaviorsResponse {
    #[prost(uint32, repeated, tag="1")]
    pub behaviors: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBehaviorDetailsResponse {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub metadata: ::prost::alloc::vec::Vec<BehaviorBindingParametersSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorBindingParametersSet {
    #[prost(message, repeated, tag="1")]
    pub param1: ::prost::alloc::vec::Vec<BehaviorParameterValueDescription>,
    #[prost(message, repeated, tag="2")]
    pub param2: ::prost::alloc::vec::Vec<BehaviorParameterValueDescription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorParameterValueDescriptionRange {
    #[prost(int32, tag="1")]
    pub min: i32,
    #[prost(int32, tag="2")]
    pub max: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorParameterNil {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorParameterLayerId {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorParameterHidUsage {
    #[prost(uint32, tag="1")]
    pub keyboard_max: u32,
    #[prost(uint32, tag="2")]
    pub consumer_max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorParameterValueDescription {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="behavior_parameter_value_description::ValueType", tags="2, 3, 4, 5, 6")]
    pub value_type: ::core::option::Option<behavior_parameter_value_description::ValueType>,
}
/// Nested message and enum types in `BehaviorParameterValueDescription`.
pub mod behavior_parameter_value_description {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueType {
        #[prost(message, tag="2")]
        Nil(super::BehaviorParameterNil),
        #[prost(uint32, tag="3")]
        Constant(u32),
        #[prost(message, tag="4")]
        Range(super::BehaviorParameterValueDescriptionRange),
        #[prost(message, tag="5")]
        HidUsage(super::BehaviorParameterHidUsage),
        #[prost(message, tag="6")]
        LayerId(super::BehaviorParameterLayerId),
    }
}
// @@protoc_insertion_point(module)
