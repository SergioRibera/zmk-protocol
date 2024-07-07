// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::RequestType", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub request_type: ::core::option::Option<request::RequestType>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        #[prost(bool, tag="1")]
        GetKeymap(bool),
        #[prost(message, tag="2")]
        SetLayerBinding(super::SetLayerBindingRequest),
        #[prost(bool, tag="3")]
        CheckUnsavedChanges(bool),
        #[prost(bool, tag="4")]
        SaveChanges(bool),
        #[prost(bool, tag="5")]
        DiscardChanges(bool),
        #[prost(bool, tag="6")]
        GetPhysicalLayouts(bool),
        #[prost(uint32, tag="7")]
        SetActivePhysicalLayout(u32),
        #[prost(message, tag="8")]
        MoveLayer(super::MoveLayerRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::ResponseType", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub response_type: ::core::option::Option<response::ResponseType>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        #[prost(message, tag="1")]
        GetKeymap(super::Keymap),
        #[prost(enumeration="super::SetLayerBindingResponse", tag="2")]
        SetLayerBinding(i32),
        #[prost(bool, tag="3")]
        CheckUnsavedChanges(bool),
        #[prost(bool, tag="4")]
        SaveChanges(bool),
        #[prost(bool, tag="5")]
        DiscardChanges(bool),
        #[prost(message, tag="6")]
        GetPhysicalLayouts(super::PhysicalLayouts),
        #[prost(message, tag="7")]
        SetActivePhysicalLayout(super::SetActivePhysicalLayoutResponse),
        #[prost(message, tag="8")]
        MoveLayer(super::MoveLayerResponse),
    }
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
        #[prost(bool, tag="1")]
        UnsavedChangesStatusChanged(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActivePhysicalLayoutResponse {
    #[prost(oneof="set_active_physical_layout_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<set_active_physical_layout_response::Result>,
}
/// Nested message and enum types in `SetActivePhysicalLayoutResponse`.
pub mod set_active_physical_layout_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="1")]
        Ok(super::Keymap),
        #[prost(enumeration="super::SetActivePhysicalLayoutErrorCode", tag="2")]
        Err(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveLayerResponse {
    #[prost(oneof="move_layer_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<move_layer_response::Result>,
}
/// Nested message and enum types in `MoveLayerResponse`.
pub mod move_layer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="1")]
        Ok(super::Keymap),
        #[prost(enumeration="super::MoveLayerErrorCode", tag="2")]
        Err(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLayerBindingRequest {
    #[prost(uint32, tag="1")]
    pub layer_id: u32,
    #[prost(int32, tag="2")]
    pub key_position: i32,
    #[prost(message, optional, tag="3")]
    pub binding: ::core::option::Option<BehaviorBinding>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveLayerRequest {
    #[prost(uint32, tag="1")]
    pub start_index: u32,
    #[prost(uint32, tag="2")]
    pub dest_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keymap {
    #[prost(message, repeated, tag="1")]
    pub layers: ::prost::alloc::vec::Vec<Layer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Layer {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub bindings: ::prost::alloc::vec::Vec<BehaviorBinding>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BehaviorBinding {
    #[prost(sint32, tag="1")]
    pub behavior_id: i32,
    #[prost(uint32, tag="2")]
    pub param1: u32,
    #[prost(uint32, tag="3")]
    pub param2: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalLayouts {
    #[prost(uint32, tag="1")]
    pub active_layout_index: u32,
    #[prost(message, repeated, tag="2")]
    pub layouts: ::prost::alloc::vec::Vec<PhysicalLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalLayout {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<KeyPhysicalAttrs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPhysicalAttrs {
    #[prost(sint32, tag="1")]
    pub width: i32,
    #[prost(sint32, tag="2")]
    pub height: i32,
    #[prost(sint32, tag="3")]
    pub x: i32,
    #[prost(sint32, tag="4")]
    pub y: i32,
    #[prost(sint32, tag="5")]
    pub r: i32,
    #[prost(sint32, tag="6")]
    pub rx: i32,
    #[prost(sint32, tag="7")]
    pub ry: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetLayerBindingResponse {
    Success = 0,
    InvalidLocation = 1,
    InvalidBehavior = 2,
    InvalidParameters = 3,
}
impl SetLayerBindingResponse {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetLayerBindingResponse::Success => "SUCCESS",
            SetLayerBindingResponse::InvalidLocation => "INVALID_LOCATION",
            SetLayerBindingResponse::InvalidBehavior => "INVALID_BEHAVIOR",
            SetLayerBindingResponse::InvalidParameters => "INVALID_PARAMETERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "INVALID_LOCATION" => Some(Self::InvalidLocation),
            "INVALID_BEHAVIOR" => Some(Self::InvalidBehavior),
            "INVALID_PARAMETERS" => Some(Self::InvalidParameters),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoveLayerErrorCode {
    MoveLayerErrSuccess = 0,
    MoveLayerErrGeneric = 1,
    MoveLayerErrInvalidLayer = 2,
    MoveLayerErrInvalidDestination = 3,
}
impl MoveLayerErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MoveLayerErrorCode::MoveLayerErrSuccess => "MOVE_LAYER_ERR_SUCCESS",
            MoveLayerErrorCode::MoveLayerErrGeneric => "MOVE_LAYER_ERR_GENERIC",
            MoveLayerErrorCode::MoveLayerErrInvalidLayer => "MOVE_LAYER_ERR_INVALID_LAYER",
            MoveLayerErrorCode::MoveLayerErrInvalidDestination => "MOVE_LAYER_ERR_INVALID_DESTINATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MOVE_LAYER_ERR_SUCCESS" => Some(Self::MoveLayerErrSuccess),
            "MOVE_LAYER_ERR_GENERIC" => Some(Self::MoveLayerErrGeneric),
            "MOVE_LAYER_ERR_INVALID_LAYER" => Some(Self::MoveLayerErrInvalidLayer),
            "MOVE_LAYER_ERR_INVALID_DESTINATION" => Some(Self::MoveLayerErrInvalidDestination),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetActivePhysicalLayoutErrorCode {
    Generic = 0,
    InvalidLayoutIndex = 1,
}
impl SetActivePhysicalLayoutErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetActivePhysicalLayoutErrorCode::Generic => "GENERIC",
            SetActivePhysicalLayoutErrorCode::InvalidLayoutIndex => "INVALID_LAYOUT_INDEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GENERIC" => Some(Self::Generic),
            "INVALID_LAYOUT_INDEX" => Some(Self::InvalidLayoutIndex),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
