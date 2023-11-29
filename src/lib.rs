/*!

# HOI4 Save

HOI4 Save is a library to ergonomically work with Hearts of Iron IV saves (plaintext + binary).

```rust,ignore
use hoi4save::{Hoi4File, Encoding, EnvTokens, models::Hoi4Save};
let data = std::fs::read("assets/saves/1.10-normal-text.hoi4")?;
let file = Hoi4File::from_slice(&data)?;
let parsed_file = file.parse()?;
let save: Hoi4Save = parsed_file.deserializer().build(&EnvTokens)?;
assert_eq!(file.encoding(), Encoding::Plaintext);
assert_eq!(save.player, String::from("FRA"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

The HOI4 binary format can be converted to plaintext

```rust,ignore
use hoi4save::{Hoi4File, EnvTokens};

let data = std::fs::read("assets/saves/1.10-ironman.hoi4")?;
let file = Hoi4File::from_slice(&data)?;
let parsed_file = file.parse()?;
let binary = parsed_file.as_binary().unwrap();
let out = binary
    .melter()
    .on_failed_resolve(hoi4save::FailedResolveStrategy::Stringify)
    .melt(&EnvTokens)?;

# Ok::<(), Box<dyn std::error::Error>>(())
```

## Binary Saves

By default, binary saves will not be decoded properly.

To enable support, one must supply an environment variable
(`HOI4_IRONMAN_TOKENS`) that points to a newline delimited
text file of token descriptions. For instance:

```ignore
0xffff my_test_token
0xeeee my_test_token2
```

In order to comply with legal restrictions, I cannot share the list of
tokens. I am also restricted from divulging how the list of tokens can be derived.

*/

mod country_tag;
mod date;
mod de;
mod errors;
mod extraction;
pub mod file;
mod flavor;
mod melt;
pub mod models;
mod tokens;

pub use country_tag::*;
pub use date::*;
pub use errors::*;
pub use extraction::*;
#[doc(inline)]
pub use file::Hoi4File;
pub use jomini::binary::FailedResolveStrategy;
pub use melt::*;
pub use tokens::EnvTokens;
