// Derived from fm1-emu by Joakim Eriksson (MIT). Desktop additions by Wave-FM-1-plugin contributors, 2026.
//! The plugin's window: the front panel (the native window's and the page's drawing code) with the
//! machine's screen and LEDs, played with the mouse as on the page (keys and buttons held while
//! pressed, a drag across the keys a glissando, knobs dragged up / down or scrolled, MASTER a pot
//! that is the host's "Master" parameter), and a bar to load a firmware package.

use crate::engine::Cmd;
use crate::{catalogue, controls, online, skins};
use crate::{panel, panel_smk, Fm1Params, Link};
use fm1_soc::Board;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets};
use std::sync::Arc;

/// A pointer's grab: what it hit (fm1-web's panel_hit codes), where it was, what it holds.
struct Grab {
    ctl: i32,
    last_y: f32,
    acc: f32,
    held: i32,
}

struct Ui {
    board: Board,
    base: Vec<u32>,
    buf: Vec<u32>,
    rgba: Vec<u8>,
    tex: Option<egui::TextureHandle>,
    down: [bool; 64],
    enc_clicks: [i32; 8],
    encoder_display: controls::EncoderDisplay,
    led_display: panel::LedDisplay,
    panel_cache: panel::Cached,
    turning: [u32; 9],
    /// SMK-37: its analog inputs as set (fm1_soc::smk's ANALOG_* indices)
    analog: [u16; 23],
    grab: Option<Grab>,
    wheel_acc: f32,
    msg: String,
    msg_until: Option<std::time::Instant>,
    /// the file dialog's answer, while it is open
    picking: Option<(FileAction, Answer)>,
    job: Option<Job>,
    skin: String,
    pending_firmware: Option<usize>,
    mapping: bool,
    browser: bool,
    search: String,
    community: Vec<online::Entry>,
    browser_job: Option<BrowserJob>,
    browser_pick: Option<(online::Entry, Answer)>,
    browser_status: String,
    browser_error: bool,
    downloaded: Option<std::path::PathBuf>,
}

impl Default for Ui {
    fn default() -> Self {
        let mut analog = [0; 23];
        analog[fm1_soc::smk::ANALOG_PITCH as usize] = fm1_soc::smk::PITCH_REST;
        analog[fm1_soc::smk::ANALOG_MOD as usize] = fm1_soc::smk::MOD_REST;
        Ui {
            board: Board::Fm1,
            base: Vec::new(),
            buf: Vec::new(),
            rgba: Vec::new(),
            tex: None,
            down: [false; 64],
            enc_clicks: [0; 8],
            encoder_display: controls::EncoderDisplay::default(),
            led_display: panel::LedDisplay::default(),
            panel_cache: panel::Cached::default(),
            turning: [0; 9],
            analog,
            grab: None,
            wheel_acc: 0.0,
            msg: String::new(),
            msg_until: None,
            picking: None,
            job: None,
            skin: String::new(),
            pending_firmware: None,
            mapping: false,
            browser: false,
            search: String::new(),
            community: online::offline(),
            browser_job: None,
            browser_pick: None,
            browser_status: "Included index · 83 references from fm1-emu; not a live Hub check".into(),
            browser_error: false,
            downloaded: None,
        }
    }
}

const NKEY: i32 = 37;

