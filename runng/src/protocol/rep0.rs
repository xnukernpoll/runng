//! Request/reply pattern.

use super::*;
use crate::{asyncio::*, *};
use runng_sys::*;

/// Reply half of request/reply pattern.  See [nng_rep](https://nng.nanomsg.org/man/v1.2.2/nng_rep.7).
#[derive(Clone, Debug, NngGetOpts, NngSetOpts)]
#[prefix = "nng_socket_"]
pub struct Rep0 {
    socket: NngSocket,
}

impl Rep0 {
    /// Create a new reply socket.  See [nng_rep_open](https://nng.nanomsg.org/man/v1.2.2/nng_rep_open.3).
    pub fn open() -> Result<Self> {
        nng_open(
            |socket| unsafe { nng_rep0_open(socket) },
            |socket| Rep0 { socket },
        )
    }
}

impl GetSocket for Rep0 {
    fn socket(&self) -> &NngSocket {
        &self.socket
    }
    fn socket_mut(&mut self) -> &mut NngSocket {
        &mut self.socket
    }
}

impl Socket for Rep0 {}
impl Listen for Rep0 {}
impl Dial for Rep0 {}
impl RecvSocket for Rep0 {}
impl SendSocket for Rep0 {}

impl AsyncSocket for Rep0 {
    type ContextType = ReplyAsyncHandle;
}

impl AsyncStream for Rep0 {
    type ContextType = ReplyStreamHandle;
}
