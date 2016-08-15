//! # Schemes
//! A scheme is a primitive for handling filesystem syscalls in Redox.
//! Schemes accept paths from the kernel for `open`, and file descriptors that they generate
//! are then passed for operations like `close`, `read`, `write`, etc.
//!
//! The kernel validates paths and file descriptors before they are passed to schemes,
//! also stripping the scheme identifier of paths if necessary.

use syscall::Result;

use self::debug::DebugScheme;

/// Debug scheme
pub mod debug;

/// Schemes list
pub static mut SCHEME: DebugScheme = DebugScheme;

/// A scheme trait, implemented by a scheme handler
pub trait Scheme {
    /// Open the file at `path` with `flags`.
    ///
    /// Returns a file descriptor or an error
    fn open(&mut self, path: &[u8], flags: usize) -> Result<usize>;

    /// Read the file `number` into the `buffer`
    ///
    /// Returns the number of bytes read
    fn read(&mut self, file: usize, buffer: &mut [u8]) -> Result<usize>;

    /// Write the `buffer` to the `file`
    ///
    /// Returns the number of bytes written
    fn write(&mut self, file: usize, buffer: &[u8]) -> Result<usize>;

    /// Close the file `number`
    fn close(&mut self, file: usize) -> Result<()>;
}
