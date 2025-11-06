mod test_game;

use engine_core::settings::Settings;
use engine_runner::runner::Runner;
use purrengine::{engine_core, engine_runner};

fn main() {
    let settings = Settings {
        app_name: String::from("Test Game"),
    };

    let mut runner = Runner::new(settings);

    let mut test_scene = test_game::test_scene::TestScene::new();
    
    

    //test_scene.add_actor();

    runner.add_scene(Box::new(test_scene));

    runner.init();

    runner.run();
}
