extern crate redis;
use crate::Error;
use poise::{serenity_prelude::{UserId}};
use serenity::utils::Colour;
use serenity::model::Permissions;

/// Setup server ticket
#[poise::command(slash_command)]
pub async fn ticket_setup(
      ctx: poise::Context<'_, (), Error>, 
      #[description = "First question!"] question: String
) -> Result<(), Error> {
      let client = redis::Client::open(dotenv!("REDIS_URI")).unwrap();
      let mut con = client.get_async_connection().await?;

      let guild = ctx.partial_guild().await.unwrap();
      let user_id: UserId = ctx.author().id;

      let is_admin = guild.member_permissions(&ctx, user_id).await.map(|it| it.contains(Permissions::ADMINISTRATOR)).unwrap();

      if is_admin {
            redis::cmd("HSET").arg(&[guild.id.to_string(),"question1".to_string(), question.to_string()]).query_async(&mut con).await?;
            ctx.send(|b| {
                  b.embed(|b| b.description("First question is set!").title("Setup started!").colour(Colour::BLITZ_BLUE))      
            })
            .await?;
      } else {
            ctx.send(|b| {
                  b.embed(|b| b.description("You don't have the privileges to use this command!").title("Error!").colour(Colour::DARK_RED))
                  .ephemeral(true)        
            })
            .await?;
      }


      Ok(())
}