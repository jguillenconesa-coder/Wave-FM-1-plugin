// MIT. Optional community browser and bounded HTTPS downloads. See desktop/README.md.
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub const HUB: &str = "https://thefirmwarehub.com/";
pub const CATALOGUE_URL: &str = "https://thefirmwarehub.com/catalog.json";
const MAX_CATALOGUE: usize = 2 << 20;
const INDEX: &str = include_str!("community-index.json");

#[derive(Clone, Deserialize)]
pub struct Entry {
    pub id: String,
    #[serde(default)] pub hub: String,
    pub name: String,
    pub version: String,
    pub author: String,
    #[serde(default)] pub license: String,
    #[serde(default)] pub repo: String,
    #[serde(default)] pub package: String,
    #[serde(default)] pub alternate_package: String,
    #[serde(default)] pub alternate_version: String,
    #[serde(default)] pub alternate_sha256: String,
    #[serde(default)] pub alternate_smoke: Option<bool>,
}

#[derive(Deserialize)]
struct Index { firmwares: Vec<Entry> }

pub fn offline() -> Vec<Entry> {
    serde_json::from_str::<Index>(INDEX).expect("checked-in community index").firmwares
}

/// Parse data only: never evaluate the site's JavaScript, HTML or embedded programs.
pub fn parse_catalogue(bytes: &[u8]) -> Result<Vec<Entry>, String> {
    if bytes.len() > MAX_CATALOGUE { return Err("Hub catalogue is too large".into()); }
    let root: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| "Hub did not return a JSON catalogue")?;
    let list = root.get("firmwares").and_then(|v| v.as_array()).ok_or("Hub catalogue format changed")?;
    if list.is_empty() || list.len() > 1000 { return Err("Hub catalogue count is invalid".into()); }
    let mut entries = Vec::new();
    let mut ids = std::collections::HashSet::new();
    let known_entries = offline();
    let text = |value: &serde_json::Value, key: &str, max: usize| -> Result<String, String> {
        let value = match value.get(key) {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Number(n)) => n.to_string(),
            None | Some(serde_json::Value::Null) => String::new(),
            _ => return Err(format!("Invalid Hub field: {key}")),
        };
        if value.len() > max || value.chars().any(char::is_control) { return Err(format!("Invalid Hub field: {key}")); }
        Ok(value)
    };
    for item in list {
        let id = text(item, "id", 96)?;
        if id.is_empty() || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)) || !ids.insert(id.clone()) {
            return Err("Duplicate or invalid Hub identifier".into());
        }
        let package = text(item, "pkg", 768)?;
        let package = if package.starts_with("https://") { package }
            else if package.is_empty() { String::new() }
            else { format!("{HUB}{}", package.strip_prefix('/').unwrap_or(&package)) };
        // An unsupported address can still be listed, but cannot be fetched.
        let repo = text(item, "repo", 256)?;
        let repo = repo.strip_prefix("https://github.com/").unwrap_or(&repo).trim_end_matches('/');
        let repo = if valid_repo(&repo) { format!("https://github.com/{repo}") } else { String::new() };
        let name = text(item, "name", 160)?;
        if name.is_empty() { return Err("Hub entry has no name".into()); }
        let known = known_entries.iter().find(|e| (e.hub == id || e.id == id) && e.repo.eq_ignore_ascii_case(&repo));
        entries.push(Entry { id: id.clone(), hub: id, name,
            version: text(item, "version", 96)?, author: text(item, "author", 160)?,
            license: "See the author's terms; not verified by this plugin".into(), repo, package,
            alternate_package: known.as_ref().map_or(String::new(), |e| e.alternate_package.clone()),
            alternate_version: known.as_ref().map_or(String::new(), |e| e.alternate_version.clone()),
            alternate_sha256: known.map_or(String::new(), |e| e.alternate_sha256.clone()),
            alternate_smoke: known.and_then(|e| e.alternate_smoke) });
    }
    Ok(entries)
}

