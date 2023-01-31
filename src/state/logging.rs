use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yewdux::prelude::Dispatch;

use crate::{
    data::{prelude::*, spread_id::SpreadId},
    state::prelude::*,
};

use super::data_state::DataState;

/// This token can only be used to ingest data into our bucket
const API_TOKEN: &str = "xaat-ba30896b-604b-4837-8924-ec8097e55eee";

#[derive(Debug, Clone, Serialize)]
pub struct EventLog {
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "is_false")]
    pub resent: bool,
    pub event: LoggableEvent,
}

fn is_false(b: &bool) -> bool {
    !b
}

impl EventLog {
    pub fn new(user_id: Uuid, event: LoggableEvent) -> Self {
        Self {
            user_id,
            resent: false,
            event,
        }
    }
    pub fn new_resent(user_id: Uuid, event: LoggableEvent) -> Self {
        Self {
            user_id,
            resent: true,
            event,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoggableEvent {
    NewUser {
        referrer: Option<String>,
    },
    NewSpread {
        question_data: QuestionData,
        spread_id: String
    },
    Share {
        src_data: SrcData2,
    },
    Achievement {
        achievement: Achievement,
    },
    ReceivedShare{
        referrer: Option<String>,
        spread_id: Option<String>,
        img_id: Option<String>
    }
}

impl LoggableEvent {
    pub fn new_spread(data: &DataState) -> Self {
        let question_data = data.question_data;
        let spread_id = SpreadId::new(&question_data, &data.cards_permutation).encode();

        Self::NewSpread {
            question_data,
            spread_id,
        }
    }

    pub fn new_share(referrer: Option<String>,
        spread_id: Option<String>,
        img_id: Option<String>)-> Self{
            Self::ReceivedShare { referrer, spread_id, img_id }
        }
}

impl From<Achievement> for LoggableEvent {
    fn from(achievement: Achievement) -> Self {
        Self::Achievement { achievement }
    }
}

impl From<SrcData> for LoggableEvent {
    fn from(src_data: SrcData) -> Self {
        Self::Share {
            src_data: src_data.into(),
        }
    }
}

impl EventLog {
    pub fn send_log(self) {
        wasm_bindgen_futures::spawn_local(Self::log(self));
    }

    async fn try_log<T: Serialize>(data: &T) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.axiom.co/v1/datasets/intarot_usage/ingest")
            // .header("Authorization", format!("Bearer {API_TOKEN}"))
            .bearer_auth(API_TOKEN)
            .header("Content-Type", "application/json")
            .json(&[data])
            .send()
            .await?;

        res.error_for_status().map(|_| ())
    }

    async fn log(data: Self) {
        let r = Self::try_log(&data).await;
        if let Err(err) = r {
            log::error!("Logging Error {}", err);
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(data.event));
        } else {
            log::debug!("Log sent successfully");
        }
    }
}

// // impl Loggable for EventLog {}

// pub trait Loggable: Sized + Serialize + 'static {

// }
