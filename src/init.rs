use std::os::fd::OwnedFd;

use nix::sys::memfd::MFdFlags;

static INIT_BIN: &[u8] = include_bytes!(env!("CARGO_BIN_FILE_MICROPOD_INIT"));

pub fn get_init_fd() -> crate::Result<OwnedFd> {
    let fd = nix::sys::memfd::memfd_create("micropod_init", MFdFlags::empty())
        .map_err(crate::Error::Memfd)?;

    let mut buf = INIT_BIN;
    while !buf.is_empty() {
        let written = nix::unistd::write(&fd, buf).map_err(crate::Error::Memfd)?;
        buf = &buf[written..];
    }

    Ok(fd)
}
