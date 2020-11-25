use bevy::math;
use bevy::{prelude::*, render::camera::Camera};
use bevy_tiled_prototype::level;
use bevy_tiled_prototype::TiledMapCenter;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        .add_startup_system(setup.system())
        .add_system(camera_movement.system())
        .add_system(animate_sprite_system.system())
        .add_system(character_movement.system())
        .add_system(character_intersect.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(bevy_tiled_prototype::TiledMapComponents {
            map_asset: asset_server.load("map1.tmx"),
            center: TiledMapCenter(false),
            origin: Transform::from_scale(Vec3::new(8.0, 8.0, 1.0)),
            ..Default::default()
        })
        .spawn(Camera2dComponents::default());

    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::splat(6.0),
                translation: Vec3::new(30.0 * 8.0, 30.0 * -8.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true));
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            // println!("timer");
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn camera_movement(
    query: Query<(&TextureAtlasSprite, &Transform)>,
    mut cam_query: Query<(&Camera, &mut Transform)>,
) {
    let mut pos = None;

    for (_, t) in query.iter() {
        pos = Some(t.translation);
    }

    if let Some(pos) = pos {
        for (_, mut t) in cam_query.iter_mut() {
            // println!("pos: {:?}", pos);
            t.translation = pos;
        }
    }
}

fn character_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&TextureAtlasSprite, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::zero();
        let scale = transform.scale.x();

        let speed = if keyboard_input.pressed(KeyCode::LShift) {
            0.1
        } else {
            1.0
        };

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(speed, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(speed, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, speed, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, speed, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        if keyboard_input.pressed(KeyCode::X) && scale > 1.1 {
            let scale = scale - 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        transform.translation += time.delta_seconds * direction * 1000.;
    }
}

fn intersect(shape: &level::CollisionShape, rect: &math::Rect<f32>) -> bool {
    match shape {
        level::CollisionShape::Rect(shape) => {
            rect.left <= shape.right
                && rect.right >= shape.left
                && rect.top <= shape.bottom
                && rect.bottom >= shape.top
        }
    }
}

pub fn character_intersect(
    level: Res<Option<level::Level>>,
    mut query: Query<(&TextureAtlasSprite, &Transform)>,
) {
    if level.is_none() {
        return;
    }

    let level = level.as_ref().unwrap();

    for (_, mut transform) in query.iter() {
        //println!("transform: {:?}", transform);
        let mut pixel_coord = transform.translation / 8f32;
        *pixel_coord.y_mut() *= -1f32;

        let character_rect = math::Rect {
            left: pixel_coord.x(),
            right: pixel_coord.x() + 16.0,
            top: pixel_coord.y(),
            bottom: pixel_coord.y() + 16.0,
        };

        println!("char: {:?}", character_rect);

        for shape in level.collision_shapes.iter() {
            if intersect(shape, &character_rect) {
                println!("intersect {:?} {:?}", character_rect, shape);
            }
        }
    }
}
