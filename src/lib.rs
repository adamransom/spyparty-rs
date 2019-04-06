/*!
  A Rust library for parsing [SpyParty][1] replays.

  It parses everything that is currently documented about the header, but the rest of the replay is currently undocumented (and constantly changing) and therefore not parsed.

  It currently supports versions 2 to 6 (the most current) of the replay.

  # Examples

  ```
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
*/
#[macro_use]
mod error;

pub mod replay;

mod game_mode;
mod game_result;
mod map;
mod mission;
mod utils;

pub use error::{Error, Result};
pub use game_mode::GameMode;
pub use game_result::GameResult;
pub use map::Map;
pub use mission::Mission;
pub use replay::Replay;
