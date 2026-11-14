use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

#[repr(C)]
pub struct SockAddrRc {
    pub rc_family: libc::sa_family_t,
    pub rc_bdaddr: [u8; 6],
    pub rc_channel: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sockaddr_rc_layout() {
        assert_eq!(std::mem::size_of::<SockAddrRc>(), 10);
        assert_eq!(std::mem::align_of::<SockAddrRc>(), 2);

        assert_eq!(std::mem::offset_of!(SockAddrRc, rc_family), 0);
        assert_eq!(std::mem::offset_of!(SockAddrRc, rc_bdaddr), 2);
        assert_eq!(std::mem::offset_of!(SockAddrRc, rc_channel), 8);
    }

    #[test]
    fn create_rfcomm_socket() {
        let fd = unsafe {
            libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_STREAM,
                3, // BTPROTO_RFCOMM
            )
        };

        assert!(
            fd >= 0,
            "socket() failed: {}",
            std::io::Error::last_os_error()
        );

        let socket = unsafe {
            OwnedFd::from_raw_fd(fd)
        };

        println!("RFCOMM socket = {socket:?}");
    }

    #[test]
    fn bind_rfcomm_channel_1() {
        let fd = unsafe {
            libc::socket(libc::AF_BLUETOOTH, libc::SOCK_STREAM, 3)
        };

        assert!(
            fd >= 0,
            "socket() failed: {}",
            std::io::Error::last_os_error()
        );

        let socket = unsafe {
            OwnedFd::from_raw_fd(fd)
        };

        let address = SockAddrRc {
            rc_family: libc::AF_BLUETOOTH as libc::sa_family_t,
            rc_bdaddr: [0; 6],
            rc_channel: 1,
        };

        let result = unsafe {
            libc::bind(
                socket.as_raw_fd(),
                (&address as *const SockAddrRc).cast::<libc::sockaddr>(),
                std::mem::size_of::<SockAddrRc>() as libc::socklen_t,
            )
        };

        if result == 0 {
            let listen_result = unsafe {
                libc::listen(socket.as_raw_fd(), 1)
            };

            assert_eq!(
                listen_result,
                0,
                "listen() failed: {}",
                std::io::Error::last_os_error()
            );
        }

        let error = std::io::Error::last_os_error();

        assert_eq!(result, 0, "bind() failed: {error}");
    }
}


