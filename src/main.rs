//use purrengine::engine_assets::asset::Asset;
//use purrengine::engine_builder::builder::Builder;
//use purrengine::engine_render::components::camera::Camera;
//use std::path::Component;

fn main() {
    /*
        let mut builder = Builder::new()
            .configure(|options| {
                options.app_name = String::from("Test Game");
            })
            .add_assets(|assets| assets.add("test".to_string(), Asset::new("Test".to_string())));

        builder.add_scene(|scene| {
            scene.add_actor(|actor| {});
            scene.add_actor(|actor| {});
        });

        builder.run();

    */

    TestBuilder::new().add_scene().run();
}

struct TestBuilder {}

impl TestBuilder {
    fn new() -> Self {
        println!("Creating");
        Self {}
    }

    fn add_scene(&mut self) -> &mut self {
        println!("Adding scene");
        self
    }

    fn run(&mut self) {
        println!("running...");
        self
    }
}
