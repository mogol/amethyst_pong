mod game;

#[no_mangle]
fn start_pong_game() {
    game::start();
}
