//! Headless tests of resetting a slider (issue #1436): double-clicking its name or its thumb puts it
//! back to its default in one undo step, as in Lightroom, and resting the pointer on either says so.

use std::time::Duration;

use serde_json::json;

use crate::headless::Headless;
use crate::{LightcraftApp, Services};

const T: Duration = Duration::from_secs(20);
const SETTLE: Duration = Duration::from_secs(120);
/// The sliders the issue names, each set away from its default.
const EDITED: [(&str, f64); 2] = [("light.exposure", 1.25), ("light.contrast", 40.0)];

/// The demo library in Detail with the Edit panel's Light section open.
fn edit_panel() -> Headless {
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), Services { png: None, ..Default::default() });
    let mut h = Headless::new(app, [1400.0, 900.0], 1.0);
    let id = h.app.session.visible_cloned()[0].0;
    assert_eq!(h.request("engine.execute", json!({"command": "library.select", "params": {"ids": [id]}}), T)["ok"], true);
    assert_eq!(h.request("ui.set", json!({"view": "detail", "right": "edit"}), T)["ok"], true);
    h.app.ui.open_sections = vec!["light".to_string()];
    h.settle(SETTLE);
    h
}

fn widget(h: &Headless, id: &str) -> egui::Rect {
    h.app.widgets.iter().find(|(w, _)| w == id).map(|(_, r)| *r).unwrap_or_else(|| panic!("no widget {id}"))
}

fn value(h: &Headless, control: &str) -> f64 {
    let id = h.app.session.selection.active.unwrap();
    lightcraft_develop::controls::get(&h.app.session.develop_of(id).unwrap_or_default(), control).unwrap()
}

fn default(control: &str) -> f64 {
    lightcraft_develop::controls::find(control).unwrap().default
}

fn ask(h: &mut Headless, method: &str, params: serde_json::Value) {
    let r = h.request(method, params.clone(), T);
    assert_eq!(r["ok"], true, "{method} {params}: {r}");
    h.settle(SETTLE);
}

fn set(h: &mut Headless, control: &str, value: f64) {
    ask(h, "engine.execute", json!({"command": "develop.set", "params": {"control": control, "value": value}}));
}

/// The thumb's place along the track (0..1), plus `dx` points: a hand lands near its centre, not on it.
fn thumb_fx(h: &Headless, control: &str, dx: f32) -> f32 {
    let spec = lightcraft_develop::controls::find(control).unwrap();
    let track = widget(h, &format!("slider:{control}"));
    spec.position(value(h, control)) as f32 + dx / track.width()
}

/// A double-click on a widget: the control channel plays each press and release in a frame of its
/// own, so the slider sees the first click before the second, as with a hand on a mouse.
fn double_click(h: &mut Headless, id: &str, fx: f32) {
    ask(h, "ui.clickWidget", json!({"id": id, "count": 2, "fx": fx}));
}

#[test]
fn double_clicking_a_sliders_name_resets_it_in_one_step() {
    let mut h = edit_panel();
    for (control, edited) in EDITED {
        set(&mut h, control, edited);
        let undo = h.app.session.undo.len();
        double_click(&mut h, &format!("sliderLabel:{control}"), 0.1);
        assert_eq!(value(&h, control), default(control), "{control} is back at its default");
        assert_eq!(h.app.session.undo.len() - undo, 1, "{control}: one undo step");
        ask(&mut h, "engine.execute", json!({"command": "edit.undo"}));
        assert_eq!(value(&h, control), edited, "{control}: undo brings the edit back");
    }
}

/// The first click of the double-click lands a few points off the thumb's centre: it leaves the value
/// alone (it used to move it there, a history step of its own before the reset).
#[test]
fn double_clicking_a_sliders_thumb_resets_it_in_one_step() {
    let mut h = edit_panel();
    for (control, edited) in EDITED {
        set(&mut h, control, edited);
        let undo = h.app.session.undo.len();
        let fx = thumb_fx(&h, control, 4.0);
        double_click(&mut h, &format!("slider:{control}"), fx);
        assert_eq!(value(&h, control), default(control), "{control} is back at its default");
        assert_eq!(h.app.session.undo.len() - undo, 1, "{control}: one undo step");
        ask(&mut h, "engine.execute", json!({"command": "edit.undo"}));
        assert_eq!(value(&h, control), edited, "{control}: undo brings the edit back");
    }
}

/// A click on the thumb leaves the value alone; a click elsewhere on the track still moves it there.
#[test]
fn a_click_on_the_thumb_keeps_the_value() {
    let mut h = edit_panel();
    set(&mut h, "light.exposure", 1.25);
    let fx = thumb_fx(&h, "light.exposure", -4.0);
    ask(&mut h, "ui.clickWidget", json!({"id": "slider:light.exposure", "fx": fx}));
    assert_eq!(value(&h, "light.exposure"), 1.25);
    ask(&mut h, "ui.clickWidget", json!({"id": "slider:light.exposure", "fx": 0.25}));
    assert!((value(&h, "light.exposure") - -2.5).abs() < 0.02, "{}", value(&h, "light.exposure"));
}

