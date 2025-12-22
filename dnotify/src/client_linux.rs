use dashmap::DashMap;
use notify_rust::Notification;
use serenity::{Client, all::*, async_trait};
use std::sync::Arc;

/// Holds extra information about an avatar.
pub struct AvatarData {
    /// Image for the avatar.
    image: notify_rust::Image,

    /// URL to the avatar.
    url: String,
}

/// **Linux-variant**: Discord Notify structure.
pub struct DNotify {
    /// Cached avatars.
    ///
    /// - `Key`: User ID
    /// - `Value`: Avatar Data
    cached_avatars: DashMap<u64, Arc<AvatarData>>,

    /// Host User ID.
    host_user_id: u64,
}

impl DNotify {
    /// Tries to start the client.
    pub async fn start() {
        let token = std::env::args().nth(1).unwrap_or_else(|| {
            std::env::var("TOKEN").unwrap_or_else(|_| {
                std::fs::read_to_string("./token.txt").expect(
                    "[ERROR] Missing argument for Token, usage: ./dnotify TOKEN_HERE HOST_USER_ID",
                )
            })
        });
        let host_user_id: u64 = std::env::args()
            .nth(2).unwrap_or_else(|| std::env::var("HOST_USER_ID")
                .unwrap_or_else(|_|
                    std::fs::read_to_string("./host_user_id.txt")
                        .expect("[ERROR] Missing argument for Host User ID, usage: ./dnotify TOKEN_HERE HOST_USER_ID")))
            .parse()
            .expect("[ERROR] Failed parsing Host User ID as u64!");

        let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(token, intents)
            .event_handler(Self {
                cached_avatars: DashMap::new(),
                host_user_id,
            })
            .await
            .expect("[ERROR] Failed creating client!");
        client
            .start()
            .await
            .expect("[ERROR] Failed starting client!");
    }

    /// Tries to get the avatar url for `author`, capping the size
    /// at `32` pixels as we don't need higher for a basic thumbnail icon.
    fn get_avatar_url(&self, author: &User) -> Option<String> {
        Some(author.avatar_url()?.replace("size=1024", "size=32"))
    }

    /// **Expensive**: Gets the remote avatar for `author`, if any is present
    /// and assuming it could be successfully read and converted.
    async fn get_remote_avatar(&self, author: &User) -> Option<Arc<AvatarData>> {
        let avatar_url = self.get_avatar_url(author)?;
        if let Ok(image_response) = reqwest::get(&avatar_url).await
            && let Ok(image_bytes) = image_response.bytes().await
            && let Ok(image) = image::load_from_memory(&image_bytes)
        {
            let image_rgba = image.into_rgba8();
            return Some(Arc::new(AvatarData {
                image: notify_rust::Image::from_rgba(
                    image_rgba.width() as i32,
                    image_rgba.height() as i32,
                    image_rgba.into_vec(),
                )
                .expect("[ERROR] Failed converting image!"),
                url: avatar_url,
            }));
        }

        None
    }

    /// Gets the avatar for `author` if one is present.
    ///
    /// It first looks in the local cache and validates it, otherwise it tries
    /// to get the remote avatar, cache it and return.
    async fn get_avatar(&self, author: &User) -> Option<Arc<AvatarData>> {
        let avatar_url = self.get_avatar_url(author)?;

        if let Some(cached_avatar_data) = self.cached_avatars.get(&author.id.get()) {
            // If the URL doesn't match, update the entry and return.
            if cached_avatar_data.url != avatar_url {
                drop(cached_avatar_data);
                let avatar_data = self.get_remote_avatar(author).await?;
                self.cached_avatars
                    .insert(author.id.get(), Arc::clone(&avatar_data));
                return Some(avatar_data);
            }

            return Some(Arc::clone(cached_avatar_data.value()));
        }

        // Not cached; Try and cache it before returning.
        let avatar_data = self.get_remote_avatar(author).await?;
        self.cached_avatars
            .insert(author.id.get(), Arc::clone(&avatar_data));
        Some(avatar_data)
    }

    /// Displays a notification for `msg`.
    async fn display_notification(&self, msg: Message) {
        let mut notification = Notification::new();
        let mut notification = notification
            .summary(msg.author.display_name())
            .body(&msg.content)
            .appname("DNotify");

        if let Some(avatar_data) = self.get_avatar(&msg.author).await {
            notification = notification.image_data(avatar_data.image.clone());
        }

        if let Err(error) = notification.show_async().await {
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
