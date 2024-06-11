use std::time::Duration;
use azalea::{BotClientExt, Client};
use azalea::container::ContainerClientExt;
use azalea::protocol::packets::game::serverbound_interact_packet::InteractionHand;
use log::{debug, trace};
use rand::Rng;

pub async fn random_head_rotations(client: &mut Client) -> anyhow::Result<()> {
    let y_rot = rand::thread_rng().gen_range(-180..180);
    trace!("1 second has passed, setting `y_rot` to {}", y_rot);

    client.set_direction(y_rot as f32, client.direction().1);

    Ok(())
}

pub async fn open_nearest_chest(client: &mut Client) -> anyhow::Result<()> {
    let nearest_chest = client.world().read().find_block(
        client.position(),
        &azalea::registry::Block::Chest.into()
    );

    if let Some(pos) = nearest_chest {
        client.look_at(pos.to_vec3_floored());
        tokio::time::sleep(Duration::from_millis(500)).await;
        let chest_handle = client.open_container_at(pos).await;
        tokio::time::sleep(Duration::from_secs(2)).await;
        if let Some(handle) = chest_handle {
            drop(handle);
        }
    } else {
        debug!("Unable to find nearest chest, send swing packet instead.");
        send_swing_packet(client).await?
    }

    Ok(())
}

pub async fn send_swing_packet(client: &mut Client) -> anyhow::Result<()> {
    let swing_packet = azalea::protocol::packets::game::serverbound_swing_packet::ServerboundSwingPacket {
        hand: InteractionHand::MainHand,
    };

    client.write_packet(swing_packet.get()).expect("Unable to send Swing Packet.");

    Ok(())
}