use serenity::prelude::*;
use serenity::model::gateway::Ready;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} ist verbunden!", ready.user.name);
    }
}

fn main() {
    // Laden der Umgebungsvariablen aus der .env-Datei
    dotenv::dotenv().ok();

    // Extrahieren des Discord-Bot-Tokens aus den Umgebungsvariablen
    let token = env::var("DISCORD_TOKEN")
        .expect("Fehler beim Laden des Bot-Tokens aus der Umgebungsvariablen");

    // Erstellt einen neuen Discord-Client mit dem geladenen Token
    let mut client = Client::new(&token, Handler)
        .expect("Fehler beim Erstellen des Clients");

    // Startet den Discord-Bot
    if let Err(why) = client.start() {
        println!("Fehler beim Starten des Bots: {:?}", why);
    }
}
