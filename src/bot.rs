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
use std::time::Instant;
use chrono::{Datelike, NaiveDate};
use serenity::model::gateway::Activity;
use crate::persistence::establish_database_connection;
use crate::timetable::{get_timetable_by_user_id, get_timetable_id_by_user_id_if_it_exists, initialise_timetable};
use crate::user::{create_user_by_id, delete_user_by_id};

// For code simplicity
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const WELCOME_MESSAGE: &str = "\
        To start using the bot, send a message with the following format: ``$start <synergetic_id>`` \
        Where your Synergetic ID is the number found on your student card which looks like this: ``102760``\n\
        **Ensure your message follows the format with no other letters or numbers**\n\
        **Please Note: Properly formatted yet incorrect IDs will be accepted by the bot and must be removed manually by support**\n\
        ``For security reasons, you may choose to send this message in a DM to the bot``";

#[group]
#[commands(
    help,
    start,
    tt,
    compete,
    play
)]
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
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, WELCOME_MESSAGE).await?;

    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let now = Instant::now();

    let pool = establish_database_connection().await?;
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

    // Used to check if timetable hasn't been fetched recently
    let today = NaiveDate::parse_from_str(chrono::offset::Local::now().format("%Y-%m-%d").to_string().as_str(), "%Y-%m-%d").unwrap();

    // If a timetable exists, make sure that it was not fetched recently
    if get_timetable_id_by_user_id_if_it_exists(user_id, &pool).await? != 0 &&
        get_timetable_by_user_id(user_id, &pool).await?.fetched_date
            .signed_duration_since(today).num_days() > -7 {

        msg.reply(ctx, "\
        Your timetable has already been fetched in the past 7 days, try again later.\n\
        **If your timetable was fetched with the wrong Synergetic ID, contact support:** https://discord.gg/NU2hVUnj").await?;
        return Ok(());
    }

    let reply_msg = format!(
        "**SUCCESS**\n\
        The bot has fetched the timetable associated with Synergetic ID ``{}`` and assigned it to your account.\n\
        **If you have entered your ID incorrectly, please contact support:** https://discord.gg/NU2hVUnj"
        , synergetic_id);

    // Delete user so that creating user works all the time.
    delete_user_by_id(user_id, &pool).await?;
    let user = create_user_by_id(user_id, synergetic_id, "kbpalletenne@student.mgs.vic.edu.au", "12062004", &pool).await?;

    // Check if timetable exists on myMGS API
    let timetable_response = initialise_timetable(user_id, &pool).await?;
    let timetable_response = timetable_response.as_str();
    match timetable_response {
        "successful" => {msg.reply(ctx, reply_msg).await?;}
        _ => {
            msg.reply(ctx, "No such timetable exists. Ensure your Synergetic ID is correct.").await?;
            delete_user_by_id(user_id, &pool).await?;
        }
    }

    //msg.reply(ctx,reply_msg).await?;
    println!("Fetched timetable {} and created user: {}ms", synergetic_id, now.elapsed().as_millis());
    Ok(())
}

#[command]
async fn tt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let functions: Vec<&str> = args.message().split(" ").collect();

    let error_message = format!("Command error - ``{}`` is not a recognised command", functions[0]);
    match functions[0] {
        "today" => {},
        "tomorrow" => {},
        _ => { msg.reply(ctx, error_message).await?; }
    }



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