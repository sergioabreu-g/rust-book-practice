mod hello_world;
mod guessing_game;
mod grep;
mod multi_threaded_server;

fn main() {
    //hello_world::say_hello();
    //guessing_game::play_game();
    //grep::start();
    multi_threaded_server::start_server();
}