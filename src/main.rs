use std::path::Component;
use purrengine::engine_assets::asset::Asset;
use purrengine::engine_builder::builder::Builder;
use purrengine::engine_render::components::camera::Camera;

fn main() {
    let mut builder = Builder::new();

    builder.configure(|options| {
        options.app_name = String::from("Test Game");
    });
    builder.add_assets(|assets| assets.add("test".to_string(), Asset::new("Test".to_string())));

    builder.add_scene(|app, scene_id| {
        app.add_actor(scene_id, |actor, actor_id| {
           //app.add_component(actor_id,Box::new(Camera::new(|s|{
               //s.aperture = 45.0;
               //s.orthographic = false;
           //})) );
        });
    });

    builder.run();
}

/*

*/
