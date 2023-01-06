#![cfg(feature = "bot")]

use serenity::async_trait;
use serenity::model::prelude::AttachmentType;
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

            let result = silence_run_for_ms(&mut emu, 1000.0);
            let mut att = Vec::<AttachmentType>::new();

            let screen = emu.get_screen();
            let width  = screen.width();
            let height = screen.height();
            let pixels = screen.pixels();
            let mut png = Image::new(width as u32, height as u32);
            for (i, el) in pixels.iter().enumerate() {
                png.pixels[i] = RGBA24{r: (el >> 24) as u8, g: (el >> 16) as u8, b: (el >> 8) as u8, a: *el as u8}
            }
            let mut png_file = Vec::<u8>::new();
            if let Err(err) = png.write_png(&mut png_file) {
                println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err);
                drop(png_file);
            } else {
                att.push((png_file.as_slice(), "image.png").into());
            }

            use emulator::emulator::StepResult;
            match result {
                StepResult::HLT => {
                    let output = emu.get_output();
                    if let Err(err) = msg.channel_id.send_files(&ctx.http, att, |m| m.content(format!("Program exited: ```\n{}```", output))).await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                },
                StepResult::Continue => {
                    let output = emu.get_output();
                    if let Err(err) = msg.channel_id.send_files(&ctx.http, att, |m| m.content(format!("Program ran for more than 1000ms: ```\n{}```", output))).await {
                        println!("\x1b[1;93mDiscord bot warning: Unable to send message, reason: {}\x1b[0;0m", err)
                    };
                },
                StepResult::Error => {
                    let output = emu.get_output();
                    if let Err(err) = msg.channel_id.send_files(&ctx.http, att, |m| m.content(format!("Program exited with error: ```ansi\n{}```Output: ```\n{}```",
                        emu.get_err().unwrap(), output))
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

#[derive(Clone)]
pub struct RGBA24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Clone)]
pub struct Image {
    pub pixels: Vec<RGBA24>,
    pub width : u32,
    pub height: u32
}

#[allow(dead_code)]
impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            pixels: vec![RGBA24{r: 0, g: 0, b: 0, a: 255}; (width*height) as usize],
            width, height
        }
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, c: RGBA24) {
        self.pixels[(y * self.height + x) as usize] = c
    }
    pub fn get_pixel(&mut self, x: u32, y: u32) -> &RGBA24 {
        &self.pixels[(y * self.height + x) as usize]
    }

    pub fn write_png<W: std::io::Write>(&self, target: &mut W) -> std::io::Result<()> {
        let mut encoder = png::Encoder::new(target, self.width, self.height);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut raw_color: Vec<u8> = Vec::new();
        for c in self.pixels.iter() {
            raw_color.push(c.r);
            raw_color.push(c.g);
            raw_color.push(c.b);
            raw_color.push(c.a);
        }

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&raw_color)?;
        Ok(())
    }
}
