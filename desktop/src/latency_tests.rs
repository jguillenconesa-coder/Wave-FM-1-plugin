//! Actual plugin initialization and engine boot/clear, beyond arithmetic plans.
use super::*;
use crate::audio_timing::Reserve;
use std::{cell::Cell, time::{Duration, Instant}};

#[derive(Default)]
struct Host { latency: Cell<u32>, calls: Cell<usize> }
impl InitContext<Fm1> for Host {
    fn plugin_api(&self) -> PluginApi { PluginApi::Clap }
    fn execute(&self, _: ()) {}
    fn set_latency_samples(&self, samples: u32) {
        self.latency.set(samples); self.calls.set(self.calls.get() + 1);
    }
    fn set_current_voice_capacity(&self, _: u32) { panic!("not a polyphonic host request") }
}

fn initialize(plugin: &mut Fm1, host: &mut Host) -> bool {
    plugin.initialize(&Fm1::AUDIO_IO_LAYOUTS[0], &BufferConfig {
        sample_rate: 48000.0, min_buffer_size: Some(1), max_buffer_size: 512,
        process_mode: ProcessMode::Realtime,
    }, host)
}

#[test]
fn a_fresh_instance_reports_zero_even_with_the_legacy_safety_reserve() {
    let mut plugin = Fm1::default(); let mut host = Host::default();
    assert!(initialize(&mut plugin, &mut host));
    assert_eq!(host.latency.get(), 0);
    assert_eq!(plugin.lead, 1495);
    assert_eq!(plugin.params.audio_diagnostics.latency(), 0);
    assert_eq!(plugin.update_audio_timing(false, false, Reserve::Low), None);
    assert_eq!(plugin.lead, 535);
    assert_eq!(plugin.params.audio_diagnostics.requested_lead(), 535);
    // Loaded audio has a real, nonzero delay; never hide it from compensation.
    assert_eq!(plugin.update_audio_timing(false, true, Reserve::Low), Some(582));
    assert_eq!(plugin.update_audio_timing(false, true, Reserve::Low), None);
    assert_eq!(plugin.update_audio_timing(true, true, Reserve::Safe), None);
    assert_eq!(plugin.lead, 535, "defer reserve changes while transport plays");
    assert_eq!(plugin.update_audio_timing(false, true, Reserve::Safe), Some(1627));
    assert_eq!(plugin.update_audio_timing(false, false, Reserve::Safe), Some(0));
    assert_eq!(plugin.update_audio_timing(false, false, Reserve::Safe), None);
}

#[test]
fn real_firmware_load_and_empty_state_change_the_active_latency_lifecycle() {
    let mut plugin = Fm1::default(); let mut host = Host::default();
    assert!(initialize(&mut plugin, &mut host));
    let directory = std::env::var_os("FM1_CATALOGUE_TEST_DIR").map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../firmware/catalogue"));
    let entry = catalogue::ENTRIES.iter().find(|entry| entry.id == "felucca").unwrap();
    let firmware = catalogue::read_at(&directory, entry).unwrap();
    let link = plugin.link.as_ref().unwrap().clone();
    let (reply, answer) = mpsc::channel();
    link.cmds.send(Cmd::Load { firmware, sectors: Vec::new(), reply }).unwrap();
    answer.recv_timeout(Duration::from_secs(10)).unwrap().unwrap();
    assert!(plugin.params.audio_diagnostics.loaded());
    assert_eq!(plugin.update_audio_timing(false, true, Reserve::Safe), Some(1627));
    *plugin.params.firmware.lock().unwrap() = Firmware::default();
    plugin.params.flash.lock().unwrap().clear();
    link.boot(&Firmware::default(), &[]);
    let deadline = Instant::now() + Duration::from_secs(5);
    while plugin.params.audio_diagnostics.loaded() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(!plugin.params.audio_diagnostics.loaded());
    assert!(!link.shared.view.lock().unwrap().booted);
    assert_eq!(plugin.update_audio_timing(false, false, Reserve::Safe), Some(0));
    assert!(link.running.lock().unwrap().is_none());
}
