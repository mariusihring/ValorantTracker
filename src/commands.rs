use crate::api;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn get_player(
    ctx: Context<'_>,
    #[description = "Player Name"] name: String,
    #[description = "Player Tag"] tag: String,
) -> Result<(), Error> {
    let player = api::get_player::get_rank("", &name, &tag, &Default::default())
        .await
        .expect("TODO: panic message");

    ctx.say(format!("<@{}> \n {}", ctx.author().id, player))
        .await?;
    Ok(())
}
