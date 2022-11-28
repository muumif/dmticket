extern crate redis;
use crate::Error;
use serenity::utils::Colour;

/// Create a support ticket
#[poise::command(slash_command)]
pub async fn ticket(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let author = ctx.author();
    let guild = ctx.partial_guild().await.unwrap();

    let client = redis::Client::open(dotenv!("REDIS_URI")).unwrap();
    let mut con = client.get_async_connection().await?;

    let result: String = redis::cmd("HGET")
        .arg(&[guild.id.to_string(), "question1".to_string()])
        .query_async(&mut con)
        .await?;

    let dm = author.create_dm_channel(ctx).await.unwrap();
    dm.say(ctx, result).await.unwrap();

    ctx.send(|b| {
        b.embed(|b| b.description("The bot will has sent you a message in DM-s!\n").title("Check your DM-s!").colour(Colour::BLITZ_BLUE))
            .ephemeral(true)        
    })
    .await?;
    Ok(())
}