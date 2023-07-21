use bevy::winit::WinitWindows;
use bevy::{prelude::*, window::PrimaryWindow};
use winit::window::Icon;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
pub struct BackgroundHandle {
    handle: Handle<Image>,
}

#[derive(Resource)]
pub struct BackgroundSpriteSize {
    width: u32,
    height: u32,
}

fn setupBounds(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    assets: Res<Assets<Image>>,
    bg: Res<BackgroundHandle>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if *handle == bg.handle {
                    let img = assets.get(&bg.handle.clone()).unwrap();

                    let bg_size = BackgroundSpriteSize {
                        width: img.texture_descriptor.size.width,
                        height: img.texture_descriptor.size.height,
                    };

                    commands.insert_resource(bg_size);
                }
            }
            _ => {}
        }
    }
}

pub fn setup_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
    main_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {
        return;
    };

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}
