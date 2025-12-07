use bincode::{Decode, Encode};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};
use std::sync::LazyLock;

mod origin;

pub use origin::Origin;

const DATA: &[u8; 476107] = include_bytes!("../data.bin");

static NAMES: LazyLock<Vec<Name>> = LazyLock::new(|| {
    let (names, _): (Vec<Name>, _) =
        bincode::decode_from_slice(DATA, bincode::config::standard()).unwrap();
    names
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Decode, Encode)]
pub enum Gender {
    Unisex,
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Decode, Encode)]
pub enum NameKind {
    First,
    Last,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Decode, Encode)]
pub struct Name {
    pub text: String,
    pub origins: Vec<Origin>,
    pub gender: Gender,
    pub kind: NameKind,
}

#[derive(Debug, Clone, Default)]
pub struct Generator {
    first_letters: Option<SmallVec<[char; 1]>>,
    origins: Option<SmallVec<[Origin; 1]>>,
    kind: Option<NameKind>,
    gender: Option<Gender>,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn by_first_letter(&mut self, letter: char) -> &mut Self {
        if let Some(first_letters) = &mut self.first_letters {
            first_letters.push(letter);
        } else {
            self.first_letters = Some(smallvec![letter]);
        }
        self
    }

    pub fn by_first_letters(&mut self, letters: &[char]) -> &mut Self {
        if let Some(first_letters) = &mut self.first_letters {
            first_letters.extend(letters.iter().copied());
        } else {
            self.first_letters = Some(letters.into());
        }
        self
    }

    pub fn by_origin(&mut self, origin: Origin) -> &mut Self {
        if let Some(origins) = &mut self.origins {
            origins.push(origin);
        } else {
            self.origins = Some(smallvec![origin]);
        }
        self
    }

    pub fn by_origins(&mut self, os: &[Origin]) -> &mut Self {
        if let Some(origins) = &mut self.origins {
            origins.extend(os.iter().copied());
        } else {
            self.origins = Some(os.into());
        }
        self
    }

    pub fn only_first_names(&mut self) -> &mut Self {
        self.kind = Some(NameKind::First);
        self
    }

    pub fn only_last_names(&mut self) -> &mut Self {
        self.kind = Some(NameKind::Last);
        self
    }

    pub fn only_masculine(&mut self) -> &mut Self {
        self.gender = Some(Gender::Male);
        self
    }

    pub fn only_feminine(&mut self) -> &mut Self {
        self.gender = Some(Gender::Female);
        self
    }

    pub fn only_unisex(&mut self) -> &mut Self {
        self.gender = Some(Gender::Unisex);
        self
    }

    pub fn finish<R: Rng>(&self, rng: &mut R) -> Option<Name> {
        NAMES
            .iter()
            .filter(|name: &&Name| {
                if let Some(first_letters) = &self.first_letters {
                    let found = first_letters.iter().any(|c| name.text.starts_with(*c));
                    if !found {
                        return false;
                    }
                }

                if let Some(origins) = &self.origins {
                    let found = origins
                        .iter()
                        .any(|o| name.origins.iter().any(|o2| o == o2));
                    if !found {
                        return false;
                    }
                }

                if let Some(kind) = &self.kind {
                    if name.kind != *kind {
                        return false;
                    }
                }

                if let Some(gender) = &self.gender {
                    if name.kind == NameKind::First && name.gender != *gender {
                        return false;
                    }
                }

                true
            })
            .choose(rng)
            .cloned()
    }

    pub fn full_name<R: Rng>(&self, rng: &mut R) -> Option<(Name, Name)> {
        let first = self.clone().only_first_names().finish(rng);
        let last = self.clone().only_last_names().finish(rng);
        match (first, last) {
            (Some(first), Some(last)) => Some((first, last)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator() {
        let mut rng = rand::rng();
        assert!(Generator::new().finish(&mut rng).is_some());
        assert!(
            Generator::new()
                .only_masculine()
                .only_last_names()
                .finish(&mut rng)
                .is_some(),
        );
        assert!(
            Generator::new()
                .by_origin(Origin::Biblical)
                .only_last_names()
                .finish(&mut rng)
                .is_none(),
        );
    }
}
