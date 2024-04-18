use actions::XrActionPlugin;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    render::{pipelined_rendering::PipelinedRenderingPlugin, RenderPlugin},
    utils::default,
    window::{PresentMode, Window, WindowPlugin},
};
use bevy_xr::camera::XrCameraPlugin;
use init::XrInitPlugin;
use render::XrRenderPlugin;

pub mod actions;
pub mod camera;
pub mod error;
pub mod extensions;
pub mod graphics;
pub mod init;
pub mod layer_builder;
pub mod render;
pub mod resources;
pub mod types;

pub fn add_xr_plugins<G: PluginGroup>(plugins: G) -> PluginGroupBuilder {
    plugins
        .build()
        .disable::<RenderPlugin>()
        .disable::<PipelinedRenderingPlugin>()
        .add_before::<RenderPlugin, _>(bevy_xr::session::XrSessionPlugin)
        .add_before::<RenderPlugin, _>(XrInitPlugin {
            app_info: default(),
            exts: default(),
            blend_modes: default(),
            backends: default(),
            formats: Some(vec![wgpu::TextureFormat::Rgba8UnormSrgb]),
            resolutions: default(),
            synchronous_pipeline_compilation: default(),
        })
        .add(XrRenderPlugin)
        .add(XrCameraPlugin)
        .add(XrActionPlugin)
        .set(WindowPlugin {
            #[cfg(not(target_os = "android"))]
            primary_window: Some(Window {
                transparent: true,
                present_mode: PresentMode::AutoNoVsync,
                // title: self.app_info.name.clone(),
                ..default()
            }),
            #[cfg(target_os = "android")]
            primary_window: None, // ?
            #[cfg(target_os = "android")]
            exit_condition: bevy::window::ExitCondition::DontExit,
            #[cfg(target_os = "android")]
            close_when_requested: true,
            ..default()
        })
}