fn valid_repo(repo: &str) -> bool {
    let parts: Vec<_> = repo.split('/').collect();
    parts.len() == 2 && parts.iter().all(|p| !p.is_empty() && *p != "." && *p != ".." &&
        p.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)))
}

/// Exact public hosts only. Signed queries are accepted only for GitHub's asset CDN.
/// No credentials, ports, IP literals, local-network destinations or executable downloads.
fn split_allowed(url: &str) -> Result<(&str, &str), String> {
    let rest = url.strip_prefix("https://").ok_or("Only HTTPS downloads are allowed")?;
    let (host, path) = rest.split_once('/').ok_or("Invalid download address")?;
    if url.len() > 8192 || path.is_empty() || path.contains("..") ||
        !path.bytes().all(|b| b.is_ascii_alphanumeric() || b"/_.-%?&=~:+,".contains(&b)) {
        return Err("Unsupported download address".into());
    }
    let permitted = match host {
        "thefirmwarehub.com" => path == "catalog.json" || path.starts_with("fw/") && path.ends_with(".fwsc") &&
            path.bytes().all(|b| b.is_ascii_alphanumeric() || b"/_.-".contains(&b)),
        "github.com" => path.split('/').count() >= 6 && path.contains("/releases/download/") && path.ends_with(".fwsc") && !path.contains('?'),
        "release-assets.githubusercontent.com" => path.starts_with("github-production-release-asset/") && path.contains('?'),
        "raw.githubusercontent.com" => path.starts_with("zvenson/dxsloop/") && path.ends_with(".fwsc") && !path.contains('?'),
        "joakimeriksson.github.io" => path == "fm1-emu/fm1-sid/sidplayer.fwsc",
        "shaw-core.github.io" => path == "Sloop_ALG02/firmware/sloop-ALG05-TEST.fwsc",
        _ => false,
    };
    if !permitted { return Err("Download host or path is not allowed; use the author's page and manual import".into()); }
    Ok((host, path))
}

pub fn downloadable(entry: &Entry) -> bool {
    entry.package.ends_with(".fwsc") && split_allowed(&entry.package).is_ok()
}

pub fn alternative(entry: &Entry) -> Option<Entry> {
    if entry.alternate_package.is_empty() { return None; }
    let mut alternate = entry.clone();
    alternate.package = entry.alternate_package.clone();
    alternate.version = entry.alternate_version.clone();
    if !downloadable(&alternate) { return None; }
    Some(alternate)
}

/// The main download button and the network worker must resolve the same source.
/// Pinned author releases take priority over Hub links that may reject downloads.
pub fn preferred(entry: &Entry) -> Entry {
    alternative(entry).unwrap_or_else(|| entry.clone())
}

pub fn provider(entry: &Entry) -> &'static str {
    if entry.package.starts_with("https://github.com/") { "GitHub del autor" }
    else if entry.package.starts_with(HUB) { "Firmware Hub" }
    else { "sitio del autor" }
}

struct Response { status: u32, location: Option<String>, body: Vec<u8> }

fn fetch_with(url: &str, limit: usize, mut transport: impl FnMut(&str, usize) -> Result<Response, String>) -> Result<Vec<u8>, String> {
    let mut current = url.to_owned();
    let started = std::time::Instant::now();
    for _ in 0..4 {
        if started.elapsed().as_secs() >= 30 { return Err("HTTPS operation timed out".into()); }
        split_allowed(&current)?;
        let response = transport(&current, limit)?;
        if response.body.len() > limit { return Err("Download exceeds the size limit".into()); }
        if response.status == 200 { return Ok(response.body); }
        if [301, 302, 303, 307, 308].contains(&response.status) {
            let location = response.location.ok_or("Redirect has no destination")?;
            current = if location.starts_with('/') && !location.starts_with("//") {
                let (host, _) = split_allowed(&current)?;
                format!("https://{host}{location}")
            } else { location };
        } else { return Err(format!("Server returned HTTP {}. The included catalogue and manual import remain available.", response.status)); }
    }
    Err("Too many download redirects".into())
}

