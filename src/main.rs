use deminer::game::Game;

fn main() {
    let mut g = Game::new(3, 3, 2);
    g.plant_bomb((0, 0));
    g.plant_bomb((2, 2));

    print!("{:?}", g);
}
