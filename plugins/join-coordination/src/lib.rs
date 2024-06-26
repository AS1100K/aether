#![feature(duration_constructors)]

mod common;

use crate::common::{handle_bot_disconnect, handle_bots, login_first_account};
use azalea::app::{App, Plugin, Startup, Update};
use azalea::ecs::prelude::*;
use azalea::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

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

        for (account, default_state) in self.0.accounts {
            let username = account.username.to_owned();
            accounts.insert(username, (account, default_state));
        }

        let join_coordinate_res = JoinCoordinateRes {
            accounts,
            max_login_time: self.0.max_login_time,
            total_queue: 0,
            accounts_in_queue: Default::default(),
            accounts_logged_in: Default::default(),
        };

        app.insert_resource(join_coordinate_res)
            .add_systems(Startup, login_first_account)
            .add_systems(GameTick, handle_bots.after(login_first_account))
            .add_systems(Update, handle_bot_disconnect.before(handle_bots));
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
    pub(crate) accounts_in_queue: HashMap<String, AccountQueueInformation>,
    pub(crate) accounts_logged_in: HashMap<String, AccountLoggedInInformation>,
}

#[derive(Clone, Copy)]
pub(crate) struct AccountQueueInformation {
    pub(crate) position_in_queue: u32,
    pub(crate) last_position_in_queue: u32,
    pub(crate) last_position_time: Instant,
    pub(crate) login_rate: u32,
}

#[derive(Clone, Copy)]
pub(crate) struct AccountLoggedInInformation {
    pub(crate) login_time: Instant,
    pub(crate) is_disconnected: bool,
}
