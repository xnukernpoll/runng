//! Push/pull ("pipeline") pattern.

use super::*;
use crate::asyncio::*;
use runng_sys::*;

/// Pull half of push/pull ("pipeline") pattern.  See [nng_pull](https://nng.nanomsg.org/man/v1.2.2/nng_pull.7).
#[derive(Clone, Debug, NngGetOpts, NngSetOpts)]
#[prefix = "nng_socket_"]
pub struct Pull0 {
    socket: NngSocket,
}

impl Pull0 {
    /// Create a pull socket.  See [nng_pull_open](https://nng.nanomsg.org/man/v1.2.2/nng_pull_open.3).
    pub fn open() -> Result<Self> {
        nng_open(
            |socket| unsafe { nng_pull0_open(socket) },
            |socket| Pull0 { socket },
        )
    }
}

impl GetSocket for Pull0 {
    fn socket(&self) -> &NngSocket {
        &self.socket
    }
    fn socket_mut(&mut self) -> &mut NngSocket {
        &mut self.socket
    }
}

impl Socket for Pull0 {}
impl Dial for Pull0 {}
impl Listen for Pull0 {}
impl RecvSocket for Pull0 {}

impl AsyncSocket for Pull0 {
    type ContextType = PullAsyncHandle;
}

impl AsyncStream for Pull0 {
    type ContextType = PullAsyncStream;
}
