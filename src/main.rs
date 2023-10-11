mod config;
mod discord;

use config::Config;
use discord::DiscordClient;

use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

#[tokio::main]
pub async fn main() {
    let config = Config::load();
    let token = config.get_discord_token().unwrap();
    let stream_chat_id = config.get_stream_chat_id().unwrap();

    let discord_client = DiscordClient::build(token, stream_chat_id).await;

    let twitch_client_config = ClientConfig::default();
    let (incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(twitch_client_config);

    let join_handle = tokio::spawn(message_listener(incoming_messages, discord_client));

    client.join(config.get_twitch_channel().unwrap()).unwrap();
    println!("Listening!");

    join_handle.await.unwrap();
}

async fn message_listener(
    mut incoming_messages: UnboundedReceiver<ServerMessage>,
    discord_client: DiscordClient,
) {
    while let Some(message) = incoming_messages.recv().await {
        if let ServerMessage::Privmsg(content) = message {
            let name = content.sender.name;
            let text = content.message_text;
            let message_content = format!("**[{}]**: {}", name, text);
            discord_client.send_message(message_content).await;
        }
    }
}
