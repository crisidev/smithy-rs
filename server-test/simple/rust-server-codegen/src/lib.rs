#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! A simple service example, with a Service resource that can be registered and a readonly healthcheck

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Errors that can occur when calling the service.
pub mod error;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_ser;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
/// A registry of your service's operations.
pub mod operation_registry;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
pub use aws_smithy_types::DateTime;