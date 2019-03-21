# spyparty-rs

This Rust crate allows simple parsing of [SpyParty][1] replays.

It parses everything that is currently documented about the header, but the rest of the replay is currently undocumented (and constantly changing) and therefore not parsed.

It currently supports versions 3 to 6 (the most current) of the replay.

# Examples

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
[3]: https://secure.spyparty.com/beta/forums/viewtopic.php?f=8&t=2309&p=50270