pub fn create(params: Arc<Fm1Params>, link: Arc<Link>) -> Option<Box<dyn Editor>> {
    // (the state outlives a window: one opened again has a new egui context, so the panel's
    // texture, which belongs to the old one, is made again)
    let build = |_: &egui::Context, ui: &mut Ui| {
        ui.tex = None;
        ui.grab = None;
        ui.encoder_display = controls::EncoderDisplay::default();
        ui.led_display = panel::LedDisplay::default();
        ui.panel_cache = panel::Cached::default();
    };
    create_egui_editor(params.editor_state.clone(), Ui::default(), build, move |ctx, setter, ui| {
        let (lcd, leds, pads, board, booted, halted, speed, late) = {
            let v = link.shared.view.lock().unwrap();
            (v.lcd.clone(), v.leds, v.pads, v.board, v.booted, v.halted, v.speed, v.late)
        };
        let skin_id = params.skin.lock().unwrap().clone();
        let skin = skins::get(&skin_id);
        if board != ui.board || ui.base.is_empty() || ui.skin != skin.id {
            ui.skin = skin.id.into();
            ui.board = board;
            ui.base = if board == Board::Smk37 { panel_smk::base() } else { panel::base_with_palette(&skin.palette) };
            ui.buf = vec![0; ui.base.len()];
            ui.panel_cache = panel::Cached::default();
        }
        let smk = board == Board::Smk37;
        if !smk {
            let changed = ui.encoder_display.sync(std::array::from_fn(|i| params.controls.encoders[i].value.value()));
            for i in 0..7 {
                if changed[i] { ui.turning[i] = 10; }
            }
        }
        let (w, h) = if smk { (panel_smk::W, panel_smk::H) } else { (panel::W, panel::H) };

        egui::TopBottomPanel::top("bar").show(ctx, |bar| {
            bar.horizontal_wrapped(|bar| {
                if ui.msg_until.is_some_and(|until| std::time::Instant::now() >= until) {
                    ui.msg.clear(); ui.msg_until = None;
                }
                if let Some(result) = ui.job.as_ref().and_then(|job| job.lock().unwrap().take()) {
                    ui.job = None;
                    ui.msg_until = result.as_ref().ok().map(|_| std::time::Instant::now() + std::time::Duration::from_secs(3));
                    ui.msg = match result { Ok(message) => message, Err(error) => format!("Firmware / save: {error}") };
                }
                let name = params.firmware.lock().unwrap().name.clone();
                let idle = ui.picking.is_none() && ui.job.is_none() && ui.pending_firmware.is_none();
                let has_firmware = !params.firmware.lock().unwrap().pkg.is_empty();
                let mut selected = None;
                bar.add_enabled_ui(idle, |bar| {
                    egui::ComboBox::from_id_salt("firmware-catalogue").selected_text(if name.is_empty() { "Firmware…" } else { &name })
                        .show_ui(bar, |menu| {
                            menu.label("Included · SHA-256 verified");
                            for (i, entry) in catalogue::ENTRIES.iter().enumerate() {
                                if menu.selectable_label(false, format!("{} {}", entry.name, entry.version))
                                    .on_hover_text(format!("{}\n{} · {}", entry.description, entry.author, entry.license)).clicked() {
                                    selected = Some(i);
                                }
                            }
                            menu.separator();
                            if menu.selectable_label(false, "Open firmware / save file…").clicked() {
                                ui.picking = Some((FileAction::Load, pick(FileAction::Load, &name)));
                            }
                        });
                });
                if let Some(index) = selected {
                    if has_firmware { ui.pending_firmware = Some(index); }
                    else { start_bundled(ui, index, params.clone(), link.clone()); }
                }
                if bar.add_enabled(idle && ui.job.is_none(), egui::Button::new("Load firmware / save…"))
                    .on_hover_text("Open a .fwsc firmware package or a shared .fm1save setup").clicked() {
                    ui.picking = Some((FileAction::Load, pick(FileAction::Load, &name)));
                }
                if bar.add_enabled(idle && has_firmware, egui::Button::new("Export save…"))
                    .on_hover_text("Share the running firmware, patches and settings as a .fm1save file").clicked() {
                    ui.picking = Some((FileAction::Export, pick(FileAction::Export, &name)));
                }
                let chosen = ui.picking.as_ref().and_then(|(action, answer)|
                    answer.lock().unwrap().take().map(|path| (*action, path)));
                if let Some((action, path)) = chosen {
                    ui.picking = None;
                    if let Some(path) = path {
                        ui.msg_until = None;
                        ui.msg = if matches!(action, FileAction::Load) { "Loading save…" } else { "Exporting live flash…" }.into();
                        ui.job = Some(file_job(action, path, params.clone(), link.clone()));
                    }
                }
                bar.label(egui::RichText::new(if name.is_empty() { "no firmware".into() } else { name }).strong());
                bar.separator();
                bar.label("Skin");
                egui::ComboBox::from_id_salt("skin").selected_text(skin.name).show_ui(bar, |menu| {
                    for candidate in skins::SKINS {
                        if menu.selectable_label(skin.id == candidate.id, candidate.name).clicked() {
                            *params.skin.lock().unwrap() = candidate.id.into();
                        }
                    }
                });
                bar.label("MIDI channel");
                bar.add(widgets::ParamSlider::for_param(&params.channel, setter).with_width(90.0));
                bar.label("CPU clock").on_hover_text("Virtual CPU speed, not the audio buffer or musical tempo. Higher MHz can help firmware CPU deadlines but increases this computer's load and may cause audio underruns. Start at 240 MHz when diagnosing host underruns. Changing it restarts the firmware.");
                bar.add(widgets::ParamSlider::for_param(&params.clock, setter).with_width(90.0));
                bar.label("Audio buffer");
                bar.add_enabled(!params.audio_diagnostics.playing(),
                    widgets::ParamSlider::for_param(&params.audio_reserve, setter).with_width(140.0))
                    .on_hover_text("Change while Live is stopped. Low latency reduces the internal reserve but needs more CPU/scheduling headroom; Balanced is intermediate; Safe / legacy keeps the previous reserve. Live may briefly restart audio when compensation changes. Empty instances report zero latency.");
                bar.label("Musical clock");
                bar.add(widgets::ParamSlider::for_param(&params.transport_sync, setter).with_width(165.0))
                    .on_hover_text("Internal / firmware: use the firmware's own tempo. Ableton / DAW: send 24-PPQN MIDI Clock and Start/Stop from the host. Set the firmware's clock input to external TRS/MIDI; USB clock is not the emulated MIDI IN. CPU clock is unrelated to musical tempo.");
                if params.transport_sync.value() == crate::host_sync::Source::Daw {
                    bar.label(params.transport_feedback.label(params.transport_sync.value()));
                    bar.label("Firmware clock: external TRS / MIDI")
                        .on_hover_text("Load and configure the firmware before starting Live. Every DAW start or timeline jump restarts the sequence; firmware support is required. Return its clock setting to Internal when leaving DAW sync.");
                }
                bar.separator();
                bar.toggle_value(&mut ui.mapping, "MIDI / Live")
                    .on_hover_text("Show clearly labelled controls you can expose with Live's Configure button, then map to your MIDI keyboard or controller");
                bar.toggle_value(&mut ui.browser, "Community firmwares…")
                    .on_hover_text("Browse known firmwares, check Firmware Hub for new versions, or download a .fwsc file. No network access until you ask.");
                bar.separator();
                let status = if !ui.msg.is_empty() {
                    ui.msg.clone()
                } else if !booted {
                    "Open a .fwsc firmware or shared .fm1save to start".into()
                } else if halted {
                    "The firmware stopped: an instruction the emulator does not know".into()
                } else {
                    "Firmware running".into()
                };
                bar.label(status);
            });
        });

        // Audio health has its own space: a firmware/download message must not
        // hide the speed needed to distinguish overload from a buffering fault.
        egui::TopBottomPanel::bottom("audio-health").show(ctx, |health| {
            let (rate, samples) = params.audio_diagnostics.host();
            let engine = if speed > 0.0 { format!("{speed:.2}x") } else { "measuring…".into() };
            let cpu = params.clock.value().mhz();
            let latency = params.audio_diagnostics.latency();
            let latency_ms = if rate > 0.0 { latency as f32 * 1000.0 / rate } else { 0.0 };
            health.horizontal_wrapped(|row| {
                row.label(egui::RichText::new(format!("Engine: {engine}")).strong())
                    .on_hover_text("Above 1.00x means average emulation is faster than real time. Brief scheduling delays can still cause late audio. Below 1.00x indicates sustained overload.");
                row.separator();
                let late_label = format!("Audio late: {late} frames/s");
                row.colored_label(if late > 0 { egui::Color32::YELLOW } else { row.visuals().text_color() }, late_label)
                    .on_hover_text("The plugin's own audio queue, not the ASIO driver's error counter. Start-up loading may miss frames. Persistent late audio needs investigation; higher CPU clock MHz increases host load.");
                row.separator();
                row.label(format!("CPU: {cpu} MHz"));
                row.label(if rate > 0.0 { format!("Host: {rate:.0} Hz / {samples} samples") } else { "Host: not processing".into() });
                row.label(format!("Plugin latency: {latency} samples ({latency_ms:.1} ms)"))
                    .on_hover_text("The delay reported for host compensation, separate from the audio interface's input/output buffers. Zero only when no firmware is loaded. Lower reserves can underrun on a busy or slower CPU.");
                row.label(params.audio_diagnostics.scheduling());
                if row.button("Copy audio status").clicked() {
                    let firmware = params.firmware.lock().unwrap().name.clone();
                    ctx.copy_text(format!("FM-1 {} | Firmware: {firmware} | Engine: {engine} | Audio late: {late} frames/s | CPU: {cpu} MHz | Host: {rate:.0} Hz / {samples} samples | Audio buffer: {:?} | Plugin latency: {latency} samples ({latency_ms:.2} ms) | Scheduling: {}",
                        env!("CARGO_PKG_VERSION"), params.audio_reserve.value(), params.audio_diagnostics.scheduling()));
                }
            });
        });

        if let Some(result) = ui.browser_job.as_ref().and_then(|job| job.lock().unwrap().take()) {
            ui.browser_job = None;
            ui.browser_error = result.is_err();
            match result {
                Ok(BrowserResult::Catalogue(entries)) => {
                    let changed = entries.iter().filter(|entry| !ui.community.iter().any(|old|
                        (old.id == entry.id || !old.hub.is_empty() && old.hub == entry.hub) && old.version == entry.version)).count();
                    ui.browser_status = format!("Hub checked · {} entries · {changed} new or changed since the displayed index. Downloads are not functionally verified.", entries.len());
                    ui.community = entries;
                }
                Ok(BrowserResult::Download(download)) => {
                    ui.browser_status = format!("{} · {}\nArchivo: {}\nOrigen: {}\nSHA-256: {}\nPara usarlo, pulsa «Load downloaded firmware into this instance…».",
                        if download.reused { "El archivo ya estaba descargado y se ha verificado" } else { "Descarga guardada correctamente" },
                        download.version, download.path.display(), download.source, download.sha256);
                    ui.downloaded = Some(download.path);
                }
                Err(error) => ui.browser_status = error,
            }
        }
        let chosen = ui.browser_pick.as_ref().and_then(|(entry, answer)|
            answer.lock().unwrap().take().map(|path| (entry.clone(), path)));
        if let Some((entry, path)) = chosen {
            ui.browser_pick = None;
            if let Some(mut path) = path {
                if path.extension().is_none() { path.set_extension("fwsc"); }
                let job: BrowserJob = Default::default(); let result = job.clone();
                ui.browser_job = Some(job);
                let target = online::preferred(&entry);
                ui.browser_error = false;
                ui.browser_status = format!("Descargando {} {} desde {}…", target.name, target.version, online::provider(&target));
                link.background(move || { *result.lock().unwrap() = Some(online::download(&entry, &path).map(BrowserResult::Download)); });
            } else { ui.browser_error = false; ui.browser_status = "Descarga cancelada: no se ha guardado ningún archivo.".into(); }
        }
        if ui.browser {
            let mut open = true;
            egui::Window::new("Community firmwares · Firmware Hub").open(&mut open).default_width(640.0)
                .show(ctx, |browser| {
                    browser.label("23 bundled/tested firmwares remain in Firmware. This explorer lists additional references; a listing does not verify functionality or redistribution rights.");
                    browser.horizontal(|row| {
                        let idle = ui.browser_job.is_none() && ui.browser_pick.is_none();
                        if row.add_enabled(idle, egui::Button::new("Check Hub for new firmwares")).clicked() {
                            let job: BrowserJob = Default::default(); let result = job.clone();
                            ui.browser_job = Some(job); ui.browser_status = "Checking https://thefirmwarehub.com/catalog.json…".into();
                            ui.browser_error = false;
                            link.background(move || { *result.lock().unwrap() = Some(online::check().map(BrowserResult::Catalogue)); });
                        }
                        row.hyperlink_to("Open Firmware Hub", "https://thefirmwarehub.com/#play");
                    });
                    if ui.browser_job.is_some() {
                        browser.horizontal(|row| { row.spinner(); row.label("Operación en curso…"); });
                    }
                    browser.colored_label(if ui.browser_error { egui::Color32::LIGHT_RED } else { egui::Color32::LIGHT_GREEN }, &ui.browser_status);
                    browser.small("Downloads contact the displayed provider only on request. They save a new .fwsc file where you choose; existing files are not overwritten and open Sets are not updated.");
                    if let Some(path) = &ui.downloaded {
                        if browser.add_enabled(ui.job.is_none() && ui.picking.is_none(), egui::Button::new("Load downloaded firmware into this instance…"))
                            .on_hover_text("Replaces this instance's firmware and patches. Export the current setup first. Downloaded packages have not passed the bundled catalogue's audio tests.").clicked() {
                            ui.job = Some(file_job(FileAction::Load, path.clone(), params.clone(), link.clone()));
                        }
                    }
                    browser.separator();
                    browser.horizontal(|row| { row.label("Search"); row.text_edit_singleline(&mut ui.search); });
                    let query = ui.search.to_lowercase();
                    egui::ScrollArea::vertical().max_height(300.0).show(browser, |list| {
                        for entry in &ui.community {
                            if !format!("{} {} {}", entry.name, entry.author, entry.version).to_lowercase().contains(&query) { continue; }
                            list.group(|card| {
                                card.label(egui::RichText::new(format!("{} {} · {}", entry.name, entry.version, entry.author)).strong());
                                if let Some(bundled) = catalogue::ENTRIES.iter().find(|b| b.id == entry.id || !entry.hub.is_empty() && b.id == entry.hub) {
                                    card.small(format!("Bundled/tested: {} · {}", bundled.version, bundled.license));
                                } else { card.small("External firmware · not included or functionally tested by this plugin"); }
                                card.small(format!("Reported terms: {}", entry.license));
                                if let Some(passed) = entry.alternate_smoke {
                                    card.small(format!("GitHub {}: {}", entry.alternate_version,
                                        if passed { "basic boot/LCD/8-second audio passed; not full verification" }
                                        else { "no audio in the generic smoke test; not functionally verified" }));
                                }
                                card.horizontal_wrapped(|row| {
                                    let idle = ui.browser_job.is_none() && ui.browser_pick.is_none();
                                    let target = online::preferred(entry);
                                    if row.add_enabled(idle && online::downloadable(&target), egui::Button::new("Download .fwsc…"))
                                        .on_hover_text(format!("{} {} · {}\n{}\nReview the author's terms. Saving does not load firmware automatically.", target.name, target.version, online::provider(&target), target.package)).clicked() {
                                        ui.browser_error = false;
                                        ui.downloaded = None;
                                        ui.browser_status = format!("Elige dónde guardar {} {}. Si cancelas la ventana no se descargará ningún archivo.", target.name, target.version);
                                        ui.browser_pick = Some((entry.clone(), pick(FileAction::Download, &format!("{}-{}", entry.id, target.version))));
                                    }
                                    row.label(format!("{} · {}", online::provider(&target), target.version));
                                    if online::downloadable(&target) {
                                        row.hyperlink_to("Download in browser", &target.package)
                                            .on_hover_text("Alternativa si Windows, el proxy o el servidor bloquean la conexión del plugin. Después usa Load firmware / save… para abrir el archivo descargado.");
                                    }
                                    if entry.repo.starts_with("https://github.com/") { row.hyperlink_to("Author / terms", &entry.repo); }
                                    if !entry.hub.is_empty() { row.hyperlink_to("Hub card", format!("https://thefirmwarehub.com/#fw-{}", entry.hub)); }
                                });
                            });
                        }
                    });
                });
            ui.browser = open;
        }

        if ui.mapping {
            egui::SidePanel::right("midi-live").exact_width(265.0).resizable(false).show(ctx, |map| {
                egui::ScrollArea::vertical().show(map, |map| {
                    map.heading("MIDI / Live");
                    map.label("Assignable host controls");
                    map.collapsing("How to assign in Ableton Live", |help| {
                        help.label("1. In Live's device, click Configure.");
                        help.label("2. Move a slider or click a button here.");
                        help.label("3. Turn Configure off. Enable MIDI Map (Ctrl+M / Cmd+M).");
                        help.label("4. Select the new parameter in Live and move your controller. Exit MIDI Map.");
                        help.label("Enable Remote for that MIDI input in Live's MIDI settings. For encoders, use absolute CC (0–127); for buttons, momentary 127/0.");
                    });
                    map.separator();
                    map.label(egui::RichText::new("Master").strong());
                    map.add(widgets::ParamSlider::for_param(&params.master, setter).with_width(240.0))
                        .on_hover_text("Master · absolute level · the same parameter as the panel's MASTER pot");
                    map.add_enabled_ui(!smk, |map| {
                        map.label("Encoders · relative reference 0–127")
                            .on_hover_text("A change of one step sends one encoder click. The panel indicators follow these MIDI references over a 270° sweep; they are not firmware parameter values. Mouse turns remain endless.");
                        // Put the four performance knobs first, with names matching Live exactly.
                        for i in [3, 4, 5, 6, 0, 1, 2] {
                            let param = &params.controls.encoders[i].value;
                            map.horizontal(|row| {
                                row.add_sized([78.0, 18.0], egui::Label::new(egui::RichText::new(param.name()).strong()));
                                row.add(widgets::ParamSlider::for_param(param, setter).with_width(155.0))
                                    .on_hover_text(format!("{} · assign this slider in Live's Configure view. Its musical function depends on the loaded firmware.", param.name()));
                            });
                        }
                        map.separator();
                        map.label("Buttons · 100 ms trigger")
                            .on_hover_text("Each Off → On change sends a short press and releases it automatically, even if the controller stays On. Use the physical panel for long presses.");
                        egui::Grid::new("midi-buttons").num_columns(2).spacing([6.0, 5.0]).show(map, |grid| {
                            for (i, button) in params.controls.buttons.iter().enumerate() {
                                let response = grid.add_sized([117.0, 27.0], egui::Button::new(controls::BUTTONS[i]))
                                    .on_hover_text(format!("Host name: {}\nConfigure: click here. Controller: momentary 127/0. The action depends on the firmware.", button.value.name()));
                                if response.clicked() {
                                    setter.begin_set_parameter(&button.value);
                                    setter.set_parameter(&button.value, false);
                                    setter.set_parameter(&button.value, true);
                                    setter.set_parameter(&button.value, false);
                                    setter.end_set_parameter(&button.value);
                                }
                                if i % 2 == 1 { grid.end_row(); }
                            }
                        });
                    });
                    if smk { map.label("FM-1 encoders/buttons are disabled for SMK-37 firmware."); }
                    map.separator();
                    map.small("Names identify physical controls. Their sound function depends on the firmware and the current screen. MIDI notes still play through the track as usual.");
                });
            });
        }

        if let Some(index) = ui.pending_firmware {
            egui::Window::new("Replace firmware in this instance?").collapsible(false).resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]).show(ctx, |dialog| {
                    dialog.label(format!("Load {} {}?", catalogue::ENTRIES[index].name, catalogue::ENTRIES[index].version));
                    dialog.label("This replaces this instance's firmware and patches with the factory setup. Export your current setup first if you want to keep it.");
                    dialog.horizontal(|buttons| {
                        if buttons.button("Cancel").clicked() { ui.pending_firmware = None; }
                        if buttons.button("Load firmware").clicked() {
                            ui.pending_firmware = None;
                            start_bundled(ui, index, params.clone(), link.clone());
                        }
                    });
                });
        }

        egui::CentralPanel::default().frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(0x18, 0x19, 0x1b))).show(ctx, |c| {
            let avail = c.available_rect_before_wrap();
            let k = (avail.width() / w as f32).min(avail.height() / h as f32);
            let rect = egui::Rect::from_center_size(avail.center(), egui::vec2(w as f32 * k, h as f32 * k));
            let resp = c.allocate_rect(rect, egui::Sense::click_and_drag());
            let at = |p: egui::Pos2| ((p.x - rect.min.x) / k, (p.y - rect.min.y) / k);

            // the pointer
            let (pressed, released, pos, scroll) =
                ctx.input(|i| (i.pointer.primary_pressed(), i.pointer.primary_released(), i.pointer.interact_pos(), i.raw_scroll_delta.y));
            if pressed && resp.hovered() {
                if let Some(p) = pos {
                    let (x, y) = at(p);
                    let ctl = hit(smk, x, y);
                    let mut g = Grab { ctl, last_y: y, acc: 0.0, held: -1 };
                    if (100..200).contains(&ctl) {
                        g.held = ctl - 100;
                        if smk && g.held < NKEY {
                            velocity(&link, panel_smk_hit_y(x, y));
                        }
                        key(ui, &link, g.held, true);
                    } else if smk && ctl >= 200 {
                        slide(ui, &link, ctl, y);
                    } else if !smk && ctl == 7 {
                        setter.begin_set_parameter(&params.master);
                    }
                    ui.grab = Some(g);
                }
            }
            if let (Some(g), Some(p)) = (ui.grab.as_mut(), pos) {
                let (x, y) = at(p);
                let keys = if smk { 0..NKEY } else { 14..41 };
                if (100..200).contains(&g.ctl) && keys.contains(&g.held) {
                    let h = hit(smk, x, y);
                    if keys.contains(&(h - 100)) && h - 100 != g.held {
                        let old = g.held;
                        g.held = h - 100;
                        let new = g.held;
                        key(ui, &link, old, false);
                        if smk {
                            velocity(&link, panel_smk_hit_y(x, y));
                        }
                        key(ui, &link, new, true);
                    }
                } else if smk && g.ctl >= 300 {
                    let ctl = g.ctl;
                    slide(ui, &link, ctl, y);
                } else if g.ctl >= 0 && g.ctl < if smk { 8 } else { 7 } {
                    // 12 px of drag a click
                    g.acc += g.last_y - y;
                    let clicks = (g.acc / 12.0).trunc();
                    g.acc -= clicks * 12.0;
                    let ctl = g.ctl;
                    if clicks != 0.0 {
                        turn(ui, &link, smk, ctl, clicks as i32);
                    }
                } else if !smk && g.ctl == 7 {
                    let v = (params.master.value() + (g.last_y - y) * 4.0 / 1023.0).clamp(0.0, 1.0);
                    setter.set_parameter(&params.master, v);
                    ui.turning[7] = 10;
                }
                if let Some(g) = ui.grab.as_mut() {
                    g.last_y = y;
                }
            }
            if released {
                if let Some(g) = ui.grab.take() {
                    if g.held >= 0 {
                        key(ui, &link, g.held, false);
                    }
                    if smk && (200..216).contains(&g.ctl) {
                        analog(ui, &link, (g.ctl - 200) as usize, 0); // a pad let go
                    }
                    if smk && g.ctl == 400 {
                        analog(ui, &link, fm1_soc::smk::ANALOG_PITCH as usize, fm1_soc::smk::PITCH_REST); // springs back
                    }
                    if !smk && g.ctl == 7 {
                        setter.end_set_parameter(&params.master);
                    }
                }
            }
            if scroll != 0.0 && resp.hovered() {
                if let Some(p) = pos {
                    let (x, y) = at(p);
                    let h = hit(smk, x, y);
                    if smk && (300..304).contains(&h) {
                        let i = 16 + (h - 300) as usize;
                        analog(ui, &link, i, (ui.analog[i] as f32 + scroll * 2.0).clamp(0.0, 1023.0) as u16);
                    } else if !smk && h == 7 {
                        let v = (params.master.value() + scroll * 0.5 / 1023.0).clamp(0.0, 1.0);
                        setter.begin_set_parameter(&params.master);
                        setter.set_parameter(&params.master, v);
                        setter.end_set_parameter(&params.master);
                    } else if (0..if smk { 8 } else { 7 }).contains(&h) {
                        ui.wheel_acc += scroll;
                        while ui.wheel_acc >= 40.0 {
                            ui.wheel_acc -= 40.0;
                            turn(ui, &link, smk, h, 1);
                        }
                        while ui.wheel_acc <= -40.0 {
                            ui.wheel_acc += 40.0;
                            turn(ui, &link, smk, h, -1);
                        }
                    }
                }
            }

            // the panel
            let panel_changed = if smk {
                let a = &ui.analog;
                let st = panel_smk::State {
                    lcd: &lcd,
                    enc_clicks: &ui.enc_clicks,
                    turning: &std::array::from_fn(|i| ui.turning[i]),
                    down: &ui.down,
                    leds: &std::array::from_fn(|i| leds[i]),
                    pads: &pads,
                    pad_force: &std::array::from_fn(|p| a[p]),
                    faders: &std::array::from_fn(|f| a[16 + f]),
                    pitch: a[fm1_soc::smk::ANALOG_PITCH as usize],
                    modw: a[fm1_soc::smk::ANALOG_MOD as usize],
                };
                panel_smk::draw(&mut ui.buf, &ui.base, &st);
                true
            } else {
                let stable_leds = ui.led_display.update(&leds);
                let st = panel::State {
                    lcd: &lcd,
                    enc_clicks: &std::array::from_fn(|i| ui.enc_clicks[i]),
                    pot: (params.master.value() * 1023.0).round() as u16,
                    down: &std::array::from_fn(|i| ui.down[i]),
                    leds: &stable_leds,
                    turning: &std::array::from_fn(|i| ui.turning[i]),
                };
                ui.panel_cache.draw(&mut ui.buf, &ui.base, &st, &skin.palette, &ui.encoder_display.angles)
            };
            for t in ui.turning.iter_mut() {
                *t = t.saturating_sub(1);
            }
            if panel_changed || ui.tex.is_none() {
                ui.rgba.clear();
                ui.rgba.extend(ui.buf.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]));
                let img = egui::ColorImage::from_rgba_unmultiplied([w, h], &ui.rgba);
                match ui.tex.as_mut() {
                    Some(t) if t.size() == [w, h] => t.set(img, egui::TextureOptions::LINEAR),
                    _ => ui.tex = Some(ctx.load_texture("panel", img, egui::TextureOptions::LINEAR)),
                }
            }
            if let Some(t) = &ui.tex {
                c.painter().image(t.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
            }
            if !smk {
                if ui.mapping { mapping_labels(c.painter(), rect, k); }
                if resp.hovered() {
                    if let Some(p) = pos {
                        let (x, y) = at(p);
                        if let Some(name) = control_name(hit(false, x, y)) {
                            resp.on_hover_text(format!("{name}\nMIDI / Live: find this name in the assignable-controls view. The action depends on your firmware."));
                        }
                    }
                }
            }
        });
        ctx.request_repaint_after(std::time::Duration::from_millis(33));
    })
}

