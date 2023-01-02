//! This crate provides the trait [`AsRawXcbConnection`].
//!
//! The idea is to facilitate interoperability in the ecosystem. The problem is as follows:
//!
//! There are multiple crates that wrap the libxcb C API to provide a "connection" type. There are
//! also multiple crates wrapping various C libraries that need a pointer to `xcb_connection_t`
//! to work correctly.
//!
//! Without this library, API consumers must pick one Rust library that wraps libxcb and only
//! accept this type in its public API. Worse, one must also pick a specific version of the crate
//! and would then only work with that type.
//!
//! The trait [`AsRawXcbConnection`] breaks this connection. All libraries that wrap libxcb can
//! implement this trait. This makes one independent from specific versions of API consumer crates.

#![allow(non_camel_case_types)]
/// XCB connection
///
/// This type represents `xcb_connection_t` in C. It is only ever referenced via a pointer.
pub enum xcb_connection_t {}

/// A trait to extract a raw `xcb_connection_t` from an object.
///
/// # Safety
///
/// This trait is unsafe. Implementations must provide a valid connection pointer that can be used
/// with libxcb C functions. This pointer must be valid for as long as the object on which this
/// trait is implemented.
pub unsafe trait AsRawXcbConnection {
    /// Get a raw xcb connection pointer from this object.
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t;
}
