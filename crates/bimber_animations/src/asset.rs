use bevy::asset::{AssetLoader, LoadedAsset};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AnimMeta {
    pub start_idx: usize,
    pub len: usize,
    pub frame_time: f32,
    #[serde(default)]
    pub mode: AnimMode,
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
pub enum AnimMode {
    #[default]
    Repeating,
    Once,
}

#[derive(Debug, Deserialize)]
struct AnimAtlasMeta {
    map: HashMap<String, AnimMeta>,
    tile_size: usize,
    rows: usize,
    columns: usize,
}

#[derive(Debug, TypeUuid)]
#[uuid = "c33c1eaa-e107-11ed-b5ea-0242ac120002"]
pub struct Animation {
    pub map: HashMap<String, AnimMeta>,
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Debug, Default)]
pub struct AnimationLoader;

impl AssetLoader for AnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let anim_meta: AnimAtlasMeta = ron::de::from_bytes(bytes)?;
            info!("Loading meta animation asset {:?}", load_context.path());

            let image_path = load_context.path().with_extension("").with_extension("png");
            let image_bytes = load_context.read_asset_bytes(image_path).await?;

            let image = Image::from_buffer(
                &image_bytes,
                ImageType::Extension("png"),
                CompressedImageFormats::all(),
                true,
            )?;

            let image_handle = load_context.set_labeled_asset("image", LoadedAsset::new(image));

            let texture_atlas = TextureAtlas::from_grid(
                image_handle,
                vec2(anim_meta.tile_size as f32, anim_meta.tile_size as f32),
                anim_meta.columns,
                anim_meta.rows,
                None,
                None,
            );

            let atlas_handle =
                load_context.set_labeled_asset("atlas", LoadedAsset::new(texture_atlas));

            let anim = Animation {
                map: anim_meta.map,
                atlas: atlas_handle,
            };

            load_context.set_default_asset(LoadedAsset::new(anim));

            info!("Asset loaded");

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }
}

impl Default for AnimMeta {
    fn default() -> Self {
        Self {
            start_idx: 0,
            len: 1,
            frame_time: 0.1,
            mode: default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::log::LogPlugin;

    use super::*;

    #[derive(Debug, Resource, Default)]
    struct TestHandle {
        handle: Handle<Animation>,
        printed: bool,
    }

    fn load_test_startup(mut state: ResMut<TestHandle>, asset_server: Res<AssetServer>) {
        state.handle = asset_server.load::<Animation, _>("Test/test.anim");
    }

    fn load_test_print(mut state: ResMut<TestHandle>, meta_assets: Res<Assets<Animation>>) {
        let meta = meta_assets.get(&state.handle);
        info!("{meta:?}");
        match meta {
            Some(meta) if !state.printed => {
                info!("{meta:?}");
                state.printed = true;
            }
            _ => (),
        }
    }

    #[test]
    fn load_test() {
        let mut app = App::new();

        app.add_plugin(AssetPlugin::default())
            .add_plugin(TaskPoolPlugin::default())
            .add_plugin(LogPlugin::default())
            .init_resource::<TestHandle>()
            .init_asset_loader::<AnimationLoader>()
            .add_asset::<Animation>()
            .add_asset::<TextureAtlas>()
            .add_asset::<Image>()
            .add_startup_system(load_test_startup)
            .add_system(load_test_print);

        while !app.world.get_resource::<TestHandle>().unwrap().printed {
            app.update();
        }
    }
}
