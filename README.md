# dhcraftrs

# License: CC BY-SA
 * https://creativecommons.org/licenses/by-sa/4.0/
 * Note that there bevy example files for testing.

# Created By: Lightnet

# Code Language:
 * Rust 1.70.0 ( https://www.rust-lang.org/ )

# Engine:
 * Bevy Engine 0.10.1  ( https://bevyengine.org/ )

# Information: 
  Note this is prototype and work in progress testing. Files are subject to changes.

  To build crafting and sandbox game base on minecraft. To play test how logic and module components work in Bevy Engine in Rust Langauge.

# Notes:
 * Work in progress build and subject to change.
 * simple menu set up for next menu and load simple scene
 * modules, components, functions query is work in progress

# Crates:
 * bevy 0.10.1
 * bevy_asset_loader v0.16 (assets loading files)
 * bevy_rapier3d vbevy_rapier3d ( physics 3D )
 * bevy_mod_picking v0.13.0 (ray casting)
 * ...
 * clap v4.1.10
 * bevy_console v0.7.0 ( console gui )
 * bevy-inspector-egui v0.18.3 ( gui )
 * bevy_egui v0.20 ( gui )
 * bevy_flycam v0.10 (camera control)
 * bevy_renet v0.0.12 (network)
 * ...
 * moonshine-save v0.2.3 ()
 * bevy_svg v0.10.1
 * bevy_pkv v0.7.0 (config save and load)
 * bevy_transform_gizmo v0.6 ( editor transform handler location, rotation and scale )

# Run Tests:
```
cargo run (testing)
cargo run --bin game ( testing )
cargo run --bin launcher ( testing )
cargo run --bin testapp ( prototyping )
cargo run --bin physics3d ( prototyping )
cargo run --bin webgui ( prototyping http rest api )
```

# Controls:
 * A,D = rotate camera
 * W,S = direction movement
 * mouse, click = interaction

## Network test:

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