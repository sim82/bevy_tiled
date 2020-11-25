use anyhow::Result;
use bevy::{
    math::prelude::*,
    math::Rect,
    prelude::*,
    render::mesh::Indices,
    render::{
        mesh::VertexAttributeValues,
        pipeline::PrimitiveTopology,
        pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline},
        render_graph::base::MainPass,
    },
    utils::HashMap,
};
use bevy_type_registry::TypeUuid;

use crate::{loader::TiledMapLoader, map::Map, TileMapChunk, TILE_MAP_PIPELINE_HANDLE};
use glam::Vec2;
use std::{collections::HashSet, io::BufReader, path::Path};

enum CollisionShape {
    Rect(Rect<f32>),
}
pub struct Level {
    collision_shapes: Vec<CollisionShape>,
}

impl Level {
    pub fn new(map: &tiled::Map) -> Self {
        let mut collision_shapes = Vec::new();
        for layer in map.layers.iter() {
            if !layer.visible {
                continue;
            }
            for y in 0..map.height {
                let mut line = String::new();

                for x in 0..map.width {
                    let map_tile = match &layer.tiles {
                        tiled::LayerData::Finite(tiles) => &tiles[y as usize][x as usize],
                        _ => panic!("Infinte maps not supported"),
                    };
                    if map_tile.gid != 0 {
                        collision_shapes.push(CollisionShape::Rect(Rect {
                            left: (x * 16) as f32,
                            right: (x * 16 + 16) as f32,
                            top: (y * 16) as f32,
                            bottom: (y * 16 + 16) as f32,
                        }));
                        line.push('#')
                    } else {
                        line.push(' ')
                    }
                    // println!("map tile: {:?}", map_tile);
                }
                println!("{}", line)
            }

            // match &layer.tiles {
            //     tiled::LayerData::Finite(tiles) => {
            //         println!("size: {}", tiles.len());

            //         for span in tiles {
            //             println!("size x: {}", span.len());
            //         }
            //     }
            //     _ => panic!("Infinte maps not supported"),
            // }
        }

        Level { collision_shapes }
    }
}

#[derive(Default)]
pub struct MapResourceProviderState2 {
    map_event_reader: EventReader<AssetEvent<Map>>,
}
pub fn process_loaded_tile_maps2(
    // asset_server: Res<AssetServer>,
    mut state: Local<MapResourceProviderState2>,
    map_events: Res<Events<AssetEvent<Map>>>,
    mut maps: ResMut<Assets<Map>>,
    mut level: ResMut<Option<Level>>,
) {
    for event in state.map_event_reader.iter(&map_events) {
        match event {
            AssetEvent::Created { handle } => {
                let map = maps.get_mut(handle).unwrap();

                *level = Some(Level::new(&map.map));
                // match &layer.tiles {
                //     tiled::LayerData::Finite(tiles) => {
                //         println!("size: {}", tiles.len());

                //         for span in tiles {
                //             println!("size x: {}", span.len());
                //         }
                //     }
                //     _ => panic!("Infinte maps not supported"),
                // }

                println!("created: {:?}", handle);
            }
            AssetEvent::Modified { handle } => {
                // println!("modified: {:?}", handle);
            }
            AssetEvent::Removed { handle } => {
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                // println!("removed: {:?}", handle);
            }
        }
    }
}
