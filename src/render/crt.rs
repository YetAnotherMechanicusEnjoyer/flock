use bevy::{
    camera::{ImageRenderTarget, RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{
        AsBindGroup, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    sprite_render::{Material2d, MeshMaterial2d},
    window::WindowResized,
};

use crate::render::components::CrtScreenQuad;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CrtMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for CrtMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/crt.wgsl".into()
    }
}

pub fn setup_crt_pipeline(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CrtMaterial>>,
    window: Single<&Window>,
) {
    let (width, height) = (window.width(), window.height());

    let size = Extent3d {
        width: width as u32,
        height: height as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("CRT Render Target"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);

    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        RenderTarget::Image(ImageRenderTarget {
            handle: image_handle.clone(),
            scale_factor: 1.,
        }),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::ONE))),
        MeshMaterial2d(materials.add(CrtMaterial {
            source_image: image_handle,
        })),
        Transform::from_scale(Vec3::new(width, height, 1.)),
        RenderLayers::layer(1),
        CrtScreenQuad,
    ));
}

pub fn resize_crt(
    mut resize_events: MessageReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
    materials: Res<Assets<CrtMaterial>>,
    mut quad_query: Query<(&mut Transform, &MeshMaterial2d<CrtMaterial>), With<CrtScreenQuad>>,
) {
    for event in resize_events.read() {
        let size = Extent3d {
            width: event.width as u32,
            height: event.height as u32,
            ..default()
        };

        for (mut transform, material_handle) in &mut quad_query {
            transform.scale = Vec3::new(event.width, event.height, 1.0);

            if let Some(material) = materials.get(&material_handle.0)
                && let Some(mut image) = images.get_mut(&material.source_image)
            {
                image.texture_descriptor.size = size;
                image.resize(size);
            }
        }
    }
}
