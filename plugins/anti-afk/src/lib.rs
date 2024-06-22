#![doc = include_str!("../README.md")]

pub mod config;

use std::cmp::PartialEq;
use azalea::{app::{App, Plugin, Update}, ecs::prelude::*, entity::{clamp_look_direction, LookDirection, Position}, interact::{handle_block_interact_event, update_hit_result_component, BlockInteractEvent, SwingArmEvent}, prelude::*, InstanceHolder, LookAtEvent, JumpEvent};
use log::trace;
use rand::{random, Rng, thread_rng};
use std::time::{Duration, Instant};
use azalea::entity::LocalEntity;
use azalea::entity::metadata::{Player, ShiftKeyDown};
use azalea::packet_handling::game::SendPacketEvent;
use azalea::protocol::packets::game::serverbound_player_command_packet::{Action, ServerboundPlayerCommandPacket};
use azalea::world::MinecraftEntityId;
use crate::config::{AntiAFKConfig, Walk};

pub struct AntiAFKPlugin;

impl Plugin for AntiAFKPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomHeadRotationEvent>()
            .add_event::<FlipNearestLever>()
            .add_systems(GameTick, anti_afk)
            .add_systems(
                Update,
                (
                    random_head_rotation_listener,
                    flip_nearest_lever_listener,
                )
                    .chain()
                    .after(clamp_look_direction)
                    .after(handle_block_interact_event)
                    .after(update_hit_result_component)
                    .after(anti_afk)
            );
    }
}

pub trait AntiAFKClientExt {
    fn set_anti_afk(&self, enabled: bool, config: Option<AntiAFKConfig>);
}

impl AntiAFKClientExt for Client {
    fn set_anti_afk(&self, enabled: bool, config: Option<AntiAFKConfig>) {
        let mut ecs = self.ecs.lock();
        let mut entity_mut = ecs.entity_mut(self.entity);

        if enabled {
            if entity_mut.get::<AntiAFK>().is_none() {
                let config = config.expect("AntiAFK config wasn't passed");

                entity_mut.insert(AntiAFK {
                    last_afk_tick: Instant::now(),
                    config
                });
            }
        } else {
            entity_mut.remove::<AntiAFK>();
        }
    }
}

#[derive(Component, Clone)]
pub struct AntiAFK {
    last_afk_tick: Instant,
    config: AntiAFKConfig
}

fn anti_afk(
    mut query: Query<(&mut AntiAFK, Entity), (With<AntiAFK>, With<Player>, With<LocalEntity>)>,
    mut random_head_rotation_event_writer: EventWriter<RandomHeadRotationEvent>,
    mut swing_arm_event_writer: EventWriter<SwingArmEvent>,
    mut flip_nearest_lever_event_writer: EventWriter<FlipNearestLever>,
) {
    for (mut anti_afk, entity) in &mut query.iter_mut() {
        let now = Instant::now();
        let last_tick = anti_afk.last_afk_tick;

        if now.duration_since(last_tick) >= Duration::from_secs(3) {
            let chances: f64 = random();

            if chances < 0.5 {
                random_head_rotation_event_writer.send(RandomHeadRotationEvent {
                    entity,
                    jump: anti_afk.config.jump,
                    sneak: anti_afk.config.sneak
                });
            } else if chances < 0.75 {
                let new_chances: f64 = random();
                if anti_afk.config.walk != Walk::None && new_chances < 0.5 {
                    // Execute Random Walking
                    todo!()
                } else {
                    swing_arm_event_writer.send(SwingArmEvent { entity });
                }
            } else {
                flip_nearest_lever_event_writer.send(FlipNearestLever { entity });
            }

            anti_afk.last_afk_tick = Instant::now();
        }
    }
}

#[derive(Event)]pub struct RandomHeadRotationEvent {
    pub entity: Entity,
    pub jump: bool,
    pub sneak: bool
}

fn random_head_rotation_listener(
    mut events: EventReader<RandomHeadRotationEvent>,
    mut query: Query<(&mut LookDirection, &MinecraftEntityId, &ShiftKeyDown), With<AntiAFK>>,
    mut jump_event: EventWriter<JumpEvent>,
    mut send_packet_event: EventWriter<SendPacketEvent>
) {
    for event in events.read() {
        let (mut look_direction, entity_id, shift_key_down) = query.get_mut(event.entity).unwrap();
        let yaw = thread_rng().gen_range(-179..180) as f32;
        trace!("AntiAFKPlugin: Setting yaw to {}", yaw);

        look_direction.y_rot = yaw;

        let new_chances: f64 = random();

        if new_chances < 0.25 {
            // Sneak
            if event.sneak {
                std::thread::sleep(Duration::from_secs(1));
                let action_packet = if shift_key_down.0 {
                    Action::ReleaseShiftKey
                } else {
                    Action::PressShiftKey
                };

                send_packet_event.send(SendPacketEvent {
                    entity: event.entity,
                    packet: ServerboundPlayerCommandPacket {
                        id: **entity_id,
                        action: action_packet,
                        data: 0,
                    }.get()
                });
            }

        } else if new_chances < 0.5 {
            // Jump
            if event.jump {
                std::thread::sleep(Duration::from_secs(1));
                jump_event.send(JumpEvent {
                    entity: event.entity
                });
            }
        }
    }
}

#[derive(Event)]
pub struct FlipNearestLever {
    entity: Entity,
}

fn flip_nearest_lever_listener(
    mut events: EventReader<FlipNearestLever>,
    query: Query<(&InstanceHolder, &Position), With<AntiAFK>>,
    mut look_at_event_writer: EventWriter<LookAtEvent>,
    mut block_interact_event_writer: EventWriter<BlockInteractEvent>,
) {
    for event in events.read() {
        let (instance_holder, position) = query.get(event.entity).unwrap();

        let entity = event.entity;
        let instance = instance_holder.instance.clone();

        let nearest_lever = instance
            .read()
            .find_block(position, &azalea::registry::Block::Lever.into());

        if let Some(lever) = nearest_lever {
            if lever.to_vec3_floored().distance_to(position) > 4.0 {
                trace!("azalea-anti-afk: Lever is out of reach");
                return;
            }
            look_at_event_writer.send(LookAtEvent {
                entity,
                position: lever.to_vec3_floored(),
            });
            std::thread::sleep(Duration::from_secs(1));
            block_interact_event_writer.send(BlockInteractEvent {
                entity,
                position: lever,
            });
        }
    }
}
