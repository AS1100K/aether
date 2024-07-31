use crate::auto_eat::{StartAutoEat, StopAutoEat};
use azalea::Client;

pub trait UtilityExt {
    fn set_auto_eat(&self, config: Option<StartAutoEat>);
}

impl UtilityExt for Client {
    /// # Example
    /// ```rust,no_run
    /// # use azalea::prelude::*;
    /// # use azalea_utility::UtilityPlugin;
    /// use azalea_utility::client::UtilityExt;
    /// use azalea_utility::auto_eat::StartAutoEat;
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// #     let account = Account::offline("_aether");
    /// #     ClientBuilder::new()
    /// #         .add_plugins(UtilityPlugin)
    /// #         .set_handler(handle)
    /// #         .start(account, "localhost")
    /// #         .await
    /// #         .unwrap();
    /// # }
    /// #
    /// # #[derive(Component, Default, Clone)]
    /// # struct State;
    ///
    /// async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    ///     match event {
    ///         Event::Login => {
    ///             bot.set_auto_eat(Some(StartAutoEat::default()));
    ///         }
    ///         _ => {}
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    fn set_auto_eat(&self, config: Option<StartAutoEat>) {
        let mut ecs = self.ecs.lock();

        if let Some(start_auto_eat) = config {
            ecs.send_event(start_auto_eat);
        } else {
            ecs.send_event(StopAutoEat);
        }
    }
}
