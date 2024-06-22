use crate::config::Bot;
use azalea::protocol::packets::game::clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket;
use azalea::protocol::packets::game::serverbound_client_command_packet::Action::PerformRespawn;
use azalea::protocol::packets::game::serverbound_client_command_packet::ServerboundClientCommandPacket;
use azalea::{Client, ClientInformation};
use log::info;
use std::sync::Arc;
use azalea_anti_afk::AntiAFKClientExt;
use azalea_anti_afk::config::AntiAFKConfig;

pub async fn handle_init(client: Client, state: Bot) -> anyhow::Result<()> {
    info!("Initialized bot, {}", state.username);
    if state.render_distance.is_some_and(|rd| rd <= 32) {
        client
            .set_client_information(ClientInformation {
                view_distance: state.render_distance.unwrap(),
                ..Default::default()
            })
            .await?;
    } else {
        client
            .set_client_information(ClientInformation {
                view_distance: 5,
                ..Default::default()
            })
            .await?;
    }

    Ok(())
}

pub async fn handle_death(
    client: Client,
    state: Bot,
    _death: Option<Arc<ClientboundPlayerCombatKillPacket>>,
) -> anyhow::Result<()> {
    info!("{} has died, respawning.", state.username);
    let respawn_command_packet = ServerboundClientCommandPacket {
        action: PerformRespawn,
    };

    let central_afk_location = if let Some(afk_location) = state.afk_location {
        Some(afk_location.to_vec3_floored())
    } else {
        None
    };

    let anti_afk_config = AntiAFKConfig {
        jump: true,
        sneak: true,
        walk: true,
        flip_lever: true,
        central_afk_location
    };

    client.write_packet(respawn_command_packet.get())?;
    client.set_anti_afk(true, Some(anti_afk_config));

    Ok(())
}
