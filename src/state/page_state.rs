use std::rc::Rc;

use yewdux::prelude::*;

use super::{card_page_state::CardPageState, messages::*};

impl Reducer<CardPageState> for DrawMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        Rc::new(state.to_owned().draw_card())
    }
}

impl Reducer<CardPageState> for ReplaceMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        Rc::new(state.to_owned().replace_card())
    }
}
impl Reducer<CardPageState> for ToggleDescriptionMessage {
    fn apply(self, state: Rc<CardPageState>) -> Rc<CardPageState> {
        Rc::new(state.to_owned().toggle_description())
    }
}
