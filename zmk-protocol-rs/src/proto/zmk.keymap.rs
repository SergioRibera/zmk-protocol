// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::RequestType", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
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
        #[prost(message, tag="9")]
        AddLayer(super::AddLayerRequest),
        #[prost(message, tag="10")]
        RemoveLayer(super::RemoveLayerRequest),
        #[prost(message, tag="11")]
        RestoreLayer(super::RestoreLayerRequest),
        #[prost(message, tag="12")]
        SetLayerProps(super::SetLayerPropsRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::ResponseType", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
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
        #[prost(message, tag="4")]
        SaveChanges(super::SaveChangesResponse),
        #[prost(bool, tag="5")]
        DiscardChanges(bool),
        #[prost(message, tag="6")]
        GetPhysicalLayouts(super::PhysicalLayouts),
        #[prost(message, tag="7")]
        SetActivePhysicalLayout(super::SetActivePhysicalLayoutResponse),
        #[prost(message, tag="8")]
        MoveLayer(super::MoveLayerResponse),
        #[prost(message, tag="9")]
        AddLayer(super::AddLayerResponse),
        #[prost(message, tag="10")]
        RemoveLayer(super::RemoveLayerResponse),
        #[prost(message, tag="11")]
        RestoreLayer(super::RestoreLayerResponse),
        #[prost(enumeration="super::SetLayerPropsResponse", tag="12")]
        SetLayerProps(i32),
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
pub struct SaveChangesResponse {
    #[prost(oneof="save_changes_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<save_changes_response::Result>,
}
/// Nested message and enum types in `SaveChangesResponse`.
pub mod save_changes_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(bool, tag="1")]
        Ok(bool),
        #[prost(enumeration="super::SaveChangesErrorCode", tag="2")]
        Err(i32),
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
pub struct AddLayerResponse {
    #[prost(oneof="add_layer_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<add_layer_response::Result>,
}
/// Nested message and enum types in `AddLayerResponse`.
pub mod add_layer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="1")]
        Ok(super::AddLayerResponseDetails),
        #[prost(enumeration="super::AddLayerErrorCode", tag="2")]
        Err(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLayerResponseDetails {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(message, optional, tag="2")]
    pub layer: ::core::option::Option<Layer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLayerResponse {
    #[prost(oneof="remove_layer_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<remove_layer_response::Result>,
}
/// Nested message and enum types in `RemoveLayerResponse`.
pub mod remove_layer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="1")]
        Ok(super::RemoveLayerOk),
        #[prost(enumeration="super::RemoveLayerErrorCode", tag="2")]
        Err(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLayerOk {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreLayerResponse {
    #[prost(oneof="restore_layer_response::Result", tags="1, 2")]
    pub result: ::core::option::Option<restore_layer_response::Result>,
}
/// Nested message and enum types in `RestoreLayerResponse`.
pub mod restore_layer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag="1")]
        Ok(super::Layer),
        #[prost(enumeration="super::RestoreLayerErrorCode", tag="2")]
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
pub struct AddLayerRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLayerRequest {
    #[prost(uint32, tag="1")]
    pub layer_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreLayerRequest {
    #[prost(uint32, tag="1")]
    pub layer_id: u32,
    #[prost(uint32, tag="2")]
    pub at_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLayerPropsRequest {
    #[prost(uint32, tag="1")]
    pub layer_id: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keymap {
    #[prost(message, repeated, tag="1")]
    pub layers: ::prost::alloc::vec::Vec<Layer>,
    #[prost(uint32, tag="2")]
    pub available_layers: u32,
    #[prost(uint32, tag="3")]
    pub max_layer_name_length: u32,
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
pub enum SaveChangesErrorCode {
    SaveChangesErrOk = 0,
    SaveChangesErrGeneric = 1,
    SaveChangesErrNotSupported = 2,
    SaveChangesErrNoSpace = 3,
}
impl SaveChangesErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SaveChangesErrorCode::SaveChangesErrOk => "SAVE_CHANGES_ERR_OK",
            SaveChangesErrorCode::SaveChangesErrGeneric => "SAVE_CHANGES_ERR_GENERIC",
            SaveChangesErrorCode::SaveChangesErrNotSupported => "SAVE_CHANGES_ERR_NOT_SUPPORTED",
            SaveChangesErrorCode::SaveChangesErrNoSpace => "SAVE_CHANGES_ERR_NO_SPACE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAVE_CHANGES_ERR_OK" => Some(Self::SaveChangesErrOk),
            "SAVE_CHANGES_ERR_GENERIC" => Some(Self::SaveChangesErrGeneric),
            "SAVE_CHANGES_ERR_NOT_SUPPORTED" => Some(Self::SaveChangesErrNotSupported),
            "SAVE_CHANGES_ERR_NO_SPACE" => Some(Self::SaveChangesErrNoSpace),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetLayerBindingResponse {
    SetLayerBindingRespOk = 0,
    SetLayerBindingRespInvalidLocation = 1,
    SetLayerBindingRespInvalidBehavior = 2,
    SetLayerBindingRespInvalidParameters = 3,
}
impl SetLayerBindingResponse {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetLayerBindingResponse::SetLayerBindingRespOk => "SET_LAYER_BINDING_RESP_OK",
            SetLayerBindingResponse::SetLayerBindingRespInvalidLocation => "SET_LAYER_BINDING_RESP_INVALID_LOCATION",
            SetLayerBindingResponse::SetLayerBindingRespInvalidBehavior => "SET_LAYER_BINDING_RESP_INVALID_BEHAVIOR",
            SetLayerBindingResponse::SetLayerBindingRespInvalidParameters => "SET_LAYER_BINDING_RESP_INVALID_PARAMETERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_LAYER_BINDING_RESP_OK" => Some(Self::SetLayerBindingRespOk),
            "SET_LAYER_BINDING_RESP_INVALID_LOCATION" => Some(Self::SetLayerBindingRespInvalidLocation),
            "SET_LAYER_BINDING_RESP_INVALID_BEHAVIOR" => Some(Self::SetLayerBindingRespInvalidBehavior),
            "SET_LAYER_BINDING_RESP_INVALID_PARAMETERS" => Some(Self::SetLayerBindingRespInvalidParameters),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoveLayerErrorCode {
    MoveLayerErrOk = 0,
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
            MoveLayerErrorCode::MoveLayerErrOk => "MOVE_LAYER_ERR_OK",
            MoveLayerErrorCode::MoveLayerErrGeneric => "MOVE_LAYER_ERR_GENERIC",
            MoveLayerErrorCode::MoveLayerErrInvalidLayer => "MOVE_LAYER_ERR_INVALID_LAYER",
            MoveLayerErrorCode::MoveLayerErrInvalidDestination => "MOVE_LAYER_ERR_INVALID_DESTINATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MOVE_LAYER_ERR_OK" => Some(Self::MoveLayerErrOk),
            "MOVE_LAYER_ERR_GENERIC" => Some(Self::MoveLayerErrGeneric),
            "MOVE_LAYER_ERR_INVALID_LAYER" => Some(Self::MoveLayerErrInvalidLayer),
            "MOVE_LAYER_ERR_INVALID_DESTINATION" => Some(Self::MoveLayerErrInvalidDestination),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddLayerErrorCode {
    AddLayerErrOk = 0,
    AddLayerErrGeneric = 1,
    AddLayerErrNoSpace = 2,
}
impl AddLayerErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddLayerErrorCode::AddLayerErrOk => "ADD_LAYER_ERR_OK",
            AddLayerErrorCode::AddLayerErrGeneric => "ADD_LAYER_ERR_GENERIC",
            AddLayerErrorCode::AddLayerErrNoSpace => "ADD_LAYER_ERR_NO_SPACE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ADD_LAYER_ERR_OK" => Some(Self::AddLayerErrOk),
            "ADD_LAYER_ERR_GENERIC" => Some(Self::AddLayerErrGeneric),
            "ADD_LAYER_ERR_NO_SPACE" => Some(Self::AddLayerErrNoSpace),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemoveLayerErrorCode {
    RemoveLayerErrOk = 0,
    RemoveLayerErrGeneric = 1,
    RemoveLayerErrInvalidIndex = 2,
}
impl RemoveLayerErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RemoveLayerErrorCode::RemoveLayerErrOk => "REMOVE_LAYER_ERR_OK",
            RemoveLayerErrorCode::RemoveLayerErrGeneric => "REMOVE_LAYER_ERR_GENERIC",
            RemoveLayerErrorCode::RemoveLayerErrInvalidIndex => "REMOVE_LAYER_ERR_INVALID_INDEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REMOVE_LAYER_ERR_OK" => Some(Self::RemoveLayerErrOk),
            "REMOVE_LAYER_ERR_GENERIC" => Some(Self::RemoveLayerErrGeneric),
            "REMOVE_LAYER_ERR_INVALID_INDEX" => Some(Self::RemoveLayerErrInvalidIndex),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RestoreLayerErrorCode {
    RestoreLayerErrOk = 0,
    RestoreLayerErrGeneric = 1,
    RestoreLayerErrInvalidId = 2,
    RestoreLayerErrInvalidIndex = 3,
}
impl RestoreLayerErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RestoreLayerErrorCode::RestoreLayerErrOk => "RESTORE_LAYER_ERR_OK",
            RestoreLayerErrorCode::RestoreLayerErrGeneric => "RESTORE_LAYER_ERR_GENERIC",
            RestoreLayerErrorCode::RestoreLayerErrInvalidId => "RESTORE_LAYER_ERR_INVALID_ID",
            RestoreLayerErrorCode::RestoreLayerErrInvalidIndex => "RESTORE_LAYER_ERR_INVALID_INDEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESTORE_LAYER_ERR_OK" => Some(Self::RestoreLayerErrOk),
            "RESTORE_LAYER_ERR_GENERIC" => Some(Self::RestoreLayerErrGeneric),
            "RESTORE_LAYER_ERR_INVALID_ID" => Some(Self::RestoreLayerErrInvalidId),
            "RESTORE_LAYER_ERR_INVALID_INDEX" => Some(Self::RestoreLayerErrInvalidIndex),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetLayerPropsResponse {
    SetLayerPropsRespOk = 0,
    SetLayerPropsRespErrGeneric = 1,
    SetLayerPropsRespErrInvalidId = 2,
}
impl SetLayerPropsResponse {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetLayerPropsResponse::SetLayerPropsRespOk => "SET_LAYER_PROPS_RESP_OK",
            SetLayerPropsResponse::SetLayerPropsRespErrGeneric => "SET_LAYER_PROPS_RESP_ERR_GENERIC",
            SetLayerPropsResponse::SetLayerPropsRespErrInvalidId => "SET_LAYER_PROPS_RESP_ERR_INVALID_ID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_LAYER_PROPS_RESP_OK" => Some(Self::SetLayerPropsRespOk),
            "SET_LAYER_PROPS_RESP_ERR_GENERIC" => Some(Self::SetLayerPropsRespErrGeneric),
            "SET_LAYER_PROPS_RESP_ERR_INVALID_ID" => Some(Self::SetLayerPropsRespErrInvalidId),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetActivePhysicalLayoutErrorCode {
    SetActivePhysicalLayoutErrOk = 0,
    SetActivePhysicalLayoutErrGeneric = 1,
    SetActivePhysicalLayoutErrInvalidLayoutIndex = 2,
}
impl SetActivePhysicalLayoutErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetActivePhysicalLayoutErrorCode::SetActivePhysicalLayoutErrOk => "SET_ACTIVE_PHYSICAL_LAYOUT_ERR_OK",
            SetActivePhysicalLayoutErrorCode::SetActivePhysicalLayoutErrGeneric => "SET_ACTIVE_PHYSICAL_LAYOUT_ERR_GENERIC",
            SetActivePhysicalLayoutErrorCode::SetActivePhysicalLayoutErrInvalidLayoutIndex => "SET_ACTIVE_PHYSICAL_LAYOUT_ERR_INVALID_LAYOUT_INDEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_ACTIVE_PHYSICAL_LAYOUT_ERR_OK" => Some(Self::SetActivePhysicalLayoutErrOk),
            "SET_ACTIVE_PHYSICAL_LAYOUT_ERR_GENERIC" => Some(Self::SetActivePhysicalLayoutErrGeneric),
            "SET_ACTIVE_PHYSICAL_LAYOUT_ERR_INVALID_LAYOUT_INDEX" => Some(Self::SetActivePhysicalLayoutErrInvalidLayoutIndex),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
