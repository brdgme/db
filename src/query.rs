use postgres::Connection;
use uuid::Uuid;
use rand::{self, Rng};
use chrono::{Duration, UTC};

use errors::*;
use models::*;

lazy_static! {
    static ref CONFIRMATION_EXPIRY: Duration = Duration::minutes(30);
    static ref TOKEN_EXPIRY: Duration = Duration::days(30);
}

pub struct UserByEmail {
    pub user: User,
    pub user_email: UserEmail,
}

pub fn create_user_by_name(name: &str, conn: &Connection) -> Result<User> {
    for row in &conn.query("
        INSERT INTO users
        (
            name
        ) VALUES (
            $1
        )
        RETURNING *",
                           &[&name])
                    .chain_err(|| "error creating user")? {
        return Ok(User::from_row(&row, ""));
    }
    Err("unable to create user".into())
}

pub fn find_user(id: &Uuid, conn: &Connection) -> Result<Option<User>> {
    for row in &conn.query("
        SELECT *
        FROM users
        WHERE id=$1
        LIMIT 1
    ",
                           &[id])? {
        return Ok(Some(User::from_row(&row, "")));
    }
    Ok(None)
}

pub fn find_user_by_email(email: &str, conn: &Connection) -> Result<Option<UserByEmail>> {
    for row in &conn.query(&format!("
        SELECT
            {}, {}
        FROM user_emails ue
        INNER JOIN users u
        ON (ue.user_id = u.id)
        WHERE ue.email = $1
        LIMIT 1",
                                    User::select_cols("u", "u_"),
                                    UserEmail::select_cols("ue", "ue_")),
                           &[&email])? {
        return Ok(Some(UserByEmail {
                           user: User::from_row(&row, "u_"),
                           user_email: UserEmail::from_row(&row, "ue_"),
                       }));
    }
    Ok(None)
}

pub fn find_or_create_user_by_email(email: &str, conn: &Connection) -> Result<UserByEmail> {
    if let Some(u) = find_user_by_email(email, conn)? {
        return Ok(u);
    }
    create_user_by_email(email, conn)
}

pub fn create_user_by_email(email: &str, conn: &Connection) -> Result<UserByEmail> {
    let u = create_user_by_name(email, conn)?;
    let ue = create_user_email(&NewUserEmail {
                                    user_id: &u.id,
                                    email: email,
                                    is_primary: true,
                                },
                               conn)?;
    Ok(UserByEmail {
           user: u,
           user_email: ue,
       })
}

pub fn create_user_email(ue: &NewUserEmail, conn: &Connection) -> Result<UserEmail> {
    for row in &conn.query("
        INSERT INTO user_emails
        (
            email,
            user_id,
            is_primary
        ) VALUES (
            $1,
            $2,
            $3
        ) RETURNING *",
                           &[&ue.email, &ue.user_id, &ue.is_primary])? {
        return Ok(UserEmail::from_row(&row, ""));
    }
    Err("could not create user email".into())
}

fn rand_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{}{:05}",
            (rng.gen::<usize>() % 9) + 1,
            rng.gen::<usize>() % 100000)
}

pub fn generate_user_login_confirmation(user_id: &Uuid, conn: &Connection) -> Result<String> {
    let code = rand_code();
    match conn.execute("
        UPDATE users
        SET
            login_confirmation=$1,
            login_confirmation_at=(now() AT TIME ZONE 'utc')
        WHERE id=$2
    ",
                       &[&Some(&code), user_id])? {
        0 => Err("could not update login confirmation".into()),
        _ => Ok(code),
    }
}

pub fn user_login_request(email: &str, conn: &Connection) -> Result<String> {
    let user = find_or_create_user_by_email(email, conn)?.user;

    Ok(match (user.login_confirmation, user.login_confirmation_at) {
           (Some(ref uc), Some(at)) if at + *CONFIRMATION_EXPIRY > UTC::now().naive_utc() => {
               uc.to_owned()
           }
           _ => generate_user_login_confirmation(&user.id, conn)?,
       })
}

pub fn user_login_confirm(email: &str,
                          confirmation: &str,
                          conn: &Connection)
                          -> Result<Option<UserAuthToken>> {
    let user = match find_user_by_email(email, conn)? {
        Some(ube) => ube.user,
        None => return Ok(None),
    };
    Ok(match (user.login_confirmation, user.login_confirmation_at) {
           (Some(ref uc), Some(at)) if at + *CONFIRMATION_EXPIRY > UTC::now().naive_utc() &&
                                       uc == confirmation => {
               Some(create_auth_token(&user.id, conn)?)
           }
           _ => None,
       })
}

