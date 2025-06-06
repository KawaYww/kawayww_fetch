use crate::{models::OSRelease, platforms::unix::utils};

pub fn new() -> Option<OSRelease> {
    let os_release = utils::read_file("/etc/os-release")?;

    // TODO

    todo!()
}