fn control_name(ctl: i32) -> Option<String> {
    match ctl {
        0..=6 => Some(controls::ENCODERS[ctl as usize].0.into()),
        7 => Some("Master".into()),
        100..=113 => Some(format!("{} button", controls::BUTTONS[(ctl - 100) as usize])),
        _ => None,
    }
}

// Readable egui text stays at a useful point size even when the panel texture scales down.
fn mapping_labels(painter: &egui::Painter, rect: egui::Rect, scale: f32) {
    let accent = egui::Color32::from_rgb(90, 225, 245);
    let point = |x, y| rect.min + egui::vec2(x * scale, y * scale);
    let label = |at: egui::Pos2, name: &str| {
        let galley = painter.layout_no_wrap(name.into(), egui::FontId::proportional(11.0), accent);
        let bounds = egui::Rect::from_center_size(at, galley.size() + egui::vec2(6.0, 3.0));
        painter.rect_filled(bounds, 3.0, egui::Color32::from_rgb(14, 22, 26));
        painter.galley(bounds.min + egui::vec2(3.0, 1.5), galley, accent);
    };
    for (i, &(_, _, x, y, radius)) in panel::ENCS.iter().enumerate() {
        painter.circle_stroke(point(x, y), (radius + 3.0) * scale, egui::Stroke::new(1.5_f32, accent));
        label(point(x, y - radius - 23.0), controls::ENCODERS[i].0);
    }
    let (x, y, radius) = panel::POT;
    painter.circle_stroke(point(x, y), (radius + 3.0) * scale, egui::Stroke::new(1.5_f32, accent));
    label(point(x, y - radius - 23.0), "Master");
    for (_, id, x, y, w, h) in panel::buttons() {
        let bounds = egui::Rect::from_min_size(point(x as f32, y as f32), egui::vec2(w as f32 * scale, h as f32 * scale));
        painter.rect_stroke(bounds, 3.0, egui::Stroke::new(1.0_f32, accent), egui::StrokeKind::Middle);
        label(bounds.center(), controls::BUTTONS[id as usize]);
    }
}

