extern crate dotenv;

use dotenv::dotenv;
use std::env;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN")
        .expect("token");
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