pub fn check() -> Result<Vec<Entry>, String> {
    parse_catalogue(&fetch_with(CATALOGUE_URL, MAX_CATALOGUE, transport::get)?)
}

#[derive(Debug)]
pub struct Download {
    pub path: PathBuf,
    pub sha256: String,
    pub source: String,
    pub version: String,
    pub reused: bool,
}

/// Saves only to a user-selected .fwsc file, never the bundle or a firmware in an open Set.
pub fn download(entry: &Entry, path: &Path) -> Result<Download, String> {
    download_with(entry, path, transport::get)
}

fn download_with(entry: &Entry, path: &Path, mut transport: impl FnMut(&str, usize) -> Result<Response, String>) -> Result<Download, String> {
    let mut target = preferred(entry);
    if !downloadable(&target) { return Err("No hay una descarga .fwsc directa compatible. Abre la página del autor.".into()); }
    if path.extension().and_then(|s| s.to_str()).is_none_or(|s| !s.eq_ignore_ascii_case("fwsc")) {
        return Err("Elige un nombre de archivo terminado en .fwsc.".into());
    }
    let pinned = target.package == entry.alternate_package && !entry.alternate_sha256.is_empty();
    let fetched = fetch_with(&target.package, crate::savefile::MAX_PACKAGE, &mut transport);
    let bytes = match fetched {
        Ok(bytes) => bytes,
        Err(first_error) => {
            // Retry only the SAME version against the known pinned hash. Never
            // silently substitute the Hub's older/newer firmware or bypass a hash error.
            let same_version = |a: &str, b: &str| a.trim_start_matches(['v', 'V']) == b.trim_start_matches(['v', 'V']);
            if !pinned || target.package == entry.package || !same_version(&target.version, &entry.version) || !downloadable(entry) {
                return Err(format!("No se pudo descargar {} {} desde {}: {first_error} Usa «Download in browser» si tu red bloquea la conexión del plugin.", entry.name, target.version, provider(&target)));
            }
            let bytes = fetch_with(&entry.package, crate::savefile::MAX_PACKAGE, &mut transport)
                .map_err(|second_error| format!("Falló GitHub del autor: {first_error}\nTambién falló Firmware Hub: {second_error}\nPrueba «Download in browser»."))?;
            target.package = entry.package.clone();
            bytes
        }
    };
    if pinned && format!("{:x}", Sha256::digest(&bytes)) != entry.alternate_sha256 {
        return Err("La descarga no coincide con el SHA-256 de referencia. No se ha guardado ni cargado ningún firmware.".into());
    }
    let mut downloaded = save(&bytes, path)?;
    downloaded.source = target.package;
    downloaded.version = target.version;
    Ok(downloaded)
}

fn save(bytes: &[u8], path: &Path) -> Result<Download, String> {
    if path.extension().and_then(|s| s.to_str()).is_none_or(|s| !s.eq_ignore_ascii_case("fwsc")) {
        return Err("Choose a filename ending in .fwsc".into());
    }
    if !(20 * 0x30..=crate::savefile::MAX_PACKAGE).contains(&bytes.len()) {
        return Err("Invalid firmware package size".into());
    }
    // Parser only; do not run downloaded firmware in the network worker or replace a Set.
    std::panic::catch_unwind(|| fm1_soc::stock_from_package(bytes, None))
        .map_err(|_| "Invalid firmware package")?.map_err(|e| format!("Invalid firmware package: {e}"))?;
    let sha256 = format!("{:x}", Sha256::digest(bytes));
    if same_file(path, bytes)? {
        return Ok(Download { path: path.into(), sha256, source: String::new(), version: String::new(), reused: true });
    }
    let parent = path.parent().ok_or("Choose a local firmware file")?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    use std::io::Write;
    temporary.write_all(bytes).map_err(|e| e.to_string())?;
    temporary.as_file().sync_all().map_err(|e| e.to_string())?;
    if let Err(error) = temporary.persist_noclobber(path) {
        if same_file(path, bytes)? {
            return Ok(Download { path: path.into(), sha256, source: String::new(), version: String::new(), reused: true });
        }
        return Err(format!("No se pudo guardar {}: {}. Elige otra carpeta o un nombre nuevo; no se ha sobrescrito ningún archivo.", path.display(), error.error));
    }
    Ok(Download { path: path.into(), sha256, source: String::new(), version: String::new(), reused: false })
}