fn start_bundled(ui: &mut Ui, index: usize, params: Arc<Fm1Params>, link: Arc<Link>) {
    let entry = &catalogue::ENTRIES[index];
    ui.msg = format!("Loading {} {}…", entry.name, entry.version);
    ui.msg_until = None;
    let job: Job = Default::default();
    let result = job.clone();
    ui.job = Some(job);
    link.clone().background(move || {
        let run = || -> Result<String, String> {
            let firmware = catalogue::read(entry)?;
            let (tx, rx) = std::sync::mpsc::channel();
            link.cmds.send(Cmd::Load { firmware: firmware.clone(), sectors: Vec::new(), reply: tx })
                .map_err(|_| "the machine stopped".to_owned())?;
            rx.recv().map_err(|_| "the machine stopped while loading".to_owned())??;
            // engine::run commits params and flash only after a successful boot.
            *link.running.lock().unwrap() = Some(firmware.pkg);
            Ok(format!("Loaded {} {} · {}", entry.name, entry.version, entry.author))
        };
        *result.lock().unwrap() = Some(run());
        drop(params);
    });
}

/// Ask for a package. The dialog runs modally, so not from inside the window's frame: macOS
/// sends the window events while it is open (a mouse leaving it), and the window's code is still
/// in the frame then (in GarageBand's AU host that aborts). On macOS it opens from the main queue
/// once the frame is done; the answer lands in the returned slot (a path, or None if cancelled).
type Answer = Arc<std::sync::Mutex<Option<Option<std::path::PathBuf>>>>;
type Job = Arc<std::sync::Mutex<Option<Result<String, String>>>>;
enum BrowserResult { Catalogue(Vec<online::Entry>), Download(online::Download) }
type BrowserJob = Arc<std::sync::Mutex<Option<Result<BrowserResult, String>>>>;
#[derive(Clone, Copy)]
enum FileAction { Load, Export, Download }

