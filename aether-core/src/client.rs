use crate::config::Role;
use crate::State;
use azalea::protocol::packets::game::clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket;
use azalea::protocol::packets::game::serverbound_client_command_packet::Action::PerformRespawn;
use azalea::protocol::packets::game::serverbound_client_command_packet::ServerboundClientCommandPacket;
use azalea::{Client, ClientInformation};
use log::info;
use std::sync::Arc;

pub async fn handle_init(client: Client, state: State) -> anyhow::Result<()> {
    info!("Initialized bot");
    if state.config.role == Role::Pearl {
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
    _state: State,
    _death: Option<Arc<ClientboundPlayerCombatKillPacket>>,
) -> anyhow::Result<()> {
    info!("The bot has died, respawning.");
    let respawn_command_packet = ServerboundClientCommandPacket {
        action: PerformRespawn,
    };

    client.write_packet(respawn_command_packet.get())?;

    Ok(())
}
