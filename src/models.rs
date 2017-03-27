use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::{users, user_emails, user_auth_tokens, game_types, game_versions, games, game_players,
             game_logs, game_log_targets};

#[derive(Identifiable, Queryable, Associations)]
#[has_many(user_emails)]
#[has_many(user_auth_tokens)]
#[has_many(game_players)]
#[has_many(game_log_targets)]
pub struct User {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub pref_colors: Vec<String>,
    pub login_confirmation: Option<String>,
    pub login_confirmation_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pref_colors: &'a [&'a str],
    pub login_confirmation: Option<&'a str>,
    pub login_confirmation_at: Option<&'a NaiveDateTime>,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct UserEmail {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub email: String,
    pub is_primary: bool,
}

#[derive(Insertable)]
#[table_name="user_emails"]
pub struct NewUserEmail<'a> {
    pub user_id: Uuid,
    pub email: &'a str,
    pub is_primary: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct UserAuthToken {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub token: String,
}

#[derive(Insertable)]
#[table_name="user_auth_tokens"]
pub struct NewUserAuthToken<'a> {
    pub user_id: Uuid,
    pub token: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(game_versions)]
pub struct GameType {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="game_types"]
pub struct NewGameType<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(GameType)]
#[has_many(games)]
pub struct GameVersion {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_type_id: Uuid,
    pub name: String,
    pub uri: String,
    pub is_public: bool,
    pub is_deprecated: bool,
}

#[derive(Insertable)]
#[table_name="game_versions"]
pub struct NewGameVersion<'a> {
    pub game_type_id: Uuid,
    pub name: &'a str,
    pub uri: &'a str,
    pub is_public: bool,
    pub is_deprecated: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(GameVersion)]
#[has_many(game_players)]
#[has_many(game_logs)]
pub struct Game {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_version_id: Uuid,
    pub is_finished: bool,
    pub game_state: String,
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame<'a> {
    pub game_version_id: Uuid,
    pub is_finished: bool,
    pub game_state: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Game)]
#[belongs_to(User)]
pub struct GamePlayer {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_id: Uuid,
    pub user_id: Uuid,
    pub position: i32,
    pub color: String,
    pub has_accepted: bool,
    pub is_turn: bool,
    pub is_eliminated: bool,
    pub is_winner: bool,
}

#[derive(Insertable)]
#[table_name="game_players"]
pub struct NewGamePlayer<'a> {
    pub game_id: Uuid,
    pub user_id: Uuid,
    pub position: i32,
    pub color: &'a str,
    pub has_accepted: bool,
    pub is_turn: bool,
    pub is_eliminated: bool,
    pub is_winner: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Game)]
#[has_many(game_log_targets)]
pub struct GameLog {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_id: Uuid,
    pub body: String,
    pub is_public: bool,
}

#[derive(Insertable)]
#[table_name="game_logs"]
pub struct NewGameLog<'a> {
    pub game_id: Uuid,
    pub body: &'a str,
    pub is_public: bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(GameLog)]
#[belongs_to(User)]
pub struct GameLogTarget {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_log_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[table_name="game_log_targets"]
pub struct NewGameLogTarget {
    pub game_log_id: Uuid,
    pub user_id: Uuid,
}
