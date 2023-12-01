use std::{ffi::OsStr, io, path::PathBuf};

#[cfg(unix)]
pub fn to_os_str(buf: &[u8]) -> Result<&OsStr, std::str::Utf8Error> {
    Ok(std::os::unix::ffi::OsStrExt::from_bytes(buf))
}

#[cfg(not(unix))]
pub fn to_os_str(buf: &[u8]) -> Result<&OsStr, std::str::Utf8Error> {
    // On Windows (and everything else) we will expect well-formed UTF-8 and pray
    Ok(OsStr::new(std::str::from_utf8(buf)?))
}

pub fn read_files_list(mut reader: impl io::Read, separator: u8) -> io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;

    buf.split(|c| *c == separator)
        .filter(|s| !s.is_empty())
        .map(|s| {
            to_os_str(s)
                .map(PathBuf::from)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

pub fn return_empty_vec() -> Vec<PathBuf> {
    let vec: Vec<PathBuf> = Vec::new();
    return vec;
}
