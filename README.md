# terminyl


<div align = "center">
  A CLI Pomodoro Timer in Rust <sub>but because I don't really know Rust, it's really bad</sub>

 <img src = "https://github.com/user-attachments/assets/276958d9-a471-4430-9ee1-f5bbc9b3b4f2">
 </div>

<br>

## Usage
### Build from source
To use, clone the repository and `cd terminyl/terminyl`. 

Then run the following in a terminal window, where `<study time>` and `<rest_time>` are integer lengths in minutes, and `<session_count>` is an integer representing the amount of Pomodoro sessions. If you want to use the release version, use `cargo build --release` instead of `cargo run`.

```
cargo run <study_time> <rest_time> <session_count>
```

### Pre-existing release
Alternatively, download the latest release from the [releases page](https://github.com/phthallo/terminyl/releases) and `cd` into the download location.

Then run the following in a terminal window, where `<study time>` and `<rest_time>` are integer lengths in minutes, and `<session_count>` is an integer representing the amount of Pomodoro sessions.

```
terminyl <study_time> <rest_time> <session_count>
```
