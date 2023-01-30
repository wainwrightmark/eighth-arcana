use std::rc::Rc;
use yewdux::store::Reducer;
use yewdux::store::Store;

use super::logging::EventLog;
use super::logging::Loggable;
use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserState {
    pub user_id: Option<uuid::Uuid>,
}

impl Reducer<UserState> for CreateUserIfNewMessage {
    fn apply(self, state: Rc<UserState>) -> Rc<UserState> {
        if state.user_id.is_some() {
            state
        } else {
            let user_id = uuid::Uuid::new_v4();

            let message = EventLog::new_user(user_id, self.referrer);
            message.send_log();

            Rc::new(UserState {
                user_id: Some(user_id),
            })
        }
    }
}
