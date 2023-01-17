use std::{collections::BTreeMap, str::FromStr};

use crate::data::prelude::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store)]
pub struct ImageDescriptionState {
    pub descriptions: BTreeMap<(Guide, Card), ImageDescription>,
}

impl Default for ImageDescriptionState {
    fn default() -> Self {
        let data = include_str!("../tsv/descriptions.tsv");
        let lines = data.lines();
        let descriptions: BTreeMap<_, _> = lines
            .skip(1) //skip headers
            .filter_map(|x| ImageDescription::from_str(x).ok())
            //.map(|x| ImageDescription::from_str(x).unwrap())
            .map(|x| ((x.guide, x.card), x))
            .collect();

        Self { descriptions }
    }
}

#[cfg(test)]
mod tests {
    use super::ImageDescriptionState;
    use crate::data::prelude::*;
    use strum::EnumCount;

    #[test]
    pub fn test_descriptions() {
        let state = ImageDescriptionState::default();
        assert_eq!(Guide::COUNT * Card::COUNT, state.descriptions.len());

        for (_, desc) in state.descriptions {
            assert!(!desc.guidance.is_empty());
            assert!(!desc.representation.is_empty());
            assert!(!desc.agent_representation.is_empty());
            assert!(!desc.user_representation.is_empty());
            assert!(!desc.guide_interpretation.is_empty());
        }
    }
}
