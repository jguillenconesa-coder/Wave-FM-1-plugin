// MIT. Reuses the same real-engine smoke test as the bundled catalogue.
// Usage: check-community INPUT.json OUTPUT.json. Inputs and binaries are not redistributed.
#[path = "../tests/catalogue.rs"]
mod suite;
use fm1_plugin::catalogue::Entry;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Input { id: String, name: String, version: String, file: String, sha256: String, size: usize }
fn static_text(s: String) -> &'static str { Box::leak(s.into_boxed_str()) }
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3, "check-community INPUT.json OUTPUT.json");
    let inputs: Vec<Input> = serde_json::from_slice(&std::fs::read(&args[1]).unwrap()).unwrap();
    assert!(!inputs.is_empty(), "no firmware candidates supplied");
    let mut results = Vec::new();
    for group in inputs.chunks(4) {
        std::thread::scope(|scope| {
            let mut jobs = Vec::new();
            for input in group {
                jobs.push(scope.spawn(move || {
                    let path = Path::new(&input.file);
                    let entry = Entry { id: static_text(input.id.clone()), name: static_text(input.name.clone()),
                        version: static_text(input.version.clone()), author: "See community-index.json",
                        license: "External download; not redistributed", description: "External execution smoke test",
                        file: static_text(path.file_name().unwrap().to_str().unwrap().to_owned()),
                        sha256: static_text(input.sha256.clone()), size: input.size };
                    let result = std::panic::catch_unwind(|| suite::exercise_entry(&entry, path.parent().unwrap()));
                    let error = result.err().map(|p| p.downcast_ref::<String>().cloned()
                        .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or("test panicked".into()));
                    serde_json::json!({ "id": input.id, "version": input.version, "sha256": input.sha256,
                        "size": input.size, "passed_basic_smoke": error.is_none(), "error": error })
                }));
            }
            for job in jobs { results.push(job.join().expect("community test runner failed")); }
        });
        // Save partial evidence too, so a machine interruption cannot masquerade as a completed run.
        std::fs::write(&args[2], serde_json::to_vec_pretty(&results).unwrap()).unwrap();
    }
    let passed = results.iter().filter(|r| r["passed_basic_smoke"] == true).count();
    println!("Community smoke tests completed: {passed}/{} passed basic boot, LCD and audio; this is not full functionality verification.", results.len());
}