fn pick(action: FileAction, name: &str) -> Answer {
    let answer: Answer = Default::default();
    let slot = answer.clone();
    let name: String = name.chars().take(100).map(|c|
        if c.is_alphanumeric() || " _-".contains(c) { c } else { '_' }).collect();
    let ask = move || {
        let path = match action {
            FileAction::Load => rfd::FileDialog::new()
                .add_filter("FM-1 firmware or shared save", &["fwsc", "fm1save", "json"]).pick_file(),
            FileAction::Export => rfd::FileDialog::new().add_filter("FM-1 shared save", &["fm1save"])
                .set_file_name(format!("{}.fm1save", if name.is_empty() { "FM-1" } else { &name })).save_file(),
            FileAction::Download => rfd::FileDialog::new().set_title("Save community firmware · choose a new filename")
                .add_filter("FM-1 firmware", &["fwsc"]).set_file_name(format!("{name}.fwsc")).save_file(),
        };
        *slot.lock().unwrap() = Some(path);
    };
    #[cfg(target_os = "macos")]
    dispatch2::DispatchQueue::main().exec_async(ask);
    #[cfg(not(target_os = "macos"))]
    std::thread::spawn(ask);
    answer
}

// File I/O, hashing and serialization never run in the audio callback or window frame.
// The machine acknowledges imports only after a validated boot has replaced the old setup.
fn file_job(action: FileAction, mut path: std::path::PathBuf, params: Arc<Fm1Params>, link: Arc<Link>) -> Job {
    let job: Job = Default::default();
    let result = job.clone();
    link.clone().background(move || {
        if matches!(action, FileAction::Export) && path.extension().is_none() { path.set_extension("fm1save"); }
        let run = || -> Result<String, String> {
            match action {
                FileAction::Download => Err("Use the community download worker".into()),
                FileAction::Load => {
                    let current = params.firmware.lock().unwrap().clone();
                    let data = crate::savefile::read_with_firmware(&path, Some(&current))?;
                    let firmware = data.firmware.clone();
                    let (tx, rx) = std::sync::mpsc::channel();
                    link.cmds.send(Cmd::Load { firmware: data.firmware, sectors: data.sectors, reply: tx })
                        .map_err(|_| "the machine stopped".to_owned())?;
                    rx.recv().map_err(|_| "the machine stopped while loading".to_owned())??;
                    *link.running.lock().unwrap() = Some(firmware.pkg);
                    Ok(format!("Loaded {}", firmware.name))
                }
                FileAction::Export => {
                    let firmware = params.firmware.lock().unwrap().clone();
                    if firmware.pkg.is_empty() { return Err("load a firmware first".into()); }
                    let (tx, rx) = std::sync::mpsc::channel();
                    link.cmds.send(Cmd::Snapshot { pkg: firmware.pkg.clone(), reply: tx })
                        .map_err(|_| "the machine stopped".to_owned())?;
                    let sectors = rx.recv().map_err(|_| "the machine stopped while exporting".to_owned())??;
                    crate::savefile::write(&path, &crate::savefile::SaveData { firmware, sectors })?;
                    Ok(format!("Exported {}", path.file_name().unwrap_or_default().to_string_lossy()))
                }
            }
        };
        *result.lock().unwrap() = Some(run());
    });
    job
}

