# dhcraftrs

# License: CC BY-SA
 * https://creativecommons.org/licenses/by-sa/4.0/
 * Note that there bevy example files for testing.

# Created By: Lightnet

# Status:
 * Unstable

# Code Language:
 * Rust 1.76.0 ( https://www.rust-lang.org/ )

# Engine:
 * Bevy Engine 0.13.0  ( https://bevyengine.org/ )

# Information: 
  Note this is prototype and work in progress testing. Files are subject to changes.

  To build crafting and sandbox game base on minecraft. To play test how logic and module components work in Bevy Engine in Rust Langauge.

  Required a lot knowlege to handle mesh render, loading, syncs and phyiscis. As well other things.

# Notes:
 * Work in progress build and subject to change.
 * simple menu set up for next menu and load simple scene
 * modules, components, functions query is work in progress

# Crates:
 * bevy
 * bevy_asset_loader (assets loading files)
 * bevy_rapier3d ( physics 3D )
 * bevy_mod_picking (ray casting)
 * ...
 * clap
 * bevy_console ( console gui )
 * bevy-inspector-egui ( gui )
 * bevy_egui ( gui )
 * bevy_flycam (camera control)
 * bevy_renet (network)
 * ...
 * moonshine-save 
 * bevy_svg 
 * bevy_pkv  (config save and load)
 * bevy_transform_gizmo ( editor transform handler location, rotation and scale )

# Run Tests:
```
cargo run (testing)
cargo run --bin game ( testing )
cargo run --bin launcher ( testing , N/A )
cargo run --bin testapp ( prototyping , N/A)
cargo run --bin physics3d ( prototyping , N/A)
cargo run --bin webgui ( prototyping http rest api , N/A)
```

# Controls:
 * A,D = rotate camera
 * W,S = direction movement
 * mouse, click = interaction

## Network Test ( N/A ):

```
cargo run --bin network -- server
cargo run --bin network -- client
```
https://github.com/lucaspoffo/renet/tree/master/bevy_renet

# Cargo watch:
```
cargo install cargo-watch // install cargo watch for debug detect change for rerun application.

cargo watch -x run
```

# Credits:
 * https://www.youtube.com/watch?v=iW19V3a96tY

# References:
 * https://bevy-cheatbook.github.io/setup/bevy-tools.html
 * https://bevyengine.org/
 * https://www.youtube.com/watch?v=iW19V3a96tY
 * https://github.com/frederickjjoubert/bevy-ball-game/tree/Episode-9-Fixes
 * https://www.youtube.com/watch?v=SmqQ_Is9QX8
 * https://www.youtube.com/watch?v=y50VKG1YvPA Splash screens with bevy asset loader
 * https://github.com/NiklasEi/bevy_common_assets
 * https://github.com/NiklasEi/bevy_asset_loader
 * 