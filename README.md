# human-names

A small Rust crate for generating human first and last names.

Example:

```rs
use human_names::{Generator, Origin};

let mut rng = rand::rng();
let name = Generator::new()
    .by_origin(Origin::English)
    .only_first_names()
    .only_masculine()
    .finish(&mut rng)
    .unwrap();

println!("{}", name.text); // Will print an English Male first name e.g. Xander
```

Keep in mind this library can be hefty, as it includes an entire 470kb database of 37,039 unique names.

If you want to use the dataset separately, you can download it [here](https://github.com/giraffekey/human-names/blob/main/data.csv).
