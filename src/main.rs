use std::path::Path;
use serde::{Serialize, Deserialize};
use sled;
use serde_json;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    name: String,
    info: String,
    path: String,
}
impl Game {
    fn new(name: String, info: String, path: String) -> Game {
        Game {
            name,
            info,
            path,
        }
    }
    fn add_game(&self) -> Result<(), &'static str> {
        //检查路径是否存在
        if !Path::new(&self.path).exists() {
            println!("This path does not exist.");
            return Err("This path does not exist.");
        }
        let serialized = serde_json::to_string(&self).map_err(|_| "Failed to serialize game")?;


        let db = sled::open("./my_games").unwrap();
        if db.contains_key(&self.name).map_err(|_| "Failed to check if key exists")? {
            println!("This game already exists.");
            let _ = db.flush();
            return Err("This game already exists.");
        }
        let _ = db.insert(&self.name, serialized.as_bytes());
        let _ = db.flush();
        Ok(())
    }

    fn start(&self) {
        let path = self.path.clone();
        //启动游戏
        std::process::Command::new(path)
            .status()
            .expect("Failed to start game.");
        println!("The game has started successfully.");
    }
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] 
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "List all the games that have been entered into the database. ")]
    List {
        #[arg(short, long, default_value_t = 0, help = "You can add '--level ' to specify the level of detail for displaying the information. 0 only shows the name, 1 shows the name along with relevant information about the game, 2 shows the name, game information and file path.")]
        level: usize,
    },
    #[command(about = "Add a game to the database. You can input the downloaded games into the database by using --name(-n), --info(-i), and --path(-p). Here --info is optional.")]
    Add {
        #[arg(long, short)]
        name: String,
        #[arg(long, short, default_value_t = String::from("None"))]
        info: String,
        #[arg(long, short)]
        path: String,
    },
    #[command(about = "Start a game from the database. You can start a game by using --name(-n).")]
    Start {
        #[arg(long, short)]
        name: String,
    },
    #[command(about = "Remove a game from the database. You can remove a game by using --name(-n).")]
    Remove {
        #[arg(long, short)]
        name: String,
    },
    #[command(about = "Edit a game in the database. You can use the --name (-n) option to specify the game that needs to be changed. You can use the --field(-f) option to specify the fields that need to be edited (name, info, and path).")]
    Edit {
        #[arg(long, short)]
        field: Field,
        #[arg(long, short)]
        name: String,
    }
}
#[derive(Debug, Clone, ValueEnum)]
enum Field {
    Name,
    Info,
    Path,
}
fn main() {
    let path = Path::new("./my_games");
    if !path.exists() {
        let _tree = sled::open(path).unwrap();
    }
    let args = Args::parse();
    match args.command {
        Commands::List{ level } => {
            let db = sled::open("./my_games").unwrap();
            let mut count:usize = 0;
            for (_key, value) in db.iter().map(|x| x.unwrap()) {
                count += 1;
                let game: Game = serde_json::from_slice(&value).unwrap();
                if level == 0{ 
                    println!("Name: {}", game.name);
                }
                else if level == 1 {
                    println!("Name: {}, Info: {}", game.name, game.info);
                }
                else if level == 2 {
                    println!("Name: {}, Info: {}, Path: {}", game.name, game.info, game.path)
                }
                else {
                    println!("Invalid level. The level ranges only from 0 to 2.");
                }
            }
            if count == 0 {
                println!("There are no games in the database.");
            }
            else {
                println!("Total number of games: {}", count);
            }
            let _ = db.flush();
        }
        Commands::Add { name, info, path } => {
            let game = Game::new(name, info, path);
            match game.add_game() {
                Ok(_) => println!("Game added successfully."),
                Err(e) => eprintln!("{}", e),
            }
        },
        Commands::Start { name } => {
            let db = sled::open("./my_games").unwrap();
            if let Some(value) = db.get(&name).unwrap() {
                let game: Game = serde_json::from_slice(&value).unwrap();
                game.start();
                println!("The game has started successfully.");
            } else {
                println!("This game does not exist.");
            }
            let _ = db.flush();
        },
        Commands::Remove { name } => {
            let db = sled::open("./my_games").unwrap();
            if db.contains_key(&name).unwrap() {
                let _ = db.remove(&name);
                println!("The data has been removed successfully.");
            } else {
                println!("This game does not exist.");
            }
            let _ = db.flush();
        },
        Commands::Edit { name, field } => { 
            let db = sled::open("./my_games").unwrap();
            if let Some(value) = db.get(&name).unwrap() {
                let mut game: Game = serde_json::from_slice(&value).unwrap();
                match field {
                    Field::Name => {
                        println!("Please enter the new name:");
                        std::io::stdin().read_line(&mut game.name).expect("Failed to read name.");
                    },
                    Field::Info => {
                        println!("Please enter the new info:");
                        std::io::stdin().read_line(&mut game.info).expect("Failed to read info.");
                    },
                    Field::Path => {
                        println!("Please enter the new path:");
                        std::io::stdin().read_line(&mut game.path).expect("Failed to read path.");
                    },
                }
                let _ = db.remove(&name);
                
                let serialized = serde_json::to_string(&game).map_err(|_| "Failed to serialize game");
                let _ = db.insert(&name, serialized.unwrap().as_bytes());
                let _ = db.flush();
                println!("The data has been edited successfully.");
            }
            else {
                println!("This game does not exist.");
            }
        }
    }
}
