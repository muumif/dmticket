mod commands;
use poise::serenity_prelude::{self as serenity};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[macro_use]
extern crate dotenv_codegen;

const DISCORD_TOKEN: &str = dotenv!("DISCORD_TOKEN");
const PRIVATEGUILDID: serenity::GuildId = serenity::GuildId(812033742419001355);

async fn on_ready(
      ctx: &serenity::Context,
      ready: &serenity::Ready,
      framework: &poise::Framework<(), Error>,
) -> Result<(), Error> {
      println!("{} is connected!", ready.user.name);
      let environment = dotenv!("ENVIRONMENT");
      let builder = poise::builtins::create_application_commands(&framework.options().commands);

      if environment == "development" {
            let commands =
                  serenity::GuildId::set_application_commands(&PRIVATEGUILDID, &ctx.http, |commands| {
                  *commands = builder.clone();
      
                  commands
                  })
                  .await;
            println!(
                  "I now have the following guild slash commands: \n{:#?}",
                  commands
                  );
      } else {
            let global_command1 =
            serenity::Command::set_global_application_commands(&ctx.http, |commands| {
                *commands = builder;
                commands
            })
            .await;

            println!(
                  "I now have the following guild slash commands: \n{:#?}",
                  global_command1
              );
      }
  Ok(())
}


#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    // Build our client.
    let client = poise::Framework::builder()
        .token(DISCORD_TOKEN)
        .intents(serenity::GatewayIntents::empty())
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
            ],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}