/// fm1-web's `panel_hit` codes: -1 nothing, 0..6 an encoder (FM-1; SMK-37 0..7 K1..K8), 7 the
/// FM-1's pot, 100 + id a button or key, SMK-37: 200 + p a pad, 300 + f a fader, 400 / 401 the
/// pitch / mod wheel.
fn hit(smk: bool, x: f32, y: f32) -> i32 {
    if smk {
        return match panel_smk::hit(x, y) {
            Some(panel_smk::Ctl::Enc(i)) => i as i32,
            Some(panel_smk::Ctl::Btn(id)) | Some(panel_smk::Ctl::Key(id, _)) => 100 + id as i32,
            Some(panel_smk::Ctl::Pad(p)) => 200 + p as i32,
            Some(panel_smk::Ctl::Fader(f)) => 300 + f as i32,
            Some(panel_smk::Ctl::Pitch) => 400,
            Some(panel_smk::Ctl::Mod) => 401,
            None => -1,
        };
    }
    match panel::hit(x, y) {
        Some(panel::Ctl::Enc(i)) => i as i32,
        Some(panel::Ctl::Pot) => 7,
        Some(panel::Ctl::Btn(id)) | Some(panel::Ctl::Key(id)) => 100 + id as i32,
        None => -1,
    }
}

