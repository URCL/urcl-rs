#![cfg(feature = "bot")]

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::Ready;

use crate::emulator::*;

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!emu") {
            let body;
            if msg.attachments.len() > 0 {
                body = reqwest::get(msg.attachments[0].url.clone()).await.unwrap().text().await.unwrap();
            } else {
                let tmp = msg.content.split("```").collect::<Vec<&str>>();

                if tmp.len() < 3 {
                    if let Err(err) = msg.channel_id.say(&ctx.http, "Expected file or codeblock with URCL source.").await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                    return;
                }
                body = tmp[1].to_string();
            }

            let mut emu = match emulator::silence_emulate(body) {
                Ok(emu) => emu,
                Err(err) => {
                    if let Err(err) = msg.channel_id.say(&ctx.http, format!("Cannot compile URCL code: ```ansi\n{}```", err)).await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                    return;
                }
            };

            use emulator::emulator::StepResult;
            match silence_run_for_ms(&mut emu, 1000.0) {
                StepResult::HLT => {
                    let output = emu.get_output();
                    if let Err(err) = msg.channel_id.say(&ctx.http, format!("Program exited: ```\n{}```", output)).await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                },
                StepResult::Continue => {
                    let output = emu.get_output();
                    if let Err(err) = msg.channel_id.say(&ctx.http, format!("Program ran for more than 1000ms: ```\n{}```", output)).await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                },
                StepResult::Error => {
                    let output = emu.get_output();
                    if let Err(err) = msg.channel_id.say(&ctx.http, format!("Program exited with error: ```ansi\n{}```Output: ```\n{}```",
                        emu.get_err().unwrap(), output)
                    ).await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                },
                _ => (),
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn init_bot(token: &str) -> Result<(), SerenityError> {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler{}).framework(StandardFramework::new()).await.expect("Err creating client");

    client.start().await
}


pub fn silence_run_for_ms(emu: &mut emulator::emulator::EmulatorState, max_time_ms: f64) -> emulator::emulator::StepResult {
    use emulator::emulator::StepResult;

    const BURST_LENGTH: u32 = 1024;
    let start = crate::now();
    let end = start + max_time_ms;
    while crate::now() < end {
        for _ in 0..BURST_LENGTH {
            let result = emu.step();
            match result {
                StepResult::Continue => (),
                _ => {
                    return result;
                }
            }
        }
    }
    return StepResult::Continue
}
