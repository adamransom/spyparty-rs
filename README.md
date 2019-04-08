# spyparty-rs

[![Latest version](https://img.shields.io/crates/v/spyparty.svg)](https://crates.io/crates/spyparty)
[![Documentation](https://docs.rs/spyparty/badge.svg)](https://docs.rs/spyparty)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.34+-yellow.svg)
![License](https://img.shields.io/crates/l/spyparty.svg)

A Rust library for parsing [SpyParty][1] replays.

It parses everything that is currently documented about the header, but the rest of the replay is currently undocumented (and constantly changing) and therefore not parsed.

It currently supports versions 2 to 6 (the most current) of the replay.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spyparty = "0.1"
```

and then try:

```rust
use std::fs::File;
use spyparty::{Replay, Map};

let mut file = File::open("tests/basicv6.replay").unwrap();
let replay = Replay::from_reader(&mut file).unwrap();

assert_eq!(replay.header.replay_version, 6);
assert_eq!(replay.header.result_data.map, Map::Teien);
```

# Notes

A lot of the work reverse engineering the replay was done by LtHummus with his Python project, [SpyPartyParse][2]. This was also helped by checker (SpyParty's almighty creator), who has now [documented
the replay header][3] for us over on the SpyParty beta forums (you need to own the game to see these).

[1]: http://www.spyparty.com
[2]: https://github.com/LtHummus/SpyPartyParse
[3]: https://secure.spyparty.com/beta/forums/viewtopic.php?f=8&t=2309
