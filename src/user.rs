
#[derive(Queryable)]
pub struct User {
    discord_user_id: u64, // This is also the user's id. in DB. The unique "snowflake" number (u64, but returned by Discord as a String). Stored as u64 to enable faster SQL queries) , not something like "Endveous#1689" so that users changing their username won't affect the bot.

    synergetic_user_id: u16,
    mgs_email: String, // TODO: Leave this in? Or remove as people will never trust this.
    mgs_password: String,
}