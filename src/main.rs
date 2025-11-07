pub mod test_game;
use purrengine::engine_builder::builder::Builder;
use test_game::test_game_scene::TestGameScene;
use test_game::test_game_ui::TestGameUI;

fn main() {
    Builder::new()
        .configure(|options| {
            options.app_name = String::from("Test Game");
        })
        .register_module::<TestGameScene>()
        .register_module::<TestGameUI>()
        .build()
        .run();
}
