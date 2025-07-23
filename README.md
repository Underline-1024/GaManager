# GaManager
GaManager is a game manager that runs on the terminal. It can help you manage scattered game files.
## Install
You can download the pre-compiled version [here](https://github.com/Underline-1024/GaManager/releases/tag/main).
Don't forget to decompress and set the environment variables.

Or you can do the compilation manually.
`git clone https://github.com/Underline-1024/GaManager.git
cd GaManager
cargo build --release
`

## Command List
list ：List all the games that have been entered into the database.
add ：Add a game to the database. You can input the downloaded games into the database by using --name(-n) , --info(-i) , and --path(-p). Here --info is optional.
start ：Start a game from the database. You can start a game by using --name(-n).
remove ：Remove a game from the database. You can remove a game by using --name(-n).
edit ：Edit a game in the database. You can use the --name (-n) option to specify the game that needs to be changed. You can use the --field(-f) option to specify the fields that need to be edited (name, info, and path).
scan ：Scan the specified directory and add the game to the database. You can use the --path (-p) option to specify the directory to be scanned.
help ：Print this message or the help of the given subcommand(s).
## Notes
The scan command currently only supports the detection of Unity and Godot games. We are working on expanding its capabilities to recognize more types of games in future updates.
