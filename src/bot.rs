use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{StandardFramework, CommandResult, macros::{
    command,
    group
}, Args};

use std::{env, time};
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;
use serenity::model::gateway::Activity;
use crate::timetable::initialise_timetable;

// For code simplicity
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const WELCOME_MESSAGE: &str = "Your timetable could not be found on the bot's database. \
        To start using the bot, send a message with the following format: ``$start <synergetic_id>`` \
        Where your Synergetic ID is the number found on your student card which looks like this: ``102760``\n\
        **Ensure your message follows the format with no other letters or numbers**\n\
        **Please Note: Properly formatted yet incorrect IDs will be accepted by the bot and must be removed manually by support**";

#[group]
#[commands(ping, start, compete, play)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub async fn bot_run() -> Result<()> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$")) // set command prefix to "$"
        .group(&GENERAL_GROUP);

    // Login with bot token from env
    dotenv::dotenv();
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why);
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong").await?;

    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let user_id = i64::from_str(msg.author.id.to_string().as_str()).unwrap();

    if i64::from_str(msg.author.id.to_string().as_str()).unwrap() != 436035620905943041 {
        msg.reply(ctx, "You cannot use that command").await?;

        return Ok(());
    }

    let mut synergetic_id = 0;
    let parsed_string = i32::from_str(args.message());
    match parsed_string {
        Ok(i32) => synergetic_id = parsed_string.unwrap(),
        Err(err) => { msg.reply(ctx,err).await?; return Ok(()); }
    }

    let range: Range<i32> = 4000..110000;

    if !range.contains(&synergetic_id) {
        msg.reply(ctx, "Synergetic ID is either too short or too long").await?;

        return Ok(());
    }

    let reply_msg = format!(
        "The bot will now fetch the timetable associated with user ``{}`` and assign it to your account.\n\
        **If you have entered it incorrectly, please contact support:** https://discord.gg/NU2hVUnj"
        , synergetic_id);

    let timetable_response = initialise_timetable(user_id).await?;
    let timetable_response = timetable_response.as_str();
    match timetable_response {
        "successful" => {msg.reply(ctx, "Successfully fetched timetable").await?;}
        _ => {msg.reply(ctx, "No such timetable exists. Ensure your Synergetic ID is correct.").await?; }
    }

    //msg.reply(ctx,reply_msg).await?;

    Ok(())
}



///////////////////////
// Change Bot Activity
///////////////////////

#[command]
async fn compete(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if i64::from_str(msg.author.id.to_string().as_str()).unwrap() != 436035620905943041 {
        msg.reply(ctx, "You cannot use that command").await?;

        return Ok(());
    }

    let name = args.message();
    ctx.set_activity(Activity::competing(name)).await;

    Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if i64::from_str(msg.author.id.to_string().as_str()).unwrap() != 436035620905943041 {
        msg.reply(ctx, "You cannot use that command").await?;

        return Ok(());
    }

    let name = args.message();
    ctx.set_activity(Activity::playing(name)).await;

    Ok(())
}