use super::Token;

/// Trait to be implemented by Os specific Event types
///
/// The ffi interfaces that create, or poll / select all require `Events` as
/// arguments to their api's. If we created just a standalone `Our::Event` type
/// and passed these into selectors, we would need to iterate over
/// `Vec<Our::Event>` to convert to `Vec<ffi::Event>`. This is wasteful and it
/// makes more sense to use correct OS event types throughout all of this
/// libraries API's. However, we will hide the internal structure of these
/// Events outside of Selectors etc. To facilitate this, we create and interface
/// that all OS specific Event types must implement. This should not impact
/// performance, due to monomorphization: we are not using trait objects /
/// dynamic dispatch.
///
/// # NOTE
///
/// Although marked as public, the module never allows import
/// of the trait outside of the crate. It is not expected that
/// users should implement and use their own inner types.
pub trait SysEvent: Sized {
    /// Return token for current OS `event` type
    fn token(&self) -> Token;

    /// Is event a type that allows use to read from it's associated file descriptor?
    fn is_readable(&self) -> bool;

    /// Reading from the file descriptor is closed?
    fn is_read_closed(&self) -> bool;

    /// Is event a type that allows use to write to it's associated file descriptor?
    fn is_writable(&self) -> bool;

    /// Writing to the file descriptor is closed?
    fn is_write_closed(&self) -> bool;

    /// Is event a type that indicates an error on it's associated file descriptor?
    fn is_error(&self) -> bool;
}