pub fn create_auth_token(user_id: &Uuid, conn: &Connection) -> Result<UserAuthToken> {
    for row in &conn.query("
        INSERT INTO user_auth_tokens
        (
            user_id
        ) VALUES (
            $1
        ) RETURNING *",
                           &[user_id])? {
        return Ok(UserAuthToken::from_row(&row, ""));
    }
    Err("could not create user auth token".into())
}

pub fn authenticate(email: &str, token: &Uuid, conn: &Connection) -> Result<Option<UserByEmail>> {
    for row in &conn.query(&format!("
        SELECT
            {}, {}, {}
        FROM users u
        INNER JOIN user_auth_tokens uat
        ON (uat.user_id = u.id)
        INNER JOIN user_emails ue
        ON (ue.user_id = u.id)
        WHERE ue.email = $1
        AND uat.id = $2
        AND uat.created_at > $3
        LIMIT 1",
                                    User::select_cols("u", "u_"),
                                    UserEmail::select_cols("ue", "ue_"),
                                    UserAuthToken::select_cols("uat", "uat_"),
                                    ),
                           &[&email, token, &(UTC::now().naive_utc() - *TOKEN_EXPIRY)])? {
        return Ok(Some(UserByEmail {
                           user: User::from_row(&row, "u_"),
                           user_email: UserEmail::from_row(&row, "ue_"),
                       }));
    }
    Ok(None)
}

pub fn create_game_users(ids: &[Uuid],
                         emails: &[String],
                         conn: &Connection)
                         -> Result<Vec<UserByEmail>> {
    let mut users: Vec<UserByEmail> = vec![];
    for id in ids.iter() {
        users.push(find_user_with_primary_email(id, conn)?
                       .ok_or_else::<Error, _>(|| "unable to find user".into())?);
    }
    for email in emails.iter() {
        users.push(match find_user_with_primary_email_by_email(email, conn)? {
                       Some(ube) => ube,
                       None => create_user_by_email(email, conn)?,
                   });
    }
    Ok(users)
}

pub fn find_user_with_primary_email(id: &Uuid, conn: &Connection) -> Result<Option<UserByEmail>> {
    for row in &conn.query(&format!("
        SELECT {}, {}
        FROM users u
        INNER JOIN user_emails ue
        ON (u.id = ue.user_id)
        WHERE u.id = $1
        AND ue.is_primary = TRUE
        LIMIT 1",
                                    User::select_cols("u", "u_"),
                                    UserEmail::select_cols("ue", "ue_")),
                           &[id])? {
        return Ok(Some(UserByEmail {
                           user: User::from_row(&row, "u_"),
                           user_email: UserEmail::from_row(&row, "ue_"),
                       }));
    }
    Ok(None)
}

pub fn find_user_with_primary_email_by_email(email: &str,
                                             conn: &Connection)
                                             -> Result<Option<UserByEmail>> {
    for row in &conn.query(&format!("
        SELECT {}, {}
        FROM users u
        INNER JOIN user_emails ue
        ON (u.id = ue.user_id)
        INNER JOIN user_emails uef
        ON (u.id = uef.user_id)
        WHERE uef.email = $1
        AND ue.is_primary = TRUE
        LIMIT 1",
                                    User::select_cols("u", "u_"),
                                    UserEmail::select_cols("ue", "ue_")),
                           &[&email])? {
        return Ok(Some(UserByEmail {
                           user: User::from_row(&row, "u_"),
                           user_email: UserEmail::from_row(&row, "ue_"),
                       }));
    }
    Ok(None)
}

pub fn create_game(new_game: &NewGame, conn: &Connection) -> Result<Game> {
    for row in &conn.query("
        INSERT INTO games (
            game_version_id,
            is_finished,
            game_state
        ) VALUES (
            $1,
            $2,
            $3
        )
        RETURNING *",
                           &[&new_game.game_version_id,
                             &new_game.is_finished,
                             &new_game.game_state])? {
        return Ok(Game::from_row(&row, ""));
    }
    Err("error creating game".into())
}

pub fn create_game_version(new_game_version: &NewGameVersion,
                           conn: &Connection)
                           -> Result<GameVersion> {
    for row in &conn.query("
        INSERT INTO game_versions (
            game_type_id,
            uri,
            name,
            is_public,
            is_deprecated
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5
        )
        RETURNING *",
                           &[&new_game_version.game_type_id,
                             &new_game_version.uri,
                             &new_game_version.name,
                             &new_game_version.is_public,
                             &new_game_version.is_deprecated])? {
        return Ok(GameVersion::from_row(&row, ""));
    }
    Err("error creating game version".into())
}