fn same_file(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("No se puede comprobar el destino: {error}")),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() != bytes.len() as u64 {
        return Err("Ya existe otro archivo o enlace con ese nombre. Elige un nombre nuevo; se conserva el archivo existente.".into());
    }
    use std::io::Read;
    let mut existing = Vec::new();
    std::fs::File::open(path).map_err(|e| format!("No se puede leer el archivo existente: {e}"))?
        .take((crate::savefile::MAX_PACKAGE + 1) as u64).read_to_end(&mut existing).map_err(|e| e.to_string())?;
    if existing != bytes { return Err("Ya existe un archivo distinto con ese nombre. Elige un nombre nuevo; no se ha sobrescrito el archivo existente.".into()); }
    Ok(true)
}

#[cfg(windows)]
mod transport {
    use super::*;
    use std::ffi::c_void;
    type Handle = *mut c_void;
    #[link(name = "winhttp")]
    unsafe extern "system" {
        fn WinHttpOpen(agent: *const u16, access: u32, proxy: *const u16, bypass: *const u16, flags: u32) -> Handle;
        fn WinHttpCloseHandle(handle: Handle) -> i32;
        fn WinHttpSetTimeouts(handle: Handle, resolve: i32, connect: i32, send: i32, receive: i32) -> i32;
        fn WinHttpConnect(session: Handle, server: *const u16, port: u16, reserved: u32) -> Handle;
        fn WinHttpOpenRequest(connection: Handle, verb: *const u16, object: *const u16, version: *const u16, referer: *const u16, types: *const *const u16, flags: u32) -> Handle;
        fn WinHttpSetOption(handle: Handle, option: u32, buffer: *const c_void, length: u32) -> i32;
        fn WinHttpSendRequest(request: Handle, headers: *const u16, header_len: u32, optional: *const c_void, optional_len: u32, total: u32, context: usize) -> i32;
        fn WinHttpReceiveResponse(request: Handle, reserved: *mut c_void) -> i32;
        fn WinHttpQueryHeaders(request: Handle, level: u32, name: *const u16, buffer: *mut c_void, length: *mut u32, index: *mut u32) -> i32;
        fn WinHttpReadData(request: Handle, buffer: *mut c_void, size: u32, read: *mut u32) -> i32;
    }
    struct Owned(Handle);
    impl Owned {
        fn new(h: Handle) -> Result<Self, String> { if h.is_null() { Err(error()) } else { Ok(Self(h)) } }
    }
    impl Drop for Owned { fn drop(&mut self) { unsafe { WinHttpCloseHandle(self.0); } } }
    fn wide(s: &str) -> Vec<u16> { s.encode_utf16().chain(Some(0)).collect() }
    fn error() -> String { format!("HTTPS request failed: {}", std::io::Error::last_os_error()) }
    fn ok(result: i32) -> Result<(), String> { if result == 0 { Err(error()) } else { Ok(()) } }
    pub(super) fn get(url: &str, limit: usize) -> Result<Response, String> {
        let (host, path) = split_allowed(url)?;
        let null = std::ptr::null();
        let start = std::time::Instant::now();
        unsafe {
            let session = Owned::new(WinHttpOpen(wide("Wave-FM-1/0.2.8").as_ptr(), 4, null, null, 0))?;
            ok(WinHttpSetTimeouts(session.0, 5000, 5000, 5000, 5000))?;
            let connection = Owned::new(WinHttpConnect(session.0, wide(host).as_ptr(), 443, 0))?;
            let request = Owned::new(WinHttpOpenRequest(connection.0, wide("GET").as_ptr(), wide(&format!("/{path}")).as_ptr(), null, null, std::ptr::null(), 0x00800000))?;
            // Never follow redirects implicitly or send ambient Windows credentials.
            let never: u32 = 0;
            ok(WinHttpSetOption(request.0, 88, &never as *const _ as _, 4))?;
            let high: u32 = 2;
            ok(WinHttpSetOption(request.0, 77, &high as *const _ as _, 4))?;
            ok(WinHttpSendRequest(request.0, null, 0, std::ptr::null(), 0, 0, 0))?;
            ok(WinHttpReceiveResponse(request.0, std::ptr::null_mut()))?;
            let mut status: u32 = 0;
            let mut length = 4;
            ok(WinHttpQueryHeaders(request.0, 19 | 0x20000000, null, &mut status as *mut _ as _, &mut length, std::ptr::null_mut()))?;
            let location = if [301, 302, 303, 307, 308].contains(&status) {
                let mut value = [0u16; 8193]; let mut length = (value.len() * 2) as u32;
                ok(WinHttpQueryHeaders(request.0, 33, null, value.as_mut_ptr() as _, &mut length, std::ptr::null_mut()))?;
                Some(String::from_utf16_lossy(&value[..(length as usize / 2).min(value.len())]).trim_end_matches('\0').into())
            } else { None };
            let mut body = Vec::new();
            if status == 200 {
                let mut chunk = [0u8; 16384];
                loop {
                    if start.elapsed().as_secs() > 25 { return Err("HTTPS download timed out".into()); }
                    let mut read = 0;
                    ok(WinHttpReadData(request.0, chunk.as_mut_ptr() as _, chunk.len() as u32, &mut read))?;
                    if read == 0 { break; }
                    if body.len() + read as usize > limit { return Err("Download exceeds the size limit".into()); }
                    body.extend_from_slice(&chunk[..read as usize]);
                }
            }
            Ok(Response { status, location, body })
        }
    }
}

