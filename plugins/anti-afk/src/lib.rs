#![doc = include_str!("../README.md")]

pub mod config;

use crate::config::AntiAFKConfig;
use azalea::entity::metadata::{Player, ShiftKeyDown};
use azalea::entity::LocalEntity;
use azalea::packet_handling::game::SendPacketEvent;
use azalea::protocol::packets::game::serverbound_player_command_packet::{
    Action, ServerboundPlayerCommandPacket,
};
use azalea::world::MinecraftEntityId;
use azalea::{
    app::{App, Plugin, Update},
    ecs::prelude::*,
    entity::{clamp_look_direction, LookDirection, Position},
    interact::{
        handle_block_interact_event, update_hit_result_component, BlockInteractEvent, SwingArmEvent,
    },
    prelude::*,
    InstanceHolder, JumpEvent, LookAtEvent, StartWalkEvent, WalkDirection,
};
use log::{info, trace};
use rand::{random, thread_rng, Rng};
use std::time::{Duration, Instant};

pub struct AntiAFKPlugin;

impl Plugin for AntiAFKPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomHeadRotationEvent>()
            .add_event::<FlipNearestLever>()
            .add_event::<RandomWalkEvent>()
            .add_systems(GameTick, (anti_afk, handle_stop_walk_after_certain_time).chain())
            .add_systems(
                Update,
                (
                    random_head_rotation_listener,
                    flip_nearest_lever_listener,
                    handle_random_walk_event,
                )
                    .chain()
                    .after(clamp_look_direction)
                    .after(handle_block_interact_event)
                    .after(update_hit_result_component)
                    .after(anti_afk),
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
                    config,
                    has_moved: None,
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
    config: AntiAFKConfig,
    has_moved: Option<f32>
}

fn anti_afk(
    mut query: Query<(&mut AntiAFK, Entity), (With<AntiAFK>, With<Player>, With<LocalEntity>)>,
    mut random_head_rotation_event_writer: EventWriter<RandomHeadRotationEvent>,
    mut swing_arm_event_writer: EventWriter<SwingArmEvent>,
    mut flip_nearest_lever_event_writer: EventWriter<FlipNearestLever>,
    mut random_walk_event: EventWriter<RandomWalkEvent>,
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
                    sneak: anti_afk.config.sneak,
                });
            } else if chances < 0.75 {
                let new_chances: f64 = random();
                if anti_afk.config.walk && new_chances < 0.5 {
                    random_walk_event.send(RandomWalkEvent { entity });
                } else {
                    swing_arm_event_writer.send(SwingArmEvent { entity });
                }
            } else {
                if anti_afk.config.flip_lever {
                    flip_nearest_lever_event_writer.send(FlipNearestLever { entity });
                } else {
                    swing_arm_event_writer.send(SwingArmEvent { entity });
                }
            }

            anti_afk.last_afk_tick = Instant::now();
        }
    }
}

#[derive(Event)]
pub struct RandomHeadRotationEvent {
    pub entity: Entity,
    pub jump: bool,
    pub sneak: bool,
}

fn random_head_rotation_listener(
    mut events: EventReader<RandomHeadRotationEvent>,
    mut query: Query<(&mut LookDirection, &MinecraftEntityId, &ShiftKeyDown), With<AntiAFK>>,
    mut jump_event: EventWriter<JumpEvent>,
    mut send_packet_event: EventWriter<SendPacketEvent>,
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
                    }
                    .get(),
                });
            }
        } else if new_chances < 0.5 {
            // Jump
            if event.jump {
                std::thread::sleep(Duration::from_secs(1));
                jump_event.send(JumpEvent {
                    entity: event.entity,
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

#[derive(Event)]
pub struct RandomWalkEvent {
    entity: Entity,
}

fn handle_random_walk_event(
    mut commands: Commands,
    mut events: EventReader<RandomWalkEvent>,
    mut query: Query<(&mut AntiAFK, &mut LookDirection), With<AntiAFK>>,
    mut start_walk_event: EventWriter<StartWalkEvent>,
    mut random_head_rotation_event: EventWriter<RandomHeadRotationEvent>,
) {
    for event in events.read() {
        info!("Executing Random Movement");
        let (mut anti_afk, mut look_direction) = query.get_mut(event.entity).unwrap();

        let timer = Instant::now();

        if let Some(yaw) = anti_afk.has_moved {
            look_direction.y_rot = yaw;

            std::thread::sleep(Duration::from_millis(100));
            start_walk_event.send(StartWalkEvent {
                entity: event.entity,
                direction: WalkDirection::Backward,
            });

            anti_afk.has_moved = None;
        } else {
            random_head_rotation_event.send(RandomHeadRotationEvent {
                entity: event.entity,
                jump: false,
                sneak: false,
            });

            std::thread::sleep(Duration::from_millis(100));
            start_walk_event.send(StartWalkEvent {
                entity: event.entity,
                direction: WalkDirection::Forward,
            });

            anti_afk.has_moved = Some(look_direction.y_rot);
        }

        commands.entity(event.entity).insert(StopWalkAfterCertainTime {
            start_time: timer,
            time: Duration::from_millis(250)
        });
    }
}

#[derive(Component)]
pub struct StopWalkAfterCertainTime {
    start_time: Instant,
    time: Duration
}

fn handle_stop_walk_after_certain_time(
    mut commands: Commands,
    query: Query<(&StopWalkAfterCertainTime, Entity), (With<AntiAFK>, With<StopWalkAfterCertainTime>)>,
    mut start_walk_event: EventWriter<StartWalkEvent>
) {
    for (stop_walk_after_certain_time, entity) in query.iter() {
        if stop_walk_after_certain_time.start_time.elapsed() >= stop_walk_after_certain_time.time {
            start_walk_event.send(StartWalkEvent {
                entity,
                direction: WalkDirection::None
            });

            commands.entity(entity).remove::<StopWalkAfterCertainTime>();
        }
    }
}