pub fn create_game_type(new_game_type: &NewGameType, conn: &Connection) -> Result<GameType> {
    for row in &conn.query("
        INSERT INTO game_types (
            name
        ) VALUES (
            $1
        )
        RETURNING *",
                           &[&new_game_type.name])? {
        return Ok(GameType::from_row(&row, ""));
    }
    Err("error creating game type".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::NewUserEmail;
    use postgres::Connection;
    use Connections;
    use connect_env;

    lazy_static! {
        static ref CONN: Connections = connect_env().unwrap();
    }

    #[test]
    fn rand_code_works() {
        for _ in 1..100000 {
            let n: usize = rand_code().parse().unwrap();
            assert!(n > 99999, "n <= 99999");
            assert!(n < 1000000, "n >= 1000000");
        }
    }

    fn with_db<F>(closure: F)
        where F: Fn(&Connection)
    {
        let ref conn = *CONN.w.get().unwrap();
        let trans = conn.transaction().unwrap();
        trans.set_rollback();
        closure(conn);
        trans.finish().unwrap();
    }

    #[test]
    #[ignore]
    fn create_user_by_name_works() {
        with_db(|conn| {
                    assert!(create_user_by_name("beefsack", conn).is_ok());
                });
    }

    #[test]
    #[ignore]
    fn find_user_works() {
        with_db(|conn| {
                    assert_eq!(find_user(&Uuid::new_v4(), conn).unwrap(), None);
                    let u = create_user_by_name("beefsack", conn).unwrap();
                    assert!(find_user(&u.id, conn).unwrap().is_some());
                });
    }

    #[test]
    #[ignore]
    fn create_user_email_works() {
        with_db(|conn| {
            assert_eq!(find_user(&Uuid::new_v4(), conn).unwrap(), None);
            let u = create_user_by_name("beefsack", conn).unwrap();
            assert!(create_user_email(&NewUserEmail {
                                           user_id: &u.id,
                                           email: "beefsack@gmail.com",
                                           is_primary: true,
                                       },
                                      conn)
                            .is_ok());
        });
    }

    #[test]
    #[ignore]
    fn login_works() {
        with_db(|conn| {
            let confirmation = user_login_request("beefsack@gmail.com", conn).unwrap();
            let uat = user_login_confirm("beefsack@gmail.com", &confirmation, conn)
                .unwrap()
                .unwrap();
            assert!(authenticate("beefsack@gmail.com", &uat.id, conn)
                        .unwrap()
                        .is_some());
            assert!(authenticate("beefsacke@gmail.com", &uat.id, conn)
                        .unwrap()
                        .is_none());
        });
    }

    #[test]
    #[ignore]
    fn find_user_with_primary_email_works() {
        with_db(|conn| {
                    let ube = create_user_by_email("beefsack@gmail.com", conn).unwrap();
                    let found = find_user_with_primary_email(&ube.user.id, conn)
                        .unwrap()
                        .unwrap();
                    assert_eq!(ube.user.id, found.user.id);
                    assert_eq!("beefsack@gmail.com", ube.user_email.email);
                });
    }

    #[test]
    #[ignore]
    fn find_user_with_primary_email_by_email_works() {
        with_db(|conn| {
            let ube = create_user_by_email("beefsack@gmail.com", conn).unwrap();
            create_user_email(&NewUserEmail {
                                   user_id: &ube.user.id,
                                   email: "beefsack+two@gmail.com",
                                   is_primary: false,
                               },
                              conn)
                    .unwrap();
            let found = find_user_with_primary_email_by_email("beefsack+two@gmail.com", conn)
                .unwrap()
                .unwrap();
            assert_eq!(ube.user.id, found.user.id);
            assert_eq!("beefsack@gmail.com", ube.user_email.email);
        });
    }

    #[test]
    #[ignore]
    fn create_game_works() {
        with_db(|conn| {
            let game_type = create_game_type(&NewGameType { name: "Lost Cities" }, conn).unwrap();
            let game_version = create_game_version(&NewGameVersion {
                                                        game_type_id: &game_type.id,
                                                        uri: "https://example.com/lost-cities-1",
                                                        name: "v1",
                                                        is_public: true,
                                                        is_deprecated: false,
                                                    },
                                                   conn)
                    .unwrap();
            assert!(create_game(&NewGame {
                                     game_version_id: &game_version.id,
                                     is_finished: false,
                                     game_state: "{}",
                                 },
                                conn)
                            .is_ok());
        });
    }
}
