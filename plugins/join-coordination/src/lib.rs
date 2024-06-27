#![feature(duration_constructors)]

mod common;
mod queue;
mod chat;
mod utils;

use crate::common::{add_account, handle_bot_disconnect, handle_bots, login_first_account};
use azalea::app::{App, Plugin, Startup, Update};
use azalea::ecs::prelude::*;
use azalea::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use bevy_time::common_conditions::on_timer;
use regex::Regex;
use crate::chat::handle_chat;
use crate::queue::{add_login_component, add_queue_component, queue_information};

pub struct JoinCoordination<S>
where
    S: Send + Sync + Clone + Component + 'static,
{
    accounts: Vec<(Account, S)>,
    max_login_time: Duration,
}

impl<S> JoinCoordination<S>
where
    S: Send + Sync + Clone + Component + 'static,
{
    /// Creates a new `JoinCoordination` specifically for 2b2t.org. If you want to customize this
    /// use `JoinCoordination::new_opts` instead.
    pub fn new() -> Self {
        Self::new_opts(Duration::from_hours(6))
    }

    /// Creates a new `JoinCoordination` with custom values of `max_login_time` and `average_queue_size`.
    ///
    /// `max_login_time`: Maximum time a bot can stay in the server before getting kicked.
    pub fn new_opts(max_login_time: Duration) -> Self {
        Self {
            accounts: Vec::new(),
            max_login_time,
        }
    }

    /// Adds a new account which will be using join coordination plugin.
    pub fn add(&mut self, account: Account, state: S) {
        self.accounts.push((account, state));
    }
}

pub struct JoinCoordinationPlugin<S>(JoinCoordination<S>)
where
    S: Send + Sync + Clone + Component + 'static;

impl<S> Plugin for JoinCoordinationPlugin<S>
where
    S: Send + Sync + Clone + Component + 'static,
{
    fn build(&self, app: &mut App) {
        let mut accounts: HashMap<String, (Account, S)> = HashMap::new();

        for (account, default_state) in self.0.accounts.clone().into_iter() {
            let username = account.username.to_owned();
            accounts.insert(username, (account, default_state));
        }

        let join_coordinate_res = JoinCoordinateRes {
            accounts,
            max_login_time: self.0.max_login_time,
            total_queue: 0,
            login_rate: 400f32/12f32/60f32/60f32,
            eta: Duration::from_hours(12),
            accounts_in_queue: Default::default(),
            accounts_logged_in: Default::default(),
            re_position: Regex::new(r"Position in queue: (\d+)").unwrap(),
            re_total_position: Regex::new(r"normal: (\d+)").unwrap()
        };

        app
            .insert_resource(join_coordinate_res)
            .add_event::<AddAccountEvent>()
            .add_event::<AccountJoinedEvent>()
            .add_event::<AddAccount<S>>()
            .add_systems(Startup, login_first_account::<S>)
            .add_systems(GameTick, handle_bots::<S>.after(login_first_account::<S>))
            .add_systems(
                Update,
                (
                    handle_bot_disconnect::<S>.before(handle_bots::<S>),
                    handle_chat::<S>,
                    queue_information::<S>.run_if(on_timer(Duration::from_secs(5))),
                    add_login_component::<S>,
                    add_queue_component::<S>,
                    add_account::<S>
                )
                    .chain()
            );
    }
}

#[derive(Resource, Clone)]
pub(crate) struct JoinCoordinateRes<S>
where
    S: Send + Sync + Clone + Component + 'static,
{
    pub(crate) accounts: HashMap<String, (Account, S)>,
    pub(crate) max_login_time: Duration,
    pub(crate) total_queue: u32,
    pub(crate) login_rate: f32,
    pub(crate) eta: Duration,
    pub(crate) accounts_in_queue: HashMap<String, AccountQueueInformation>,
    pub(crate) accounts_logged_in: HashMap<String, AccountLoggedInInformation>,
    pub(crate) re_position: Regex,
    pub(crate) re_total_position: Regex
}

#[derive(Clone, Copy)]
pub(crate) struct AccountQueueInformation {
    pub(crate) position_in_queue: u32,
    pub(crate) last_position_in_queue: u32,
    pub(crate) last_position_time: Instant,
}

#[derive(Clone, Copy)]
pub(crate) struct AccountLoggedInInformation {
    pub(crate) login_time: Instant,
    pub(crate) is_disconnected: bool,
}

#[derive(Event)]
pub(crate) struct AddAccount<S>
where S: Send + Sync + Clone + Component + 'static
{
    account: Account,
    state: S
}

#[derive(Event)]
/// This event is called when an account is added to queue
pub struct AddAccountEvent {
    pub account: Account
}

#[derive(Event)]
/// This event is called when an account has joined the server i.e. pass the queue
pub struct AccountJoinedEvent {
    pub username: String,
    pub login_time: Instant
}

#[derive(Component)]
pub struct AccountInQueue;

#[derive(Component)]
pub struct AccountLoggedIn;