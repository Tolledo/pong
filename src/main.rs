use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    ui::{RenderUi, UiBundle},
    input::{InputBundle, StringBindings},
    audio::{AudioBundle, DjSystemDesc},
};

use crate::pong::Pong;
use crate::audio::Music;

mod pong;
mod system;
mod audio;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_path = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(system::PaddleSystem, "paddle_system", &["input_system"])
        .with(system::MoveBallsSystem, "ball_system", &[])
        .with(system::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(system::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new(assets_path, Pong::default(), game_data)?;
    game.run();

    Ok(())
}
