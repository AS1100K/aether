use azalea::app::{App, Plugin, Update};
use azalea::chat::ChatReceivedEvent;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::entity::LocalEntity;
use azalea::entity::metadata::Player;
use azalea::TabList;
use uuid::Uuid;
use crate::SendDiscordMessage;

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
///         .add_plugins(DiscordChatBridgePlugin {
///             webhook: "https://url-of-discord-webhook.com"
///          })
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
///             bot.set_discord_chat_bridge(true);
///         }
///         _ => {}
///     }
///
///     Ok(())
/// }
/// ```
pub struct DiscordChatBridgePlugin {
    pub webhook: &'static str
}

impl Plugin for DiscordChatBridgePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DiscordChatBridgeRes { webhook: self.webhook })
            .add_systems(Update, handle_chat_event);
    }
}

#[derive(Resource)]
struct DiscordChatBridgeRes {
    webhook: &'static str
}

#[derive(Component, Clone)]
pub struct DiscordChatBridge;

fn handle_chat_event(
    discord_chat_bridge_res: Res<DiscordChatBridgeRes>,
    mut events: EventReader<ChatReceivedEvent>,
    query: Query<&TabList, (With<DiscordChatBridge>, With<Player>, With<LocalEntity>)>,
    mut send_discord_message: EventWriter<SendDiscordMessage>
) {
    for event in events.read() {
        let tab_list = query.get(event.entity).unwrap();
        let (username, content) = event.packet.split_sender_and_content();

        let mut new_username: Option<&str> = Some("2b2t Server");
        let mut avatar: Option<&str> = Some("https://www.2b2t.org/content/images/size/w256h256/2023/08/2b2t_256x-1.png");

        if let Some(uname) = username {
            new_username = Some(&*uname);
            let uuid = extract_uuid_from_tab_list(&tab_list, uname);
            if let Some(x) = uuid {
                avatar = Some(&format!("https://crafatar.com/avatars/{}", x.to_string()));
            }
        }

        let send_discord_message_content = SendDiscordMessage {
            webhook: discord_chat_bridge_res.webhook,
            contents: &*content,
            username: new_username,
            avatar_url: avatar,
        };

        send_discord_message.send(send_discord_message_content);
    }
}

fn extract_uuid_from_tab_list(
    tab_list: &TabList,
    username: String
) -> Option<Uuid> {
    for (uuid, player_info) in tab_list.iter() {
        if player_info.profile.name == username {
            return Some(*uuid)
        }
    }

    None
}

pub trait DiscordChatBridgeExt {
    fn set_discord_chat_bridge(&self, enabled: bool);
}

impl DiscordChatBridgeExt for Client {
    fn set_discord_chat_bridge(&self, enabled: bool) {
        let mut ecs = self.ecs.lock().entity_mut(self.entity);

        if enabled {
            if !ecs.contains::<DiscordChatBridge>() {
                ecs.insert(DiscordChatBridge);
            }
        } else {
            ecs.remove::<DiscordChatBridge>();
        }
    }
}
