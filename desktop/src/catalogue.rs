// Copyright (c) 2026 Wave-FM-1-plugin contributors. MIT.
//! Fixed, offline catalogue. Data files are separate works, not linked firmware code.
use crate::{Firmware, savefile::MAX_PACKAGE};
use sha2::{Digest, Sha256};
use std::{io::Read, path::{Path, PathBuf}, sync::Arc};

pub struct Entry {
    pub id: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub license: &'static str,
    pub description: &'static str,
    pub file: &'static str,
    pub sha256: &'static str,
    pub size: usize,
}

include!("catalogue_entries.rs");

pub fn read_at(directory: &Path, entry: &Entry) -> Result<Firmware, String> {
    // No paths from a downloaded manifest, environment variable or working directory.
    if entry.file.contains(['/', '\\']) || !entry.file.ends_with(".fwsc") {
        return Err("invalid catalogue filename".into());
    }
    let mut pkg = Vec::new();
    std::fs::File::open(directory.join(entry.file)).map_err(|_| "bundled firmware missing; reinstall the complete plugin".to_owned())?
        .take((MAX_PACKAGE + 1) as u64).read_to_end(&mut pkg).map_err(|e| e.to_string())?;
    if pkg.len() != entry.size || pkg.len() > MAX_PACKAGE || format!("{:x}", Sha256::digest(&pkg)) != entry.sha256 {
        return Err("bundled firmware failed SHA-256 verification; reinstall it from the release".into());
    }
    Ok(Firmware { name: format!("{} {}", entry.name, entry.version), pkg: Arc::new(pkg) })
}

pub fn read(entry: &Entry) -> Result<Firmware, String> {
    let directory = module_directory().ok_or("cannot locate the plugin's private firmware folder")?;
    #[cfg(target_os = "macos")]
    let directory = directory.parent().ok_or("invalid CLAP bundle layout")?.join("Resources");
    read_at(&directory.join("firmwares"), entry)
}

// current_exe() would locate Ableton, not this DLL. Locate the CLAP module by an
// address inside it; never search global folders or load code from the catalogue.
#[cfg(windows)]
fn module_directory() -> Option<PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleExW(flags: u32, address: *const u16, module: *mut isize) -> i32;
        fn GetModuleFileNameW(module: isize, name: *mut u16, size: u32) -> u32;
    }
    let mut module = 0;
    let mut path = vec![0u16; 32768];
    unsafe {
        if GetModuleHandleExW(0x00000004 | 0x00000002, module_directory as *const () as *const u16, &mut module) == 0 { return None; }
        let count = GetModuleFileNameW(module, path.as_mut_ptr(), path.len() as u32) as usize;
        if count == 0 || count >= path.len() { return None; }
        PathBuf::from(std::ffi::OsString::from_wide(&path[..count])).parent().map(Path::to_path_buf)
    }
}

#[cfg(target_os = "macos")]
fn module_directory() -> Option<PathBuf> {
    use std::ffi::{c_char, c_void, CStr};
    #[repr(C)]
    struct Info { filename: *const c_char, base: *mut c_void, symbol: *const c_char, address: *mut c_void }
    unsafe extern "C" { fn dladdr(address: *const c_void, info: *mut Info) -> i32; }
    let mut info = Info { filename: std::ptr::null(), base: std::ptr::null_mut(), symbol: std::ptr::null(), address: std::ptr::null_mut() };
    unsafe {
        if dladdr(module_directory as *const () as *const c_void, &mut info) == 0 || info.filename.is_null() { return None; }
        Path::new(CStr::from_ptr(info.filename).to_str().ok()?).parent().map(Path::to_path_buf)
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
fn module_directory() -> Option<PathBuf> { None }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_missing_or_modified_bundled_firmware() {
        let dir = tempfile::tempdir().unwrap();
        let e = &ENTRIES[0];
        assert!(read_at(dir.path(), e).is_err());
        std::fs::write(dir.path().join(e.file), vec![0; e.size]).unwrap();
        assert!(read_at(dir.path(), e).is_err());
    }
}
