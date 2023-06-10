# dhcraftrs

# License: CC BY-SA

# Created By: Lightnet
https://creativecommons.org/licenses/by-sa/4.0/

# Code Language:
 * Rust 1.70.0 ( https://www.rust-lang.org/ )

# Engine:
 * Bevy Engine 0.10.0  ( https://bevyengine.org/ )

# Information: (work in progress...)
  Still need to set up stand alone. To build sandbox world. As well some menu set up for test build them.
  
## Network:
  To develop server and client. For player to enjoy the sandbox world. One reason is that to develop permission system. One reason is handle user permission when entering the game to handle hacking and spam. For editing and playing at the same time. It would required some default settings.

  Simalar to chat group system.

## Design:
  To develop module component is not easy as every part is broke up to handle render and logics.

# Run Tests:
```
cargo run --bin game
cargo run --bin editor
cargo run --bin launcher


```
## network test:

```
cargo run --bin network -- server
cargo run --bin network -- client
```
https://github.com/lucaspoffo/renet/tree/master/bevy_renet


# Cargo params:
```
cargo run --package testlib //param
```

# Credits:
 * https://www.youtube.com/watch?v=iW19V3a96tY

# References:
 * https://bevy-cheatbook.github.io/setup/bevy-tools.html
 * https://bevyengine.org/
 * https://www.youtube.com/watch?v=iW19V3a96tY
 * https://github.com/frederickjjoubert/bevy-ball-game/tree/Episode-9-Fixes
 * 
 * 