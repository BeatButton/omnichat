use crate::id::*;
use crate::timestamp::Timestamp;

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    ChannelMarked {
        channel: ChannelId,
        ts: Timestamp,
    },
    GroupMarked {
        channel: GroupId,
        ts: Timestamp,
    },
    ImMarked {
        channel: DmId,
        ts: Timestamp,
    },
    Message {
        channel: ConversationId,
        text: Option<String>,
        user: Option<UserId>,
        username: Option<String>,
        ts: Timestamp,
        bot_id: Option<BotId>,
        #[serde(default)]
        attachments: Vec<Attachment>,
        #[serde(default)]
        files: Vec<File>,
    },
    ReactionAdded {
        item: Reactable,
        reaction: String,
    },
    ReactionRemoved {
        item: Reactable,
        reaction: String,
    },
    Hello {},
    PrefChange {},
    UserTyping {},
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Reactable {
    Message {
        channel: ConversationId,
        ts: Timestamp,
    },
}

#[derive(Deserialize)]
pub struct Reaction {
    pub name: String,
    pub count: u32,
}

#[derive(Deserialize)]
pub struct File {
    pub url_private: String,
}

#[derive(Deserialize)]
pub struct Attachment {
    pub pretext: Option<String>,
    pub text: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub files: Vec<File>,
}
