/*
Needed stuff.
[] Add all items to the db.
[] ADDING loot lists to the db.
[] Adding attendence to the db.
[] Adding handed out loot to the db.
[] Calculating prios according to a given formula (? maybe in javascript so can be supplied by the programmer ?)
[] Web view.
*/

mod main;

use ::sqlx::postgres::*;

async fn add_user_attendence(pool: &PgPool, uid: i32, raid_id: i32) -> anyhow::Result<()> {
    let rec = sqlx::query(r#"INSERT INTO attendence(user_id, raid_id) ("$1", "$2")"#)
        .bind(uid)
        .bind(raid_id)
        .fetch_one(pool)
        .await?;
    Ok(())
}

async fn add_received_loot(pool: &PgPool, uid: i32, iid: i32) -> anyhow::Result<()> {
    let rec = sqlx::query(
        r#"INSERT INTO archived_loot(received_user_id, received_item_id) ("$1", "$2")"#,
    )
    .bind(uid)
    .bind(iid)
    .fetch_one(pool)
    .await?;
    Ok(())
}

// async fn add_prio_row(pool : &PgPool, row : LootRow, uid : i32, raid_code : i32) -> anyhow::Result<()> {
//     sqlx::query(r#"INSERT INTO loot_prios(raid_code,user_id,priority, col_1_item_id, col_2_item_id) VALUES ("$1", "$2", "$3")"#)
//         .bind(raid_code)
//         .bind(uid)
//         .bind(row.prio)
//         .bind(row.col1)
//         .bind(row.col2)
// }