/// How far down a key the SMK-37's panel was hit (0 top .. 1 bottom).
fn panel_smk_hit_y(x: f32, y: f32) -> f32 {
    match panel_smk::hit(x, y) {
        Some(panel_smk::Ctl::Key(_, f)) => f,
        _ => 0.5,
    }
}

fn key(ui: &mut Ui, link: &Link, id: i32, down: bool) {
    if !(0..64).contains(&id) || ui.down[id as usize] == down {
        return;
    }
    ui.down[id as usize] = down;
    let _ = link.cmds.send(Cmd::Key { id: id as u8, down });
}

fn turn(ui: &mut Ui, link: &Link, smk: bool, i: i32, clicks: i32) {
    ui.enc_clicks[i as usize] = ui.enc_clicks[i as usize].wrapping_add(clicks);
    if !smk { ui.encoder_display.turn(i as usize, clicks); }
    ui.turning[i as usize] = 10;
    let enc = if smk { i as u8 } else { panel::ENCS[i as usize].1 };
    let _ = link.cmds.send(Cmd::Enc { enc, clicks });
}

fn analog(ui: &mut Ui, link: &Link, i: usize, v: u16) {
    if ui.analog[i] != v {
        ui.analog[i] = v;
        let _ = link.cmds.send(Cmd::Analog { idx: i as u8, value: v });
    }
}

/// SMK-37: a pad pressed, a fader or a wheel dragged.
fn slide(ui: &mut Ui, link: &Link, ctl: i32, y: f32) {
    if ctl < 300 {
        analog(ui, link, (ctl - 200) as usize, 700);
    } else if ctl < 400 {
        let f = (ctl - 300) as usize;
        analog(ui, link, 16 + f, panel_smk::fader_value(f, y));
    } else {
        let pitch = ctl == 400;
        analog(ui, link, if pitch { 20 } else { 21 }, panel_smk::wheel_value(pitch, y));
    }
}

/// SMK-37: the next keys' velocity, from how far down the key was hit (fm1-web's curve).
fn velocity(link: &Link, f: f32) {
    let us = (80000f32.ln() + (2000f32.ln() - 80000f32.ln()) * f.clamp(0.0, 1.0)).exp();
    let _ = link.cmds.send(Cmd::Vel(us as u32));
}
