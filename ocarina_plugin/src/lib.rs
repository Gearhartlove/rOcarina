use std::ops::Deref;
use bevy::prelude::*;

// q: what happens when there is no controls?
// -> None Enum varient?

const DEFAULT_CONTROLS: CustomControls = default_controls();

pub fn ocarina(keys: Res<Input<KeyCode>>, mut custom_controls: Option<ResMut<CustomControls>>, audio: Res<Audio>,
               asset_server: Res<AssetServer>) {
    // check if custom controls are defined or not
    let controls: CustomControls = setup_controls(custom_controls);

    // handle user inputs
    if keys.pressed(controls.d_note) {
        let note: Handle<AudioSource> =
            asset_server.load("ocarina_plugin_assets/ocarina_d_note.ogg");
        audio.play(note);
    }
}

fn setup_controls(custom_controls: Option<ResMut<CustomControls>>) -> CustomControls {
    if let None = custom_controls {
        return DEFAULT_CONTROLS;
    }
    return *custom_controls.unwrap().deref();
}

#[derive(Clone, Copy)]
pub struct CustomControls {
    d_note: KeyCode,
    a_sharp_note: KeyCode,
}

const fn default_controls() -> CustomControls {
    CustomControls {
        d_note: KeyCode::D,
        a_sharp_note: KeyCode::A,
    }
}