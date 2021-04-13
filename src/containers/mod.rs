/// Module with List container
pub mod list;

/// Module with MaybePtr container
pub mod maybe_ptr;

/// Module with Ptr container
pub mod ptr;

/// Module with LocPtr container
pub mod loc_ptr;

/// Module with MaybeLocPtr container
pub mod maybe_loc_ptr;

/// Module with String container
pub mod string;

#[cfg(not(feature = "c-structures"))]
pub use list::rust::List;
#[cfg(not(feature = "c-structures"))]
pub use loc_ptr::rust::LocPtr;
#[cfg(not(feature = "c-structures"))]
pub use maybe_loc_ptr::rust::MaybeLocPtr;
#[cfg(not(feature = "c-structures"))]
pub use maybe_ptr::rust::MaybePtr;
#[cfg(not(feature = "c-structures"))]
pub use ptr::rust::Ptr;
#[cfg(not(feature = "c-structures"))]
pub use string::rust::String;

#[cfg(feature = "c-structures")]
pub use list::c::List;
#[cfg(feature = "c-structures")]
pub use loc_ptr::c::LocPtr;
#[cfg(feature = "c-structures")]
pub use maybe_loc_ptr::c::MaybeLocPtr;
#[cfg(feature = "c-structures")]
pub use maybe_ptr::c::MaybePtr;
#[cfg(feature = "c-structures")]
pub use ptr::c::Ptr;
#[cfg(feature = "c-structures")]
pub use string::c::String;
