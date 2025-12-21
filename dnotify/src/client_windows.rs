use notify_rust::Notification;
use serenity::{Client, all::*, async_trait};

/// **Windows-variant**: Discord Notify structure.
pub struct DNotify {
    /// Host User ID.
    host_user_id: u64,
}

impl DNotify {
    /// Tries to start the client.
    pub async fn start() {
        let token = std::env::args()
            .nth(1)
            .expect("[ERROR] Missing argument for Token, usage: ./dnotify TOKEN_HERE HOST_USER_ID");
        let host_user_id: u64 = std::env::args()
            .nth(2)
            .expect(
                "[ERROR] Missing argument for host User ID, usage: ./dnotify TOKEN_HERE HOST_USER_ID",
            )
            .parse()
            .expect("[ERROR] Failed parsing User ID as u64!");
        let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(token, intents)
            .event_handler(Self { host_user_id })
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
        if msg.author.id == self.host_user_id {
            return;
        }

        self.display_notification(msg).await;
    }
}
