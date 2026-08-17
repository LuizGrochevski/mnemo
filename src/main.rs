mod db;
mod models;
mod sm2;

use anyhow::Result;
use chrono::{Duration, Local};
use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "mnemo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Cria um novo deck
    Deck { name: String },
    /// Adiciona um card a um deck existente
    Add { deck: String, front: String, back: String },
    /// Lista todos os decks
    Decks,
    /// Revisa os cards do dia em um deck
    Review { deck: String },
}

fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = db::init_db("mnemo.db")?;

    match cli.command {
        Commands::Deck { name } => {
            db::create_deck(&conn, &name)?;
            println!("Deck '{}' criado.", name);
        }
        Commands::Add { deck, front, back } => {
            db::add_card(&conn, &deck, &front, &back)?;
            println!("Card adicionado ao deck '{}'.", deck);
        }
        Commands::Decks => {
            for d in db::list_decks(&conn)? {
                println!("- {}", d);
            }
        }
        Commands::Review { deck } => {
            let cards = db::due_cards(&conn, &deck)?;
            if cards.is_empty() {
                println!("Nenhum card pra revisar hoje no deck '{}'.", deck);
                return Ok(());
            }
            for mut card in cards {
                println!("\nFrente: {}", card.front);
                ask("(Enter pra ver o verso) ");
                println!("Verso: {}", card.back);

                let quality: u8 = loop {
                    let input = ask("Quão bem você lembrou? (0-5): ");
                    if let Ok(q) = input.parse::<u8>() {
                        if q <= 5 {
                            break q;
                        }
                    }
                    println!("Digite um número de 0 a 5.");
                };

                let (interval, repetition, ease_factor) =
                    sm2::sm2(quality, card.repetition, card.ease_factor, card.interval);

                card.interval = interval;
                card.repetition = repetition;
                card.ease_factor = ease_factor;
                card.due_date = Local::now().date_naive() + Duration::days(interval as i64);

                db::update_card(&conn, &card)?;
            }
            println!("\nRevisão concluída!");
        }
    }

    Ok(())
}
