#![allow(unused_variables)]
use chrono::Utc;
use discord_game_sdk::{
    Action, Activity, Discord, EventHandler, LobbyID, User, UserID,
};

struct MuEventHandler;

impl EventHandler for MuEventHandler {
    /// Fired when the current user accepts an invitation to join in chat or receives confirmation from Asking to Join.
    fn on_activity_join(&mut self, discord: &Discord<'_, Self>, secret: &str) {}

    /// Fired when the current user accepts an invitation to spectate in chat
    fn on_activity_spectate(&mut self, discord: &Discord<'_, Self>, secret: &str) {}

    /// Fires when a user asks to join the game of the current user.
    fn on_activity_join_request(&mut self, discord: &Discord<'_, Self>, user: &User) {}

    /// Fires when the current user receives an invitation to join or spectate.
    fn on_activity_invite(
        &mut self,
        discord: &Discord<'_, Self>,
        kind: Action,
        user: &User,
        activity: &Activity,
    ){
    }

    /// Fires when a user connected to voice starts or stops speaking.
    fn on_speaking(
        &mut self,
        discord: &Discord<'_, Self>,
        lobby_id: LobbyID,
        member_id: UserID,
        speaking: bool,
    ) {
    }

    /// Fires when the User struct of the currently connected user changes.
    fn on_current_user_update(&mut self, discord: &Discord<'_, Self>) {}

    /// Fires when the current user has updated their voice settings.
    fn on_voice_settings_update(&mut self, discord: &Discord<'_, Self>) {}
}

impl Default for MuEventHandler {
    fn default() -> Self {
        Self {}
    }
}

pub struct Drpc {
    pub app_id: i64,
    pub state: String,
    pub details: String,
    pub party_id: String,
    pub party_amount: u32,
    pub party_max: u32,
    pub l_img_key: String,
    pub l_img_text: String,
    pub s_img_key: String,
    pub s_img_text: String,
    pub start_time: i64,
    pub end_time: i64,
    pub join_secret: String,
    pub spec_secret: String,
}

impl Drpc {
    pub fn default() -> Self {
        Self {
            app_id: 886940899085549568,
            state: "Testing LytSyn".to_string(),
            details: "Alpha LytSyn build".to_string(),
            party_id: "000000".to_string(),
            party_amount: 1,
            party_max: 10,
            l_img_key: "1719750".to_string(),
            l_img_text: "such synergy".to_string(),
            s_img_key: "background".to_string(),
            s_img_text: "owner".to_string(),
            start_time: Utc::now().timestamp(),
            end_time: Utc::now().timestamp(),
            join_secret: "join".to_string(),
            spec_secret: "spec".to_string(),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut discord = Discord::new(self.app_id)?;
        *discord.event_handler_mut() = Some(MuEventHandler::default());

        loop {
            //https://tryitands.ee/
            discord.update_activity(
     &Activity::empty()
                    .with_state(&self.state)
                    .with_details(&self.details)
                    .with_party_id(&self.party_id)
                    .with_party_amount(self.party_amount)
                    .with_party_capacity(self.party_max)
                    .with_large_image_key(&self.l_img_key)
                    .with_large_image_tooltip(&self.l_img_text)
                    .with_small_image_key(&self.s_img_key)
                    .with_small_image_tooltip(&self.s_img_text)
                    .with_spectate_secret(&self.spec_secret)
                    .with_join_secret(&self.join_secret)
                    .with_instance(true)
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

    /// Set the drpc's state.
    pub fn set_state(&mut self, state: String) { self.state = state; }

    /// Set the drpc's details.
    pub fn set_details(&mut self, details: String) { self.details = details; }

    /// Set the drpc's party id.
    pub fn set_party_id(&mut self, party_id: String) { self.party_id = party_id; }

    /// Get a reference to the drpc's party id.
    pub fn party_id(&self) -> &str { self.party_id.as_str() }

    /// Set the drpc's party amount.
    pub fn set_party_amount(&mut self, party_amount: u32) { self.party_amount = party_amount; }

    /// Get a reference to the drpc's party amount.
    pub fn party_amount(&self) -> &u32 { &self.party_amount }

    /// Set the drpc's party max.
    pub fn set_party_max(&mut self, party_max: u32) { self.party_max = party_max; }

    /// Get a reference to the drpc's party max.
    pub fn party_max(&self) -> &u32 { &self.party_max }

    /// Set the drpc's l img key.
    pub fn set_l_img_key(&mut self, l_img_key: String) { self.l_img_key = l_img_key; }

    /// Get a reference to the drpc's l img key.
    pub fn l_img_key(&self) -> &str { self.l_img_key.as_str() }

    /// Set the drpc's l img text.
    pub fn set_l_img_text(&mut self, l_img_text: String) { self.l_img_text = l_img_text; }

    /// Get a reference to the drpc's l img text.
    pub fn l_img_text(&self) -> &str { self.l_img_text.as_str() }

    /// Set the drpc's s img key.
    pub fn set_s_img_key(&mut self, s_img_key: String) { self.s_img_key = s_img_key; }

    /// Get a reference to the drpc's s img key.
    pub fn s_img_key(&self) -> &str { self.s_img_key.as_str() }

    /// Set the drpc's s img text.
    pub fn set_s_img_text(&mut self, s_img_text: String) { self.s_img_text = s_img_text; }

    /// Get a reference to the drpc's s img text.
    pub fn s_img_text(&self) -> &str { self.s_img_text.as_str() }

    /// Get a reference to the drpc's start time.
    pub fn start_time(&self) -> &i64 { &self.start_time }

    /// Get a reference to the drpc's join secret.
    pub fn join_secret(&self) -> &str { self.join_secret.as_str() }

    /// Get a reference to the drpc's spec secret.
    pub fn spec_secret(&self) -> &str { self.spec_secret.as_str() }
}
