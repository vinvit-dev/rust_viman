use viman::*;
use self::models::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::SelectableHelper;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(status.eq(true))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error users loading");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.username);
    }
}