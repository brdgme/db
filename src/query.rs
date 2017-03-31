use postgres::Connection;
use uuid::Uuid;
use rand::{self, Rng};

use errors::*;
use models::*;

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
    let u = create_user_by_name(email, conn)?;
    let ue = create_user_email(&NewUserEmail {
        user_id: &u.id,
        email: email,
        is_primary: true,
    }, conn)?;
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
}

