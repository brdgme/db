use uuid::Uuid;
use chrono::NaiveDateTime;

pub struct User {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub pref_colors: Vec<String>,
    pub login_confirmation: Option<String>,
    pub login_confirmation_at: Option<NaiveDateTime>,
}

pub struct NewUser<'a> {
    pub name: &'a str,
    pub pref_colors: &'a [&'a str],
    pub login_confirmation: Option<&'a str>,
    pub login_confirmation_at: Option<&'a NaiveDateTime>,
}

pub struct UserEmail {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub email: String,
    pub is_primary: bool,
}

pub struct NewUserEmail<'a> {
    pub user_id: Uuid,
    pub email: &'a str,
    pub is_primary: bool,
}

pub struct UserAuthToken {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
    pub token: String,
}

pub struct NewUserAuthToken<'a> {
    pub user_id: Uuid,
    pub token: &'a str,
}

pub struct GameType {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
}

pub struct NewGameType<'a> {
    pub name: &'a str,
}

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

pub struct NewGameVersion<'a> {
    pub game_type_id: Uuid,
    pub name: &'a str,
    pub uri: &'a str,
    pub is_public: bool,
    pub is_deprecated: bool,
}

pub struct Game {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_version_id: Uuid,
    pub is_finished: bool,
    pub game_state: String,
}

pub struct NewGame<'a> {
    pub game_version_id: Uuid,
    pub is_finished: bool,
    pub game_state: &'a str,
}

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

pub struct GameLog {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_id: Uuid,
    pub body: String,
    pub is_public: bool,
}

pub struct NewGameLog<'a> {
    pub game_id: Uuid,
    pub body: &'a str,
    pub is_public: bool,
}

pub struct GameLogTarget {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub game_log_id: Uuid,
    pub user_id: Uuid,
}

pub struct NewGameLogTarget {
    pub game_log_id: Uuid,
    pub user_id: Uuid,
}
