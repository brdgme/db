use uuid::Uuid;
use chrono::NaiveDateTime;
use postgres::rows::Row;

pub struct User {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub pref_colors: Vec<String>,
    pub login_confirmation: Option<String>,
    pub login_confirmation_at: Option<NaiveDateTime>,
}

impl User {
    pub fn from_row(row: &Row, prefix: &str) -> Self {
        Self {
            id: row.get(format!("{}id", prefix).as_ref()),
            created_at: row.get(format!("{}created_at", prefix).as_ref()),
            updated_at: row.get(format!("{}updated_at", prefix).as_ref()),
            name: row.get(format!("{}name", prefix).as_ref()),
            pref_colors: row.get(format!("{}pref_colors", prefix).as_ref()),
            login_confirmation: row.get(format!("{}login_confirmation", prefix).as_ref()),
            login_confirmation_at: row.get(format!("{}login_confirmation_at", prefix).as_ref()),
        }
    }
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

impl UserEmail {
    pub fn from_row(row: &Row, prefix: &str) -> Self {
        Self {
            id: row.get(format!("{}id", prefix).as_ref()),
            created_at: row.get(format!("{}created_at", prefix).as_ref()),
            updated_at: row.get(format!("{}updated_at", prefix).as_ref()),
            user_id: row.get(format!("{}user_id", prefix).as_ref()),
            email: row.get(format!("{}email", prefix).as_ref()),
            is_primary: row.get(format!("{}is_primary", prefix).as_ref()),
        }
    }
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
