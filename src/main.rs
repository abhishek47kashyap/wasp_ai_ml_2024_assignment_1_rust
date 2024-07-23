mod resources;

use resources::game::Game;

fn main()
{
    let g: Game = Game::new();
    g.run();
    
}
