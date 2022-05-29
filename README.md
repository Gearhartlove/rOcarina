# rOcarina
Goal: downloadedable plugin from crates.io for use with bevy: bevy_ocarina
# Scope 
A bevy plugin which enables users to play an ocarina in their games. This is done by 
* Costimizable keybinds
* Different types of playing
  * Brief 
  * Hold
  * Composed (stetch goal)

### Enable plugin
```rust
fn main() {
    let mut app: App = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_system(ocarina_plugin::ocarina)
        .run();
}
```
### Customize Keybinds
```rust
fn main() {
    let mut app: App = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_resource(ocarina_plugin::CustomControls {
            a_note: KeyCode::R,
            b_note: KeyCode::U,
            d_note: KeyCode::S,
            d_high_note: KeyCode::T,
            f_note: KeyCode::Y,
        }) 
        .add_system(ocarina_plugin::ocarina)
        .run();
}
```
### Playing Varients
