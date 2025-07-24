use std::{
    error::Error,
    fs::OpenOptions,
    io::Write,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dotenv::dotenv;
use mcping::{self, Player};
use std::boxed::Box;
use std::env;

struct McInfo {
    addr: String,
    timestamp_probed: u64,
    latency: u64,
    version: String,
    max_players: i64,
    online_players: i64,
    sample_players: Option<Vec<Player>>,
}

impl McInfo {
    fn new(
        addr: &str,
        tmsp_probed: u64,
        latency: u64,
        version: String,
        max_players: i64,
        online_players: i64,
        sample_players: Option<Vec<Player>>,
    ) -> Self {
        Self {
            addr: addr.to_string(),
            timestamp_probed: tmsp_probed,
            latency,
            version,
            max_players,
            online_players,
            sample_players,
        }
    }

    /// Returns a CSV row like so:
    /// addr, timestamp_probed, latency, version, max_players, online_players, sample_players
    fn make_csv_line(&self) -> String {
        let sample_string = self
            .sample_players
            .as_ref()
            .map_or(String::default(), |ps| {
                ps.iter()
                    .map(|p| format!("{}@{}", p.name, p.id))
                    .collect::<Vec<String>>()
                    .join("!")
            });

        format!(
            "{},{},{},{},{},{},{}",
            self.addr,
            self.timestamp_probed,
            self.latency,
            self.version,
            self.max_players,
            self.online_players,
            sample_string,
        )
    }
}

/// Line contains a \n at the end!
fn update_file(file: &str, line: String) -> Result<(), std::io::Error> {
    let mut data_file = OpenOptions::new().append(true).create(true).open(file)?;

    data_file.write(line.as_bytes()).map(drop)?;

    Ok(())
}

fn probe(file: &str, addr: &str) -> Result<(), Box<dyn Error>> {
    let tmsp_probed: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let (latency, status) = mcping::get_status(&addr, Duration::from_secs(10))?;

    let info = McInfo::new(
        addr,
        tmsp_probed,
        latency,
        status.version.name,
        status.players.max,
        status.players.online,
        status.players.sample,
    );

    update_file(file, format!("{}\n", info.make_csv_line()))?;

    Ok(())
}

const PROBING_INTERVAL_SECONDS: u64 = 30;

fn main() -> Result<(), mcping::Error> {
    dotenv().ok();
    let addr = env::var("MC_ADDR").expect("Failed to read 'MC_ADDR' from env");
    let data_path = env::var("DATA_PATH").unwrap_or_else(|_| "./data.csv".to_string());

    let mut cycles: usize = 0;
    let mut successes: usize = 0;
    let mut errors: usize = 0;

    loop {
        if let Err(e) = probe(&data_path, &addr) {
            eprintln!("Error caught: {e}");
            errors += 1;
        } else {
            successes += 1;
        }
        cycles += 1;

        println!(
            "Probed {cycles:06} times ({successes:06} OK / {errors:06} ERR). (interval={PROBING_INTERVAL_SECONDS}s)"
        );
        thread::sleep(Duration::from_secs(PROBING_INTERVAL_SECONDS));
    }
}
