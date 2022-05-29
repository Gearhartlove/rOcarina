use std::ops::Deref;
use bevy::prelude::*;

// q: what happens when there is no controls?
// -> None Enum varient?
// sounds from https://noproblo.dayjo.org/ZeldaSounds/OOT/index.html

const DEFAULT_CONTROLS: CustomControls = default_controls();

pub fn ocarina(keys: Res<Input<KeyCode>>, mut custom_controls: Option<ResMut<CustomControls>>, audio: Res<Audio>,
               asset_server: Res<AssetServer>) {
    // check if custom controls are defined or not
    let controls: CustomControls = setup_controls(custom_controls);

    // todo: handle holding user inputs and cutting of sound when not hold
    // If I pressed the A key, then play the A note sound

    if keys.just_pressed(controls.a_note) {
        play_note(Note::A, &asset_server, &audio);
    }
    if keys.just_pressed(controls.b_note) {
        play_note(Note::B, &asset_server, &audio);
    }
    if keys.just_pressed(controls.d_note) {
        play_note(Note::D, &asset_server, &audio);
    }
    if keys.just_pressed(controls.d_high_note) {
        play_note(Note::DHigh, &asset_server, &audio);
    }
    if keys.just_pressed(controls.f_note) {
        play_note(Note::F, &asset_server, &audio);
    }
}

fn setup_controls(custom_controls: Option<ResMut<CustomControls>>) -> CustomControls {
    if let None = custom_controls {
        return DEFAULT_CONTROLS;
    }
    return *custom_controls.unwrap().deref();
}
fn play_note(note: Note, asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    match note {
        Note::A => {
            let note: Handle<AudioSource> =
                asset_server.load("ocarina_plugin_assets/ocarina_a_note_med.ogg");
            audio.play(note);
        }
        Note::B => {
            let note: Handle<AudioSource> =
                asset_server.load("ocarina_plugin_assets/ocarina_b_note_med.ogg");
            audio.play(note);
        }
        Note::D => {
            let note: Handle<AudioSource> =
                asset_server.load("ocarina_plugin_assets/ocarina_d_note_med.ogg");
            audio.play(note);
        }
        Note::DHigh => {
            let note: Handle<AudioSource> =
                asset_server.load("ocarina_plugin_assets/ocarina_d_high_note_med.ogg");
            audio.play(note);
        }
        Note::F => {
            let note: Handle<AudioSource> =
                asset_server.load("ocarina_plugin_assets/ocarina_f_note.ogg");
            audio.play(note);
        }
    }
}

#[derive(Clone, Copy)]
pub struct CustomControls {
    a_note: KeyCode,
    b_note: KeyCode,
    d_note: KeyCode,
    d_high_note: KeyCode,
    f_note: KeyCode,
}

const fn default_controls() -> CustomControls {
    CustomControls {
        a_note: KeyCode::A,
        b_note: KeyCode::B,
        d_note: KeyCode::D,
        d_high_note: KeyCode::H,
        f_note: KeyCode::F,
    }
}

enum Note {
    A,
    B,
    D,
    DHigh,
    F,
}