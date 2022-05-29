use bevy::prelude::*;

// source : http://soundfxcenter.com/download-sound/ocarina-of-time-notes-flute-d-long-sound-effect/

fn main() {
    let mut app: App = App::new();
    app
        .add_plugins(DefaultPlugins)
        // .add_resource(ocarina_plugin::CustomControls {}) // optional
        .add_system(ocarina_plugin::ocarina)
        // .add_plugin(Ocarina)
        .run();
}