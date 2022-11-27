use crate::Error;
use serenity::utils::Colour;

/// About command
#[poise::command(slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| b.description("Simple ping command!").title("ping").colour(Colour::BLITZ_BLUE))
            .ephemeral(true)        
    })
    .await?;

    Ok(())
}