use crate::config::Config;
use notify_rust::Notification;
use serenity_self::{Client, all::*, async_trait};
use std::sync::Arc;

/// **Windows-variant**: Discord Notify structure.
pub struct DNotify {
    /// Host User ID.
    host_user_id: u64,
}

impl DNotify {
    /// Tries to start the client.
    pub async fn start(config: Arc<Config>) {
        let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(config.get_token(), intents)
            .event_handler(Self {
                host_user_id: config.get_user_id(),
            })
            .await
            .expect("[ERROR] Failed creating client!");
        client
            .start()
            .await
            .expect("[ERROR] Failed starting client!");
    }

    /// Displays a notification for `msg`.
    async fn display_notification(&self, msg: Message) {
        if let Err(error) = Notification::new()
            .summary(msg.author.display_name())
            .body(&msg.content)
            .appname("Routecord")
            .show()
        {
            eprintln!("[ERROR] Failed showing notification, error: {error}");
        }
    }
}

#[async_trait]
impl EventHandler for DNotify {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.id == self.host_user_id {
            return;
        }

        self.display_notification(msg).await;
    }
}
