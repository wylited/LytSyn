use std::{env, thread, time, io};

pub struct Handlers;

impl EventHandlers for Handlers {
    fn ready(user: User) {
        println!("User {}#{} logged in...", user.username, user.discriminator);
    }
}


pub struct Drpc {
    pub app_id: u64,
    pub activity: String, 
    pub small_id: String,
}

impl Drpc {
    pub fn default() -> Self { 
        Self {
            app_id: 886940899085549568,
            activity: "Testing out RustMU".to_string(),
            small_id: "sky".to_string(),
        }
    }

    pub fn run(client: Self){

    }
}