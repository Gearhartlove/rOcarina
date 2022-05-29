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

// todo: hold user input
// fn user_input(keys: Res<Input<KeyCode>>, audio: Res<Audio>, asset_server: Res<AssetServer>) {
//     if keys.pressed(KeyCode::A) {
//         let note: Handle<AudioSource> = asset_server.load("ocarina_plugin/assets/ocarina_d_note.ogg");
//         audio.play(note);
//     }
// }