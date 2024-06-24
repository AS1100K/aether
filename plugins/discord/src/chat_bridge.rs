use crate::SendDiscordMessage;
use azalea::app::{App, Plugin, Update};
use azalea::chat::ChatReceivedEvent;
use azalea::ecs::prelude::*;
use azalea::entity::metadata::Player;
use azalea::entity::LocalEntity;
use azalea::prelude::*;
use azalea::TabList;
use uuid::Uuid;

/// This plugin will send all the chat messages on the server to discord via webhook.
///
/// # Examples
/// ```rust,no_run
/// # use azalea::prelude::*;
/// use azalea_discord::chat_bridge::DiscordChatBridgePlugin;
/// use azalea_discord::chat_bridge::DiscordChatBridgeExt;
///
/// #[tokio::main]
/// async fn main() {
///     # let account = Account::offline("_aether");
///     ClientBuilder::new()
///         .set_handler(handle)
///         .add_plugins(DiscordChatBridgePlugin)
///         .start(account, "localhost")
///         .await
///         .unwrap();
/// }
/// #
/// # #[derive(Default, Component, Copy, Clone)]
/// # struct State;
///
/// async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
///     match event {
///         Event::Login => {
///             // Start bridging mc chat on discord
///             bot.set_discord_chat_bridge(true, "Aether Bot", Some("https://url-of-discord-webhook.com".to_string()));
///         }
///         _ => {}
///     }
///
///     Ok(())
/// }
/// ```
pub struct DiscordChatBridgePlugin;

impl Plugin for DiscordChatBridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_chat_event);
    }
}

#[derive(Component, Clone)]
pub struct DiscordChatBridge {
    webhook: String,
    default_username: &'static str,
}

#[allow(clippy::complexity)]
fn handle_chat_event(
    mut events: EventReader<ChatReceivedEvent>,
    query: Query<
        (&TabList, &DiscordChatBridge),
        (With<DiscordChatBridge>, With<Player>, With<LocalEntity>),
    >,
    mut send_discord_message: EventWriter<SendDiscordMessage>,
) {
    for event in events.read() {
        if let Ok((tab_list, discord_chat_bridge)) = query.get(event.entity) {
            let (username, content) = event.packet.split_sender_and_content();

            let new_username = if let Some(uname) = &username {
                uname.to_owned()
            } else {
                discord_chat_bridge.default_username.to_string()
            };

            let mut avatar = "https://avatars.akamai.steamstatic.com/8d9a6a75e45129943fadcc869bfae2ee3bb2a535_full.jpg".to_string();
            if let Some(uname) = username {
                let uuid = extract_uuid_from_tab_list(tab_list, uname);
                if let Some(x) = uuid {
                    avatar = format!("https://minotar.net/avatar/{}", x);
                }
            }

            let send_discord_message_content = SendDiscordMessage {
                webhook: discord_chat_bridge.webhook.to_owned(),
                contents: content.to_owned(),
                username: Some(new_username),
                avatar_url: Some(avatar),
            };

            send_discord_message.send(send_discord_message_content);
        }
    }
}

fn extract_uuid_from_tab_list(tab_list: &TabList, username: String) -> Option<Uuid> {
    for (uuid, player_info) in tab_list.iter() {
        if player_info.profile.name == username {
            return Some(*uuid);
        }
    }

    None
}

pub trait DiscordChatBridgeExt {
    fn set_discord_chat_bridge(
        &self,
        enabled: bool,
        default_username: &'static str,
        webhook: Option<String>,
    );
}

impl DiscordChatBridgeExt for Client {
    fn set_discord_chat_bridge(
        &self,
        enabled: bool,
        default_username: &'static str,
        webhook: Option<String>,
    ) {
        let mut ecs = self.ecs.lock();
        let mut world = ecs.entity_mut(self.entity);

        if enabled {
            world.insert(DiscordChatBridge {
                webhook: webhook.expect(
                    "If you want to enable discord chat bridge, you need to provide webhook",
                ),
                default_username,
            });
        } else {
            world.remove::<DiscordChatBridge>();
        }
    }
}
