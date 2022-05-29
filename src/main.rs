use bevy::prelude::*;

fn main() {
    let mut app: App = App::new();
    app
        .add_plugins(DefaultPlugins)
        // .add_plugin(Ocarina)
        .run();
}
