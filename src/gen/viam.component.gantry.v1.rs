// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    #[prost(double, repeated, tag="1")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Number of millimeters to move the gantry by respective to each axis.
    #[prost(double, repeated, tag="2")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLengthsRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLengthsResponse {
    #[prost(double, repeated, tag="1")]
    pub lengths_mm: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a gantry
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(double, repeated, tag="1")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag="2")]
    pub lengths_mm: ::prost::alloc::vec::Vec<f64>,
    #[prost(bool, tag="3")]
    pub is_moving: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMovingRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMovingResponse {
    #[prost(bool, tag="1")]
    pub is_moving: bool,
}
/// Encoded file descriptor set for the `viam.component.gantry.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x86, 0x23, 0x0a, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67,
    0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x18, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x1a,
    0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f,
    0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f,
    0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x57, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a,
    0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53,
    0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x38, 0x0a, 0x13,
    0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x5f, 0x6d, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x52, 0x0b, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x4d, 0x6d, 0x22, 0x7d, 0x0a, 0x15, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x5f, 0x6d, 0x6d, 0x18, 0x02, 0x20, 0x03, 0x28, 0x01, 0x52, 0x0b, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x4d, 0x6d, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18,
    0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05,
    0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x18, 0x0a, 0x16, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x50,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x56, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72,
    0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x33, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x4c, 0x65,
    0x6e, 0x67, 0x74, 0x68, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x5f, 0x6d, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x01, 0x52, 0x09, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x4d, 0x6d, 0x22, 0x50, 0x0a, 0x0b,
    0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x0e,
    0x0a, 0x0c, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x67,
    0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5f, 0x6d, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x52, 0x0b,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x4d, 0x6d, 0x12, 0x1d, 0x0a, 0x0a, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x5f, 0x6d, 0x6d, 0x18, 0x02, 0x20, 0x03, 0x28, 0x01, 0x52,
    0x09, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x4d, 0x6d, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x73,
    0x5f, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x69,
    0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x22, 0x25, 0x0a, 0x0f, 0x49, 0x73, 0x4d, 0x6f, 0x76,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x2f,
    0x0a, 0x10, 0x49, 0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x73, 0x5f, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x69, 0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x32,
    0xb7, 0x07, 0x0a, 0x0d, 0x47, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x12, 0xa1, 0x01, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x2c, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x2d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x35,
    0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2f, 0x12, 0x2d, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70,
    0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67,
    0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0xae, 0x01, 0x0a, 0x0e, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79,
    0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x30, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72,
    0x79, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x39, 0xa0, 0x92, 0x29,
    0x01, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2f, 0x1a, 0x2d, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61,
    0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f,
    0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x70, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x9d, 0x01, 0x0a, 0x0a, 0x47, 0x65, 0x74, 0x4c, 0x65,
    0x6e, 0x67, 0x74, 0x68, 0x73, 0x12, 0x2b, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31,
    0x2e, 0x47, 0x65, 0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x2c, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e,
    0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65,
    0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x34, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2e, 0x12, 0x2c, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f,
    0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2f, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x73, 0x12, 0x88, 0x01, 0x0a, 0x04, 0x53, 0x74, 0x6f, 0x70, 0x12,
    0x25, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x26, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f,
    0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76,
    0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31,
    0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2b, 0x22, 0x29, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70,
    0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67,
    0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x73, 0x74, 0x6f,
    0x70, 0x12, 0x99, 0x01, 0x0a, 0x08, 0x49, 0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x12, 0x29,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e,
    0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x73, 0x4d, 0x6f, 0x76, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2a, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x61, 0x6e, 0x74, 0x72,
    0x79, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x36, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x30, 0x12, 0x2e, 0x2f,
    0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x7b, 0x6e, 0x61,
    0x6d, 0x65, 0x7d, 0x2f, 0x69, 0x73, 0x5f, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x12, 0x89, 0x01,
    0x0a, 0x09, 0x44, 0x6f, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x20, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x6f, 0x43,
    0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x21, 0x2e,
    0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x44,
    0x6f, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x37, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x31, 0x22, 0x2f, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f,
    0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2f, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x64,
    0x6f, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x42, 0x43, 0x0a, 0x1c, 0x63, 0x6f, 0x6d,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e,
    0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x5a, 0x23, 0x67, 0x6f, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x4a, 0x90,
    0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x21, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x05, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x26, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x3a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x08, 0x00, 0x3a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x35, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x35, 0x0a, 0x4c, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04,
    0x0c, 0x00, 0x37, 0x01, 0x1a, 0x40, 0x20, 0x41, 0x6e, 0x20, 0x47, 0x61, 0x6e, 0x74, 0x72, 0x79,
    0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73,
    0x20, 0x61, 0x6c, 0x6c, 0x20, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x61, 0x73,
    0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20,
    0x72, 0x6f, 0x62, 0x6f, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x08, 0x15, 0x0a, 0x5a, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0e, 0x02, 0x12, 0x03,
    0x1a, 0x4c, 0x20, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x67,
    0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x67, 0x61,
    0x6e, 0x74, 0x72, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65,
    0x72, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x2f, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x0f, 0x04, 0x11, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04,
    0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x0f, 0x04, 0x11, 0x06, 0x0a, 0x60, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x01, 0x12, 0x04, 0x15, 0x02, 0x1a, 0x03, 0x1a, 0x52, 0x20, 0x4d, 0x6f, 0x76, 0x65, 0x54,
    0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x73, 0x20,
    0x61, 0x20, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65,
    0x64, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x06, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x15, 0x15, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x15, 0x35, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x16, 0x04, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x06, 0x00, 0x02, 0x01, 0x04, 0xa4, 0x92, 0x05,
    0x12, 0x03, 0x16, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x17, 0x04, 0x19, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x01, 0x04, 0xb0, 0xca, 0xbc,
    0x22, 0x12, 0x04, 0x17, 0x04, 0x19, 0x06, 0x0a, 0x50, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12,
    0x04, 0x1d, 0x02, 0x21, 0x03, 0x1a, 0x42, 0x20, 0x47, 0x65, 0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74,
    0x68, 0x73, 0x20, 0x67, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x67,
    0x74, 0x68, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c, 0x79, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x1d, 0x06, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x1d, 0x11, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1d, 0x2d, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1e, 0x04,
    0x20, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12,
    0x04, 0x1e, 0x04, 0x20, 0x06, 0x0a, 0x2b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x04, 0x24,
    0x02, 0x28, 0x03, 0x1a, 0x1d, 0x20, 0x53, 0x74, 0x6f, 0x70, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x73,
    0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x67, 0x61, 0x6e, 0x74, 0x72,
    0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x06, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x24, 0x0b, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x25, 0x04, 0x27, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06,
    0x00, 0x02, 0x03, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x25, 0x04, 0x27, 0x06, 0x0a, 0x3c,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x04, 0x2b, 0x02, 0x2f, 0x03, 0x1a, 0x2e, 0x20, 0x49,
    0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20,
    0x69, 0x66, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2b, 0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x2b, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x2b, 0x29, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x2c, 0x04, 0x2e, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x04, 0x04, 0xb0, 0xca,
    0xbc, 0x22, 0x12, 0x04, 0x2c, 0x04, 0x2e, 0x06, 0x0a, 0x3b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x05,
    0x12, 0x04, 0x32, 0x02, 0x36, 0x03, 0x1a, 0x2d, 0x20, 0x44, 0x6f, 0x43, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x73, 0x2f, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65,
    0x73, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x32, 0x06, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x32, 0x10,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x32, 0x35, 0x50, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x33, 0x04, 0x35, 0x06, 0x0a, 0x11,
    0x0a, 0x09, 0x06, 0x00, 0x02, 0x05, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x33, 0x04, 0x35,
    0x06, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x39, 0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x39, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x3a, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x3a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x10, 0x11,
    0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x24, 0x1a, 0x24, 0x20,
    0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68,
    0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3c, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x19, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x21, 0x23, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x3f, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x3f, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x40,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x43, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x43,
    0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x44, 0x02, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x10, 0x11, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x46, 0x02, 0x23, 0x1a, 0x46, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20,
    0x6f, 0x66, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x61, 0x6e, 0x74, 0x72,
    0x79, 0x20, 0x62, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x61, 0x78, 0x69, 0x73, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x46, 0x21, 0x22, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x48, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x48, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x48, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x48, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4b, 0x00, 0x21, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x4d, 0x00, 0x51, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4d,
    0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x02, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4e, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x50, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x50, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x50, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x50, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x53, 0x00,
    0x55, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x53, 0x08, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x54, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x54, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x54, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x57, 0x00, 0x5c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x57, 0x08, 0x13, 0x0a, 0x1f, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x12, 0x1a, 0x12, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x20, 0x67, 0x61, 0x6e, 0x74, 0x72, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x59, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03,
    0x5b, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x5b, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x5b, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x5b, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x03, 0x5e, 0x00, 0x17, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x60, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x60,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x61, 0x02, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x61, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x62, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x62,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x62, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x62, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x63, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x63, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x63, 0x07, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x63, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x66, 0x00, 0x68, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x66, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x00, 0x12, 0x03, 0x67, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x67, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x67, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67,
    0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x6a, 0x00, 0x6c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x03, 0x6b, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x6b, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x6b, 0x07, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x13,
    0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.component.gantry.v1.tonic.rs");
// @@protoc_insertion_point(module)