use notify_rust::Notification;
use serenity::{Client, all::*, async_trait};

/// **Windows-variant**: Discord Notify structure.
pub struct DNotify;

impl DNotify {
    /// Tries to start the client.
    pub async fn start() {
        let token = std::env::args()
            .nth(1)
            .expect("[ERROR] Missing argument for token, usage: ./dnotify TOKEN_HERE");
        let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(token, intents)
            .event_handler(Self)
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
            .appname("DNotify")
            .show()
        {
            eprintln!("[ERROR] Failed showing notification, error: {error}");
        }
    }
}

#[async_trait]
impl EventHandler for DNotify {
    async fn message(&self, _ctx: Context, msg: Message) {
        self.display_notification(msg).await;
    }
}
