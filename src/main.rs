mod game_2d;
mod game_3d;

fn main() {
    let mut pattern: String = String::from("2d");
    if std::env::args().len() == 2 {
        pattern = std::env::args().nth(1).expect("no game mode given");
    }
    match pattern.as_str() {
        "console" => {
            game_2d::console_game(); // TODO exit game-of-life on key('q' or 'ESC') hit
        }
        "2d" => game_2d::game_2d(),
        "3d" => game_3d::game_3d(),
        _ => {
            println!("unknown game mode");
        }
    }
}
