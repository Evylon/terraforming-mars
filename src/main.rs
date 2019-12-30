
mod game_state;

fn main() {
    let mut my_state = game_state::init_state();
    my_state.players.push(game_state::player_default(0));
    my_state.players.push(game_state::player_default(1));
    my_state.players.push(game_state::player_default(2));
    println!("{:?}", my_state);
}
