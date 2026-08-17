use anyhow::Result;
use chrono::{Local, NaiveDate};
use rusqlite::{params, Connection};

use crate::models::Card;

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS decks (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );
        CREATE TABLE IF NOT EXISTS cards (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            deck_id      INTEGER NOT NULL,
            front        TEXT NOT NULL,
            back         TEXT NOT NULL,
            interval     INTEGER NOT NULL DEFAULT 0,
            repetition   INTEGER NOT NULL DEFAULT 0,
            ease_factor  REAL NOT NULL DEFAULT 2.5,
            due_date     TEXT NOT NULL,
            FOREIGN KEY(deck_id) REFERENCES decks(id)
        );
        ",
    )?;
    Ok(conn)
}

pub fn create_deck(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("INSERT OR IGNORE INTO decks (name) VALUES (?1)", params![name])?;
    Ok(())
}

pub fn list_decks(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM decks ORDER BY name")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut decks = Vec::new();
    for r in rows {
        decks.push(r?);
    }
    Ok(decks)
}

fn deck_id(conn: &Connection, deck_name: &str) -> Result<i64> {
    let id: i64 = conn.query_row(
        "SELECT id FROM decks WHERE name = ?1",
        params![deck_name],
        |row| row.get(0),
    )?;
    Ok(id)
}

pub fn add_card(conn: &Connection, deck_name: &str, front: &str, back: &str) -> Result<()> {
    let d_id = deck_id(conn, deck_name)?;
    let today = Local::now().date_naive().to_string();
    conn.execute(
        "INSERT INTO cards (deck_id, front, back, due_date) VALUES (?1, ?2, ?3, ?4)",
        params![d_id, front, back, today],
    )?;
    Ok(())
}

pub fn due_cards(conn: &Connection, deck_name: &str) -> Result<Vec<Card>> {
    let d_id = deck_id(conn, deck_name)?;
    let today = Local::now().date_naive().to_string();

    let mut stmt = conn.prepare(
        "SELECT id, front, back, interval, repetition, ease_factor, due_date
         FROM cards WHERE deck_id = ?1 AND due_date <= ?2",
    )?;
    let rows = stmt.query_map(params![d_id, today], |row| {
        let due_str: String = row.get(6)?;
        Ok(Card {
            id: row.get(0)?,
            front: row.get(1)?,
            back: row.get(2)?,
            interval: row.get(3)?,
            repetition: row.get(4)?,
            ease_factor: row.get(5)?,
            due_date: NaiveDate::parse_from_str(&due_str, "%Y-%m-%d").unwrap(),
        })
    })?;

    let mut cards = Vec::new();
    for r in rows {
        cards.push(r?);
    }
    Ok(cards)
}

pub fn update_card(conn: &Connection, card: &Card) -> Result<()> {
    conn.execute(
        "UPDATE cards SET interval = ?1, repetition = ?2, ease_factor = ?3, due_date = ?4 WHERE id = ?5",
        params![
            card.interval,
            card.repetition,
            card.ease_factor,
            card.due_date.to_string(),
            card.id
        ],
    )?;
    Ok(())
}
