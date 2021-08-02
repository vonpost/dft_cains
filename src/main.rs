use polars::chunked_array::ChunkedArray;
use sqlx::{postgres::*, query};
use std::{env, i32};
use std::fmt::Error;
use std::fs::File;
use std::future::Future;
use std::io::{self, BufRead};
use std::path::Path;
use polars::prelude::{ChunkApply, CsvReader};
use polars::prelude::SerReader;
use polars::prelude::Series;
use polars::prelude::DataFrame;
use async_std::prelude::*;
#[async_std::main]



async fn main() -> anyhow::Result<()> {

    let host = env::var("PGHOST");
    let url = format!("postgres://postgres:foo@{}/foo", host.unwrap());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url).await?;
    // add_received_loot(&pool, 1,1).await?;
    // fill up database with items
    // populate_users(&pool, "users_text.txt").await?;
    // populate_items(&pool, "t4items.txt").await?;
    // populate_loot_lists_csv(&pool, "lootlists.csv").await?;
    // populate_past_raids(&pool, "past_raids.txt").await?;
    Ok(())
}


async fn populate_loot_lists(pool : &PgPool, df : DataFrame) -> anyhow::Result<()> {
            let users = df.get_column_names();
            for user in users.iter() {
                let uid = sqlx::query!("select id from users where char_name = $1", user)
                    .fetch_one(pool)
                    .await?
                    .id;
                let loot_list = &df[*user].utf8().unwrap();
                let iter : Vec<_> = loot_list.into_iter().map(|x| if x == Some("") { None } else { x }).collect();
                for (i,chnk) in iter.chunks(2).enumerate() {
                    let (col_1, col_2) = (chnk[0] , chnk[1]);
                    if let Some(col_1) = col_1 {
                        let col1 = sqlx::query!("select * from items where name = $1", col_1)
                            .fetch_one(pool)
                            .await?;
                        let col2 = match col_2 {
                            Some(col_2) => Some(sqlx::query!("select * from items where name = $1", col_2)
                                            .fetch_one(pool)
                                            .await?
                                            .id),
                            None => None
                        };
                        let prio_row = LootListRow{ raid : col1.raid.as_ref().unwrap().to_string() , uid , prio : 50-i as i32, col1_id : col1.id, col2_id : col2 };
                        add_lootlist_row(pool, prio_row).await?;
                    }
                }
            }
    Ok(())
}
async fn populate_loot_lists_csv(pool : &PgPool, llcsv : &str) -> anyhow::Result<()> {
    let df = CsvReader::from_path(llcsv)?;
    let df = df.infer_schema(None).has_header(true).finish()?;
    populate_loot_lists(pool, df).await?;
    Ok(())
}

async fn populate_items(pool : &PgPool, items_txt : &str) -> anyhow::Result<()> {
    let lines = read_lines(items_txt)?;
        for line in lines {
            let l = line?;
            let mut sl = l.split_whitespace();
            let raid_name = sl.next().ok_or(Error)?;
            let t : Vec<&str> = sl.collect();
            let item_name = t.join(" ");
            sqlx::query!(r#"INSERT INTO items(name,raid) VALUES ($1, $2)"#, item_name, raid_name)
                    .execute(pool)
                    .await?;
        }
    Ok(())
}

async fn populate_users(pool : &PgPool, users_txt : &str) -> Result<(), sqlx::Error> {
    let lines = read_lines(users_txt)?;
    for line in lines {
        add_user(pool, classname_str_to_user(line?)).await?;
        }
    Ok(())
}

async fn populate_past_raids(pool : &PgPool, dates : &str) -> anyhow::Result<()> {
    let lines = read_lines(dates)?;
    for line in lines {
        let l = line?;
        let mut sl = l.split_whitespace();
        let mut date = sl.next().ok_or(Error)?.split("-");
        let (y,m,d) : (i32,u8,u8)  = (date.next().ok_or(Error)?.parse()?,
                                      date.next().ok_or(Error)?.parse()?,
                                      date.next().ok_or(Error)?.parse()?);
        let date : time::Date = time::Date::try_from_ymd(y,m,d)?;
        let raid_name = sl.collect::<Vec<_>>().join(" ");
        sqlx::query!("insert into past_raids(date,raid) values ($1,$2)", date, raid_name)
            .execute(pool)
            .await?;
    }
    Ok(())
}

async fn add_received_loot(pool : &PgPool, uid : i32, iid : i32, rid : i32) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"INSERT INTO archived_loot(received_user_id, received_item_id, raid_id) VALUES ($1, $2, $3)"#, uid, iid, rid)
        .execute(pool)
        .await?;
    Ok(())
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct LootListRow {
    raid : String,
    uid : i32,
    prio : i32,
    col1_id : i32,
    col2_id : Option<i32>,
}

async fn add_attendence(pool : &PgPool, uid : i32, raid_id : i32) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"INSERT INTO attendence(user_id, raid_id) VALUES ($1, $2)"#, uid, raid_id)
        .execute(pool)
        .await?;
    Ok(())
}

struct User {
    name : String,
    class : String
}

fn classname_str_to_user(classname : String) -> User {
    let mut it = classname.split_whitespace();
    let sname = it.next().unwrap();
    let sclass = it.next().unwrap();
    println!("Adding.. {},{}", sname,sclass);
    User {
        name : sname.to_string(),
        class : sclass.to_string()
    }
}


async fn add_user(pool : &PgPool, user : User) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"INSERT INTO users(char_name, class) VALUES ($1, $2)"#, user.name, user.class)
        .execute(pool)
        .await?;
    Ok(())
}

async fn add_lootlist_row(pool : &PgPool, row : LootListRow) -> Result<(), sqlx::Error> {
    match row.col2_id {
        Some(col2_id) =>
            sqlx::query!(r#"INSERT INTO loot_prios(raid, user_id, priority, col_1_item_id, col_2_item_id) VALUES ($1, $2, $3, $4, $5)"#,
            row.raid,
            row.uid,
            row.prio,
            row.col1_id,
            row.col2_id)
            .execute(pool)
            .await?,
        None =>
            sqlx::query!(r#"INSERT INTO loot_prios(raid, user_id, priority, col_1_item_id) VALUES ($1, $2, $3, $4)"#,
            row.raid,
            row.uid,
            row.prio,
            row.col1_id)
            .execute(pool)
            .await?
    };
    Ok(())
}


