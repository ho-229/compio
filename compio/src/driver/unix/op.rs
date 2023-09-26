use std::io::{IoSlice, IoSliceMut};

use compio_buf::{IoBuf, IoBufMut};
use libc::{sockaddr_storage, socklen_t};
use socket2::SockAddr;

#[cfg(doc)]
use crate::op::*;
use crate::{buf::IntoInner, driver::RawFd};

/// Accept a connection.
pub struct Accept {
    pub(crate) fd: RawFd,
    pub(crate) buffer: sockaddr_storage,
    pub(crate) addr_len: socklen_t,
}

impl Accept {
    /// Create [`Accept`].
    pub fn new(fd: RawFd) -> Self {
        Self {
            fd,
            buffer: unsafe { std::mem::zeroed() },
            addr_len: std::mem::size_of::<sockaddr_storage>() as _,
        }
    }

    /// Get the remote address from the inner buffer.
    pub fn into_addr(self) -> SockAddr {
        unsafe { SockAddr::new(self.buffer, self.addr_len) }
    }
}

/// Receive data from remote.
pub struct Recv<T: IoBufMut> {
    pub(crate) fd: RawFd,
    pub(crate) buffer: T,
}

impl<T: IoBufMut> Recv<T> {
    /// Create [`Recv`].
    pub fn new(fd: RawFd, buffer: T) -> Self {
        Self { fd, buffer }
    }
}

impl<T: IoBufMut> IntoInner for Recv<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

/// Receive data from remote into vectored buffer.
pub struct RecvVectored<T: IoBufMut> {
    pub(crate) fd: RawFd,
    pub(crate) buffer: Vec<T>,
    pub(crate) slices: Vec<IoSliceMut<'static>>,
}

impl<T: IoBufMut> RecvVectored<T> {
    /// Create [`RecvVectored`].
    pub fn new(fd: RawFd, buffer: Vec<T>) -> Self {
        Self {
            fd,
            buffer,
            slices: vec![],
        }
    }
}

impl<T: IoBufMut> IntoInner for RecvVectored<T> {
    type Inner = Vec<T>;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

/// Send data to remote.
pub struct Send<T: IoBuf> {
    pub(crate) fd: RawFd,
    pub(crate) buffer: T,
}

impl<T: IoBuf> Send<T> {
    /// Create [`Send`].
    pub fn new(fd: RawFd, buffer: T) -> Self {
        Self { fd, buffer }
    }
}

impl<T: IoBuf> IntoInner for Send<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

/// Send data to remote from vectored buffer.
pub struct SendVectored<T: IoBuf> {
    pub(crate) fd: RawFd,
    pub(crate) buffer: Vec<T>,
    pub(crate) slices: Vec<IoSlice<'static>>,
}

impl<T: IoBuf> SendVectored<T> {
    /// Create [`SendVectored`].
    pub fn new(fd: RawFd, buffer: Vec<T>) -> Self {
        Self {
            fd,
            buffer,
            slices: vec![],
        }
    }
}

impl<T: IoBuf> IntoInner for SendVectored<T> {
    type Inner = Vec<T>;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}
