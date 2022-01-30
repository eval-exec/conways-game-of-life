use clap::Parser;

mod game_2d;
mod game_3d;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    mode: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    ttl: u8,
}

fn main() {
    let mut args = Args::parse();
    if args.mode.is_empty() {
        args.mode = String::from("2d");
    }
    match args.mode.as_str() {
        "console" => game_2d::console_game(), // TODO exit game-of-life on key('q' or 'ESC') hit
        "2d" => game_2d::game_2d(),
        "3d" => game_3d::game_3d(),
        _ => {
            println!("unknown game mode");
        }
    }
}
