use postgres::Connection;

use errors::*;
use models::*;

pub struct UserByEmail {
    pub user: User,
    pub user_email: UserEmail,
}

pub fn find_user_by_email(email: &str, conn: &Connection) -> Result<UserByEmail> {
    for row in &conn.query("
        SELECT ue.*, u.*
        FROM user_emails ue
        INNER JOIN users u
        ON (ue.user_id = u.id)
        WHERE ue.email = $1
        LIMIT 1",
                           &[&email])? {
        return Ok(UserByEmail {
                      user: User::from_row(&row, "u."),
                      user_email: UserEmail::from_row(&row, "ue."),
                  });
    }
    Err("could not find user".into())
}
