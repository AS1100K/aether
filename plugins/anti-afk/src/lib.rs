use azalea::{
    app::{App, Plugin, Update},
    ecs::prelude::*,
    entity::{clamp_look_direction, LookDirection, Position},
    interact::{handle_block_interact_event, handle_swing_arm_event, update_hit_result_component, BlockInteractEvent, SwingArmEvent},
    prelude::*,
    InstanceHolder, LookAtEvent
};
use log::trace;
use rand::{random, Rng};
use std::time::{Duration, Instant};

pub struct AntiAFKPlugin;

impl Plugin for AntiAFKPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RandomHeadRotationEvent>()
            .add_event::<FlipNearestLever>()
            .add_systems(GameTick, anti_afk)
            .add_systems(
                Update,
                (
                    random_head_rotation_listener.before(flip_nearest_lever_listener),
                    flip_nearest_lever_listener,
                )
                    .after(clamp_look_direction)
                    .after(handle_block_interact_event)
                    .after(update_hit_result_component)
                    .after(anti_afk),
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
            if !entity_mut.get::<AntiAFK>() {
                entity_mut.insert(AntiAFK::default());
            }
        } else {
            entity_mut.remove::<AntiAFK>();
        }
    }
}

#[derive(Component, Clone)]
pub struct AntiAFK {
    last_afk_tick: Instant,
}

impl Default for AntiAFK {
    fn default() -> Self {
        Self {
            last_afk_tick: Instant::now()
        }
    }
}

fn anti_afk(
    mut query: Query<(&mut AntiAFK, Entity), With<AntiAFK>>,
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
                random_head_rotation_event_writer.send(RandomHeadRotationEvent);
            } else if chances < 0.75 {
                swing_arm_event_writer.send(SwingArmEvent { entity });
            } else {
                flip_nearest_lever_event_writer.send(FlipNearestLever { entity });
            }

            anti_afk.last_afk_tick = Instant::now();
        }
    }
}

#[derive(Event)]
pub struct RandomHeadRotationEvent;

fn random_head_rotation_listener(
    mut events: EventReader<RandomHeadRotationEvent>,
    mut query: Query<&mut LookDirection, With<AntiAFK>>,
) {
    for _ in events.read() {
        for mut look_direction in &mut query.iter_mut() {
            let yaw = rand::thread_rng().gen_range(-179..180) as f32;
            trace!("AntiAFKPlugin: Setting yaw to {}", yaw);

            look_direction.y_rot = yaw
        }
    }
}

#[derive(Event)]
pub struct FlipNearestLever {
    entity: Entity,
}

fn flip_nearest_lever_listener(
    mut events: EventReader<FlipNearestLever>,
    mut query: Query<(&InstanceHolder, &Position), With<AntiAFK>>,
    mut look_at_event_writer: EventWriter<LookAtEvent>,
    mut block_interact_event_writer: EventWriter<BlockInteractEvent>,
) {
    for event in events.read() {
        for (instance_holder, position) in &mut query.iter_mut() {
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
}