#[test]
fn a_sliders_name_and_thumb_say_a_double_click_resets_it() {
    let mut h = edit_panel();
    set(&mut h, "light.exposure", 1.25);
    let hint = "Exposure — double-click to reset it";
    let shown = |h: &Headless| h.painted_text().iter().any(|t| t == hint);
    let thumb = thumb_fx(&h, "light.exposure", 0.0);
    for (what, id, fx) in [("name", "sliderLabel:light.exposure", 0.1), ("thumb", "slider:light.exposure", thumb)] {
        // somewhere else first, so the hint of the step before goes
        ask(&mut h, "ui.hoverWidget", json!({"id": "sliderLabel:light.contrast", "fx": 0.9}));
        assert!(!shown(&h), "{what}: no hint left over");
        ask(&mut h, "ui.hoverWidget", json!({"id": id, "fx": fx}));
        // tooltips wait for the pointer to rest
        assert!(
            h.step_until(Duration::from_secs(5), shown),
            "{what}: {:?}",
            h.painted_text().iter().filter(|t| t.contains("reset")).collect::<Vec<_>>()
        );
    }
    // the rest of the track is for dragging and clicking: no hint there
    ask(&mut h, "ui.hoverWidget", json!({"id": "slider:light.exposure", "fx": 0.1}));
    for _ in 0..90 {
        h.step();
    }
    assert!(!shown(&h), "no hint away from the thumb");
    // the value keeps its own hint
    ask(&mut h, "ui.hoverWidget", json!({"id": "sliderValue:light.exposure"}));
    assert!(h.step_until(Duration::from_secs(5), |h| h.painted_text().iter().any(|t| t == "Click to type a value, drag to change it")));
}

/// Typing a value (KEY-SLIDER-TYPE, #1362) still works next to the reset.
#[test]
fn a_typed_value_still_applies_after_a_reset() {
    let mut h = edit_panel();
    set(&mut h, "light.exposure", 1.25);
    double_click(&mut h, "sliderLabel:light.exposure", 0.1);
    assert_eq!(value(&h, "light.exposure"), 0.0);
    let undo = h.app.session.undo.len();
    ask(&mut h, "ui.clickWidget", json!({"id": "sliderValue:light.exposure"}));
    for (method, params) in [("ui.text", json!({"text": "0.5"})), ("ui.key", json!({"key": "Enter"}))] {
        ask(&mut h, method, params);
    }
    assert_eq!(value(&h, "light.exposure"), 0.5);
    assert_eq!(h.app.session.undo.len() - undo, 1, "one undo step");
}

/// AC-PILOT-3492: the header menu must dispatch the section reset, not a whole-photo reset.
#[test]
fn section_reset_from_calibration_header_preserves_other_edits_and_is_undoable() {
    let mut h = edit_panel();
    h.size.y = 1400.0;
    h.app.ui.open_sections = vec!["calibration".into()];
    h.step();
    ask(&mut h, "engine.execute", json!({"command": "develop.profile", "params": {"id": "lc.neutral", "amount": 70.0}}));
    set(&mut h, "light.exposure", 1.25);
    for spec in lightcraft_develop::controls::in_section(lightcraft_develop::Section::Calibration) {
        set(&mut h, spec.id, 25.0);
    }
    let id = h.app.session.selection.active.unwrap();
    let before = h.app.session.develop_of(id).unwrap();
    let undo = h.app.session.undo.len();
    ask(&mut h, "ui.clickWidget", json!({"id": "section:calibration", "button": "right"}));
    assert!(h.painted_text().iter().any(|text| text == "Reset Section"));
    assert_eq!(h.app.ui.open_sections, ["calibration"]);
    if let Some(dir) = std::env::var_os("LIGHTCRAFT_TEST_SHOTS") {
        h.app.services.png = Some(Box::new(|img| {
            lightcraft_codecs::encode_png(&lightcraft_codecs::EncodeImage::rgba8(img), &lightcraft_codecs::EncodeMeta::default()).unwrap()
        }));
        h.app.services.write = Some(Box::new(|path, bytes| std::fs::write(path, bytes).map_err(|e| e.to_string())));
        h.app.services.write_new = Some(Box::new(lightcraft_engine::export::write_file_new));
        ask(&mut h, "ui.screenshot", json!({"path": std::path::Path::new(&dir).join("calibration-reset-menu.png"), "headless": true}));
        let inspect = h.request("ui.inspect", json!({}), T);
        assert_eq!(inspect["ok"], true, "{inspect}");
        std::fs::write(std::path::Path::new(&dir).join("calibration-reset-menu.inspect.json"), serde_json::to_vec_pretty(&inspect).unwrap()).unwrap();
    }
    ask(&mut h, "ui.clickWidget", json!({"id": "sectionMenu:reset"}));
    for spec in lightcraft_develop::controls::in_section(lightcraft_develop::Section::Calibration) {
        assert_eq!(value(&h, spec.id), spec.default, "{}", spec.id);
    }
    assert_eq!(value(&h, "light.exposure"), 1.25);
    assert_eq!(h.app.session.develop_of(id).unwrap().profile, before.profile);
    let mut unrelated = (*h.app.session.develop_of(id).unwrap()).clone();
    unrelated.calibration = before.calibration.clone();
    assert_eq!(&unrelated, before.as_ref(), "every setting outside Calibration is preserved");
    assert_eq!(h.app.session.undo.len() - undo, 1);
    assert_eq!(h.app.ui.open_sections, ["calibration"]);
    assert!(!h.app.ui.single_panel);
    ask(&mut h, "engine.execute", json!({"command": "edit.undo"}));
    assert_eq!(h.app.session.develop_of(id).unwrap(), before);
}

