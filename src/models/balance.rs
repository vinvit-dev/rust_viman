use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::database::Database;

use super::{errors::ErrorResponse, user::User};

#[derive(sqlx::Type, strum_macros::EnumString, Deserialize, Serialize)]
#[strum(serialize_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum BType {
    Cash,
    Card,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Balance {
    pub id: i32,
    pub uid: i32,
    pub balance_type: BType,
    pub name: String,
    pub iban: Option<String>,
    pub balance: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewBalance {
    pub balance_type: BType,
    pub name: String,
    pub iban: Option<String>,
    pub balance: Option<i32>,
}

pub struct BalanceHandler;

impl BalanceHandler {
    pub async fn crate_balance_for_user(
        db: Database,
        user: User,
        new_balance: NewBalance,
    ) -> Result<Balance, ErrorResponse> {
        let q = "INSERT INTO balances (uid, balance_type, name, iban, balance) VALUES ($1, $2, $3, $4, $5) RETURNING id, uid, balance_type, name, iban, balance";
        let balance_balance = if new_balance.balance.is_none() {
            0
        } else {
            new_balance.balance.unwrap()
        };
        let balance = sqlx::query_as::<_, Balance>(q)
            .bind(user.id)
            .bind(new_balance.balance_type)
            .bind(new_balance.name)
            .bind(new_balance.iban)
            .bind(balance_balance)
            .fetch_one(&db.connection)
            .await;

        match balance {
            Ok(balance) => Ok(balance),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 500)),
        }
    }

    pub async fn get_user_balances(
        db: Database,
        user: User,
    ) -> Result<Vec<Balance>, ErrorResponse> {
        let q = "SELECT * FROM balances WHERE uid = $1";
        let balances = sqlx::query_as::<_, Balance>(q)
            .bind(user.id)
            .fetch_all(&db.connection)
            .await;

        match balances {
            Ok(balances) => Ok(balances),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 500)),
        }
    }
}
