
# Information:
  Looking for crate packages to run well to build sandbox world.

# Packages refs:
 * https://www.youtube.com/watch?v=QTUEyAZmdv4 Pathfinding and Async Tasks in Bevy
 * https://github.com/mwbryant/logic_management_game
 * 
 * 
 * 
 * 
 * 

# Cargo params:
```
cargo run --package testlib //param
```


# reqwest:
 * https://github.com/seanmonstar/reqwest#blocking-client
 * https://docs.rs/reqwest/0.10.1/reqwest/blocking/index.html
 * https://docs.rs/reqwest/latest/reqwest/

```rs
//sync
let body = reqwest::blocking::get("https://www.rust-lang.org").await?
    .text()
    .await?;

//sync
let body = reqwest::blocking::get("https://www.rust-lang.org")?
    .text()?;

// no sync
let body = reqwest::blocking::get("https://www.rust-lang.org").unwrap().text().unwrap();
println!("body = {:?}", body);
```