/// The pilot must not expose resets for differently grouped panels or Defringe.
#[test]
fn section_reset_is_not_offered_on_other_headers_and_their_menus_preserve_edits() {
    let mut h = edit_panel();
    h.size.y = 1400.0;
    h.app.ui.open_sections.clear();
    h.step();
    for (control, edited) in [
        ("light.exposure", 1.25),
        ("color.vibrance", 25.0),
        ("effects.clarity", 25.0),
        ("vignette.amount", -25.0),
        ("grain.amount", 25.0),
        ("detail.sharpenAmount", 25.0),
        ("optics.defringePurple", 25.0),
    ] {
        set(&mut h, control, edited);
    }
    let id = h.app.session.selection.active.unwrap();
    let before = h.app.session.develop_of(id).unwrap();
    let undo = h.app.session.undo.len();
    for section in ["light", "color", "effects", "detail", "optics"] {
        ask(&mut h, "ui.clickWidget", json!({"id": format!("section:{section}"), "button": "right"}));
        assert!(!h.app.widgets.iter().any(|(id, _)| id == "sectionMenu:reset"), "{section}");
        assert!(!h.painted_text().iter().any(|text| text == "Reset Section"), "{section}");
        assert!(h.app.widgets.iter().any(|(id, _)| id == "sectionMenu:solo"), "{section}: original menu retained");
        ask(&mut h, "ui.key", json!({"key": "Escape"}));
        assert_eq!(h.app.session.develop_of(id).unwrap(), before, "{section}: no settings changed");
        assert_eq!(h.app.session.undo.len(), undo, "{section}: no history change");
        assert_eq!(h.app.session.selection.active, Some(id));
        assert!(h.app.ui.open_sections.is_empty());
        assert!(!h.app.ui.single_panel);
    }
}

#[test]
fn section_reset_keeps_header_disclosure_eye_and_solo_mode_working() {
    let mut h = edit_panel();
    h.size.y = 1400.0;
    h.app.ui.open_sections.clear();
    h.step();
    ask(&mut h, "ui.clickWidget", json!({"id": "section:calibration"}));
    assert_eq!(h.app.ui.open_sections, ["calibration"]);
    ask(&mut h, "ui.hoverWidget", json!({"id": "section:calibration"}));
    ask(&mut h, "ui.clickWidget", json!({"id": "sectionEye:calibration"}));
    let id = h.app.session.selection.active.unwrap();
    assert!(!h.app.session.develop_of(id).unwrap().section_enabled("calibration"));
    set(&mut h, "calibration.redHue", 25.0);
    ask(&mut h, "ui.clickWidget", json!({"id": "section:calibration", "button": "right"}));
    ask(&mut h, "ui.clickWidget", json!({"id": "sectionMenu:reset"}));
    assert_eq!(value(&h, "calibration.redHue"), 0.0);
    assert!(!h.app.session.develop_of(id).unwrap().section_enabled("calibration"));
    assert_eq!(h.app.ui.open_sections, ["calibration"]);
    ask(&mut h, "ui.clickWidget", json!({"id": "sectionEye:calibration"}));
    assert!(h.app.session.develop_of(id).unwrap().section_enabled("calibration"));
    ask(&mut h, "ui.clickWidget", json!({"id": "section:calibration"}));
    assert!(h.app.ui.open_sections.is_empty());
    ask(&mut h, "ui.clickWidget", json!({"id": "section:calibration", "button": "right"}));
    ask(&mut h, "ui.clickWidget", json!({"id": "sectionMenu:solo"}));
    assert!(h.app.ui.single_panel);
    ask(&mut h, "ui.clickWidget", json!({"id": "section:light"}));
    ask(&mut h, "ui.clickWidget", json!({"id": "section:calibration"}));
    assert_eq!(h.app.ui.open_sections, ["calibration"]);
}
