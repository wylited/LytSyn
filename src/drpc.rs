#![allow(unused_variables)]
use chrono::Utc;
use discord_game_sdk::{Action, Activity, Discord, Entitlement, EventHandler, LobbyID, NetworkChannelID, NetworkPeerID, Relationship, UnixTimestamp, User, UserAchievement, UserID};
use std::{env, io, thread, time};
use std::time::{Duration, SystemTime};

struct MuEventHandler;

impl EventHandler for MuEventHandler {
    /// Fired when the current user accepts an invitation to join in chat or receives confirmation from Asking to Join.
    fn on_activity_join(&mut self, discord: &Discord<'_, Self>, secret: &str) {}

    /// Fired when the current user accepts an invitation to spectate in chat
    fn on_activity_spectate(&mut self, discord: &Discord<'_, Self>, secret: &str) {}

    /// Fires when a user asks to join the game of the current user.
    fn on_activity_join_request(&mut self, discord: &Discord<'_, Self>, user: &User) {}

    /// Fires when the current user receives an invitation to join or spectate.
    fn on_activity_invite( &mut self, discord: &Discord<'_, Self>, kind: Action, user: &User, activity: &Activity) {}

    /// Fires when a user connected to voice starts or stops speaking.
    fn on_speaking(&mut self, discord: &Discord<'_, Self>, lobby_id: LobbyID, member_id: UserID, speaking: bool) {}

    /// Fires when the User struct of the currently connected user changes.
    fn on_current_user_update(&mut self, discord: &Discord<'_, Self>) {}

    /// Fires when the current user has updated their voice settings.
    fn on_voice_settings_update(&mut self, discord: &Discord<'_, Self>) {}
}

impl Default for MuEventHandler{
    fn default() -> Self {
        Self {  }
    }
}

pub struct Drpc {
    pub app_id: i64,
    pub party_amount: u32,
    pub party_max: u32,
    pub start_time: i64,
    pub end_time: i64,
}

impl Drpc {
    pub fn default() -> Self {
        Self {
            app_id: 886940899085549568,
            party_amount: 1,
            party_max: 10,
            start_time: Utc::now().timestamp(),
            end_time: Utc::now().timestamp(),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut discord = Discord::new(self.app_id)?;
        *discord.event_handler_mut() = Some(MuEventHandler::default());
        
        loop {
            discord.update_activity(
            &Activity::empty()
                    .with_state("Tester")
                    .with_details("Testing out RustMU")
                    .with_party_amount(1)
                    .with_party_capacity(20)
                    .with_large_image_key("1719750")
                    .with_large_image_tooltip("such music")
                    .with_small_image_key("background")
                    .with_small_image_tooltip("Developer")
                    .with_start_time(self.start_time),
                |discord, result| {
                    if let Err(error) = result {
                        return eprintln!("failed to update activity: {}", error);
                    }
                },
            );

            discord.run_callbacks()?;
        }
    }
}
