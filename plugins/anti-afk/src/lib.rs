use std::time::{Duration, Instant};
use azalea::app::{App, Plugin, Update};
use azalea::ecs::prelude::{Entity, Event, EventReader, EventWriter, IntoSystemConfigs};
use azalea::ecs::query::With;
use azalea::ecs::system::Query;
use azalea::entity::{clamp_look_direction, LookDirection};
use azalea::interact::{SwingArmEvent, update_hit_result_component};
use azalea::movement::MoveEventsSet;
use azalea::physics::PhysicsSet;
use azalea::prelude::*;
use log::{info, trace};
use rand::{random, Rng};

pub struct AntiAFKPlugin;

impl Plugin for AntiAFKPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RandomHeadRotationEvent>()
            .add_systems(
                GameTick,
                anti_afk
            )
            .add_systems(
                Update,
                random_head_rotation_listener
                    .before(clamp_look_direction)
                    .before(update_hit_result_component)
            );
    }
}

pub trait AntiAFKClientExt {
    fn set_anti_afk(&self, enabled: bool);
}

impl AntiAFKClientExt for Client {
    fn set_anti_afk(&self, enabled: bool) {
        let mut ecs = self.ecs.lock();
        let mut entity_mut = ecs.entity_mut(self.entity);

        if enabled {
            entity_mut.insert(AntiAFK {
                last_afk_tick: Instant::now()
            });
        } else {
            entity_mut.remove::<AntiAFK>();
        }

    }
}

#[derive(Component, Clone)]
pub struct AntiAFK {
    last_afk_tick: Instant
}

fn anti_afk(
    mut query: Query<(&mut AntiAFK, Entity), With<AntiAFK>>,
    mut random_head_rotation_event_writer: EventWriter<RandomHeadRotationEvent>,
    mut swing_arm_event_writer: EventWriter<SwingArmEvent>
) {
    for (mut anti_afk, entity) in &mut query.iter_mut() {
        let now = Instant::now();
        let last_tick = anti_afk.last_afk_tick;

        if now.duration_since(last_tick) >= Duration::from_secs(3) {
            let chances: f64 = random();

            if chances < 0.5 {
                random_head_rotation_event_writer.send(RandomHeadRotationEvent);
            } else if chances < 0.75 {
                swing_arm_event_writer.send(SwingArmEvent {
                    entity,
                });
            }

            anti_afk.last_afk_tick = Instant::now();
        }
    }
}

#[derive(Event)]
pub struct RandomHeadRotationEvent;

fn random_head_rotation_listener(
    mut events: EventReader<RandomHeadRotationEvent>,
    mut query: Query<&mut LookDirection, With<AntiAFK>>
) {
    for _ in events.read() {
        for mut look_direction in &mut query.iter_mut() {
            let yaw = rand::thread_rng().gen_range(-179..180) as f32;
            trace!("AntiAFKPlugin: Setting yaw to {}", yaw);

            look_direction.y_rot = yaw
        }
    }
}