#[cfg(target_os = "macos")]
mod transport {
    use super::*;
    use std::io::Read;
    pub(super) fn get(url: &str, limit: usize) -> Result<Response, String> {
        split_allowed(url)?;
        let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
        let header = dir.path().join("headers"); let body_path = dir.path().join("body");
        // macOS's built-in curl, with no shell, user config, implicit redirects or credentials.
        let output = std::process::Command::new("/usr/bin/curl")
            .args(["--disable", "--proto", "=https", "--silent", "--show-error", "--connect-timeout", "5", "--max-time", "25", "--max-filesize"])
            .arg(limit.to_string()).arg("--dump-header").arg(&header).arg("--output").arg(&body_path).arg("--url").arg(url)
            .output().map_err(|e| e.to_string())?;
        if !output.status.success() { return Err("HTTPS download failed or exceeded its size/time limit".into()); }
        let mut headers = String::new();
        std::fs::File::open(header).map_err(|e| e.to_string())?.take(65537).read_to_string(&mut headers).map_err(|e| e.to_string())?;
        if headers.len() > 65536 { return Err("HTTP headers too large".into()); }
        let mut status = 0; let mut location = None;
        for line in headers.lines() {
            if line.starts_with("HTTP/") {
                status = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).ok_or("Invalid HTTP status")?;
                location = None;
            } else if let Some((key, value)) = line.split_once(':') {
                if key.eq_ignore_ascii_case("location") { location = Some(value.trim().to_owned()); }
            }
        }
        let mut body = Vec::new();
        std::fs::File::open(body_path).map_err(|e| e.to_string())?.take((limit + 1) as u64).read_to_end(&mut body).map_err(|e| e.to_string())?;
        Ok(Response { status, location, body })
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
mod transport {
    use super::*;
    pub(super) fn get(_: &str, _: usize) -> Result<Response, String> { Err("Online downloads require Windows or macOS".into()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hub_json_is_parsed_as_data_with_credited_authors() {
        let entries = parse_catalogue(br#"{"firmwares":[{"id":"test","name":"Test","version":1,"author":"Original author","repo":"author/project","pkg":"fw/test-1.fwsc"}]}"#).unwrap();
        assert_eq!(entries[0].author, "Original author");
        assert_eq!(entries[0].version, "1");
        assert!(downloadable(&entries[0]));
        assert_eq!(offline().len(), 96);
    }
    #[test]
    fn malformed_large_and_duplicate_catalogues_are_rejected() {
        assert!(parse_catalogue(b"<html>blocked</html>").is_err());
        assert!(parse_catalogue(&vec![b' '; MAX_CATALOGUE + 1]).is_err());
        assert!(parse_catalogue(br#"{"firmwares":[{"id":"a","name":"A"},{"id":"a","name":"B"}]}"#).is_err());
    }
    #[test]
    fn redirects_and_addresses_cannot_escape_the_public_allowlist() {
        for url in ["http://thefirmwarehub.com/fw/a.fwsc", "https://localhost/fw/a.fwsc", "https://thefirmwarehub.com.evil.test/fw/a.fwsc", "https://thefirmwarehub.com:443/fw/a.fwsc", "https://thefirmwarehub.com/fw/../a.fwsc", "https://thefirmwarehub.com/fw/a.exe", "https://thefirmwarehub.com/fw/%2e%2e/a.fwsc"] {
            assert!(split_allowed(url).is_err(), "{url}");
        }
        let mut calls = 0;
        let result = fetch_with(CATALOGUE_URL, 1024, |_, _| {
            calls += 1;
            Ok(Response { status: 302, location: Some("https://127.0.0.1/internal".into()), body: Vec::new() })
        });
        assert!(result.is_err()); assert_eq!(calls, 1);
        assert!(fetch_with(CATALOGUE_URL, 2, |_, _| Ok(Response { status: 200, location: None, body: vec![0; 3] })).is_err());
        assert!(fetch_with(CATALOGUE_URL, 2, |_, _| Ok(Response { status: 403, location: None, body: Vec::new() })).unwrap_err().contains("403"));
    }
    #[test]
    fn only_valid_fwsc_data_is_saved_without_overwriting_files() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("firmware.fwsc");
        assert!(save(b"not a firmware", &path).is_err()); assert!(!path.exists());
        let bytes = include_bytes!("../../../reference/firmwares/felucca.fwsc");
        assert!(save(bytes, &directory.path().join("file.exe")).is_err());
        let downloaded = save(bytes, &path).unwrap();
        assert_eq!(downloaded.sha256, format!("{:x}", Sha256::digest(bytes)));
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert!(save(bytes, &path).unwrap().reused);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn melodee_main_download_uses_the_author_when_hub_is_blocked() {
        let mut entry = offline().into_iter().find(|e| e.id == "melodee").unwrap();
        // Redistributable fixture exercises download/save; the explicit network
        // test below fetches the actual original Melodee package from its author.
        let bytes = include_bytes!("../../../reference/firmwares/felucca.fwsc");
        entry.alternate_sha256 = format!("{:x}", Sha256::digest(bytes));
        let directory = tempfile::tempdir().unwrap(); let path = directory.path().join("melodee.fwsc");
        let mut urls = Vec::new();
        let downloaded = download_with(&entry, &path, |url, _| {
            urls.push(url.to_owned());
            if url.starts_with(HUB) { Ok(Response { status: 403, location: None, body: Vec::new() }) }
            else { Ok(Response { status: 200, location: None, body: bytes.to_vec() }) }
        }).unwrap();
        assert_eq!(urls, [entry.alternate_package.clone()]);
        assert_eq!(downloaded.source, entry.alternate_package);
        assert_eq!(downloaded.version, "v1.4.5");
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn fallback_preserves_version_and_hash_and_does_not_bypass_integrity() {
        let mut entry = offline().into_iter().find(|e| e.id == "melodee").unwrap();
        let bytes = include_bytes!("../../../reference/firmwares/felucca.fwsc");
        entry.alternate_sha256 = format!("{:x}", Sha256::digest(bytes));
        let directory = tempfile::tempdir().unwrap(); let path = directory.path().join("firmware.fwsc");
        let result = download_with(&entry, &path, |url, _| {
            if url == entry.alternate_package { Err("author temporarily unavailable".into()) }
            else { Ok(Response { status: 200, location: None, body: bytes.to_vec() }) }
        }).unwrap();
        assert_eq!(result.source, entry.package);
        std::fs::remove_file(&path).unwrap();
        let mut calls = 0;
        assert!(download_with(&entry, &path, |_, _| {
            calls += 1; Ok(Response { status: 200, location: None, body: b"modified release".to_vec() })
        }).unwrap_err().contains("SHA-256"));
        assert_eq!(calls, 1); assert!(!path.exists());
        entry.version = "different-version".into(); calls = 0;
        assert!(download_with(&entry, &path, |_, _| { calls += 1; Err("offline".into()) }).is_err());
        assert_eq!(calls, 1);
    }

    #[test]
    fn an_existing_different_file_is_preserved_and_refresh_keeps_author_links() {
        let bytes = include_bytes!("../../../reference/firmwares/felucca.fwsc");
        let directory = tempfile::tempdir().unwrap(); let path = directory.path().join("existing.fwsc");
        let mut other = bytes.to_vec(); other[0] ^= 1; std::fs::write(&path, &other).unwrap();
        assert!(save(bytes, &path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), other);
        let entries = parse_catalogue(br#"{"firmwares":[{"id":"melodee","name":"Melodee","version":"1.4.5","repo":"https://github.com/KEREMIMO/MELODEE/","pkg":"fw/melodee-1.4.5.fwsc"}]}"#).unwrap();
        assert_eq!(preferred(&entries[0]).package, offline().iter().find(|e| e.id == "melodee").unwrap().alternate_package);
    }

    #[test]
    fn pinned_author_alternatives_have_valid_addresses_and_hashes() {
        let entries = offline();
        let alternatives: Vec<_> = entries.iter().filter_map(alternative).collect();
        assert_eq!(alternatives.len(), 78);
        for entry in alternatives {
            assert!(entry.package.starts_with("https://github.com/"));
            assert_eq!(entry.alternate_sha256.len(), 64);
            assert!(entry.alternate_sha256.bytes().all(|b| b.is_ascii_hexdigit()));
        }
        assert!(split_allowed("https://release-assets.githubusercontent.com/github-production-release-asset/1/file?sp=r&sig=public-reference").is_ok());
    }

    #[test]
    #[ignore = "Explicit opt-in network test; native release CI runs it separately"]
    fn author_release_download_uses_real_https() {
        let directory = tempfile::tempdir().unwrap();
        for id in ["felucca", "melodee"] {
            let entry = offline().into_iter().find(|e| e.id == id).unwrap();
            let path = directory.path().join(format!("{id}.fwsc"));
            // Pass the ORIGINAL browser entry, exactly as the primary UI button does.
            let result = download(&entry, &path).expect("real primary-button HTTPS download failed");
            assert_eq!(result.source, entry.alternate_package);
            assert_eq!(result.sha256, entry.alternate_sha256);
            assert!(path.is_file());
            eprintln!("{id}: primary download saved {} bytes from the author; SHA-256 verified", std::fs::metadata(&path).unwrap().len());
        }
    }
}
