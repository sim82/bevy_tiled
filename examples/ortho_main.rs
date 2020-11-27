use bevy::math;
use bevy::{prelude::*, render::camera::Camera};
use bevy_tiled_prototype::level;
use bevy_tiled_prototype::TiledMapCenter;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(character_movement.system())
        .add_system(character_intersect.system())
        .add_system(camera_movement.system())
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
            //origin: Transform::from_scale(Vec3::new(8.0, 8.0, 1.0)),
            origin: Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                translation: Vec3::new(0.0, 16.0 * 16.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(Camera2dComponents {
            transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.0)),
            ..Default::default()
        });

    // let texture_handle = asset_server.load("gabe-idle-run.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_handle = asset_server.load("ferris.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::splat(8.0 / 8.0),
                translation: Vec3::new(0.0 * 8.0, 4.0 * 16.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(CharacterState::default());
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

#[derive(Default)]
pub struct CharacterState {
    velocity: Vec3,
}

fn character_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&TextureAtlasSprite, &mut CharacterState)>,
) {
    for (_, mut state) in query.iter_mut() {
        let mut direction = Vec3::zero();
        // let scale = transform.scale.x();

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

        // if keyboard_input.pressed(KeyCode::Z) {
        //     let scale = scale + 0.1;
        //     transform.scale = Vec3::new(scale, scale, scale);
        // }

        // if keyboard_input.pressed(KeyCode::X) && scale > 1.1 {
        //     let scale = scale - 0.1;
        //     transform.scale = Vec3::new(scale, scale, scale);
        // }

        // transform.translation += time.delta_seconds * direction * 1000.;
        state.velocity = direction;
    }
}

fn intersect(shape: &level::CollisionShape, rect: &math::Rect<f32>) -> bool {
    match shape {
        level::CollisionShape::Rect(shape) => {
            rect.left <= shape.right
                && rect.right >= shape.left
                && rect.top >= shape.bottom
                && rect.bottom <= shape.top
        }
    }
}

pub fn character_intersect(
    time: Res<Time>,
    level: Res<Option<level::Level>>,
    mut query: Query<(&TextureAtlasSprite, &mut Transform, &CharacterState)>,
) {
    if level.is_none() {
        return;
    }

    let level = level.as_ref().unwrap();

    for (_, mut transform, state) in query.iter_mut() {
        let new_translation = transform.translation + state.velocity * 128. * time.delta_seconds;
        //println!("transform: {:?}", transform);
        let mut pixel_coord = new_translation; // / 8f32;
                                               // *pixel_coord.y_mut() *= -1f32;

        let character_rect = math::Rect {
            left: pixel_coord.x() + 2.0,
            right: pixel_coord.x() + 14.0,
            top: pixel_coord.y(),
            bottom: pixel_coord.y() - 12.0,
        };

        // println!("char: {:?}", character_rect);
        let mut intersects = false;
        for shape in level.collision_shapes.iter() {
            if intersect(shape, &character_rect) {
                // println!("intersect {:?} {:?}", character_rect, shape);
                intersects = true;
                break;
            }
        }

        if !intersects {
            transform.translation = new_translation;
        }
    }
}
