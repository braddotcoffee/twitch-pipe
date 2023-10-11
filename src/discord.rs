use serenity::http::Http;
use serenity::model::prelude::ChannelId;

pub struct DiscordClient {
    pub client: Http,
    pub stream_chat_id: u64,
}

impl DiscordClient {
    pub async fn build(token: String, stream_chat_id: u64) -> DiscordClient {
        let client = Http::new(&token);
        DiscordClient {
            client,
            stream_chat_id,
        }
    }

    pub async fn send_message(&self, content: String) {
        let _ = ChannelId::from(self.stream_chat_id)
            .say(&self.client, content)
            .await;
    }
}
