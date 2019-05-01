extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::{
    core::transform::TransformBundle,
    ui::{DrawUi, UiBundle},
};

mod systems;
mod pong;

use pong::Pong;
 
pub fn start() -> amethyst::Result<()>{
    // We'll put the rest of the code here.
    let mut logger_config = amethyst::LoggerConfig::default();
    logger_config.level_filter = amethyst::LogLevelFilter::Warn;
    amethyst::start_logger(logger_config);

    use amethyst::utils::application_root_dir;

    let path = format!("{}/resources/display_config.ron", application_root_dir());

    let config = DisplayConfig::load(&path);

    use amethyst::input::InputBundle;
    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path).unwrap();

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String,String>::new())?
        .with_bundle(input_bundle)?
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
