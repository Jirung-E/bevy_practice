use bevy::{
    asset::AssetPlugin,
    core_pipeline::{Core3dSystems, FullscreenShader, schedule::Core3d},
    prelude::*,
    render::{
        RenderApp, RenderStartup,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        view::ViewTarget,
    },
    window::WindowResolution,
};

const SHADER_PATH: &str = "shaders/30b_camera_post_process.wgsl";

struct CameraPostProcessPlugin;

impl Plugin for CameraPostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<PostProcessSettings>::default(),
            UniformComponentPlugin::<PostProcessSettings>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(RenderStartup, init_post_process_pipeline)
            .add_systems(
                Core3d,
                post_process_system.in_set(Core3dSystems::PostProcess),
            );
    }
}

#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessSettings {
    intensity: f32,
    vignette: f32,
    time: f32,
    _padding: f32,
}

#[derive(Resource)]
struct EffectState {
    camera: Entity,
    enabled: bool,
    intensity: f32,
}

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct Rotates;

#[derive(Default)]
struct PostProcessBindGroupCache {
    cached: Option<(TextureViewId, BindGroup)>,
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Camera Post Process - Bevy Practice".into(),
                        resolution: WindowResolution::new(1100, 720),
                        ..default()
                    }),
                    ..default()
                }),
            CameraPostProcessPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.012, 0.018, 0.032)))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_objects, handle_input, update_settings))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let settings = PostProcessSettings {
        intensity: 0.55,
        vignette: 0.7,
        time: 0.0,
        _padding: 0.0,
    };
    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 2.3, 7.8).looking_at(Vec3::ZERO, Vec3::Y),
            settings,
            AmbientLight {
                color: Color::srgb(0.18, 0.24, 0.4),
                brightness: 220.0,
                ..default()
            },
        ))
        .id();
    commands.insert_resource(EffectState {
        camera,
        enabled: true,
        intensity: settings.intensity,
    });

    let mesh = meshes.add(Cuboid::new(1.5, 1.5, 1.5));
    for (x, color) in [
        (-2.1, Color::srgb(0.08, 0.75, 1.0)),
        (0.0, Color::srgb(1.0, 0.22, 0.08)),
        (2.1, Color::srgb(0.72, 0.12, 1.0)),
    ] {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.25,
                perceptual_roughness: 0.32,
                ..default()
            })),
            Transform::from_xyz(x, 0.0, 0.0),
            Rotates,
        ));
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 13_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.75, -0.55, 0.0)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 8.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.04, 0.05, 0.08),
            perceptual_roughness: 0.85,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.15, 0.0),
    ));

    commands.spawn((
        Text::new("CAMERA POST PROCESS\nP: TOGGLE   UP/DOWN: INTENSITY"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(0.4, 0.88, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(18),
            ..default()
        },
    ));
    commands.spawn((
        StatusText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.82, 0.24)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(16),
            left: px(18),
            ..default()
        },
    ));
}

fn rotate_objects(time: Res<Time>, mut objects: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut objects {
        transform.rotate_y(0.65 * time.delta_secs());
        transform.rotate_x(0.22 * time.delta_secs());
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<EffectState>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        state.intensity = (state.intensity + 0.1).clamp(0.0, 1.0);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        state.intensity = (state.intensity - 0.1).clamp(0.0, 1.0);
    }
    if keys.just_pressed(KeyCode::KeyP) {
        state.enabled = !state.enabled;
        if state.enabled {
            commands.entity(state.camera).insert(PostProcessSettings {
                intensity: state.intensity,
                vignette: 0.7,
                time: 0.0,
                _padding: 0.0,
            });
        } else {
            commands
                .entity(state.camera)
                .remove::<PostProcessSettings>();
        }
    }
}

fn update_settings(
    time: Res<Time>,
    state: Res<EffectState>,
    mut settings: Query<&mut PostProcessSettings>,
    mut status: Single<&mut Text, With<StatusText>>,
) {
    for mut settings in &mut settings {
        settings.intensity = state.intensity;
        settings.time = time.elapsed_secs();
    }
    status.0 = format!(
        "POST PROCESS: {}   |   INTENSITY: {:.1}",
        if state.enabled { "ON" } else { "OFF" },
        state.intensity
    );
}

fn init_post_process_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "camera_post_process_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<PostProcessSettings>(true),
            ),
        ),
    );
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("camera_post_process_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader: asset_server.load(SHADER_PATH),
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Rgba8UnormSrgb,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(PostProcessPipeline {
        layout,
        sampler,
        pipeline_id,
    });
}

fn post_process_system(
    view: ViewQuery<(
        &ViewTarget,
        &PostProcessSettings,
        &DynamicUniformIndex<PostProcessSettings>,
    )>,
    pipeline: Option<Res<PostProcessPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    uniforms: Res<ComponentUniforms<PostProcessSettings>>,
    mut cache: Local<PostProcessBindGroupCache>,
    mut ctx: RenderContext,
) {
    let Some(pipeline) = pipeline else {
        return;
    };
    let (view_target, _settings, settings_index) = view.into_inner();
    let Some(render_pipeline) = pipeline_cache.get_render_pipeline(pipeline.pipeline_id) else {
        return;
    };
    let Some(settings_binding) = uniforms.uniforms().binding() else {
        return;
    };

    let post_process = view_target.post_process_write();
    let bind_group = match &mut cache.cached {
        Some((texture_id, bind_group)) if post_process.source.id() == *texture_id => bind_group,
        cached => {
            let bind_group = ctx.render_device().create_bind_group(
                "camera_post_process_bind_group",
                &pipeline_cache.get_bind_group_layout(&pipeline.layout),
                &BindGroupEntries::sequential((
                    post_process.source,
                    &pipeline.sampler,
                    settings_binding.clone(),
                )),
            );
            &cached.insert((post_process.source.id(), bind_group)).1
        }
    };

    let mut pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("camera_post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    pass.set_pipeline(render_pipeline);
    pass.set_bind_group(0, bind_group, &[settings_index.index()]);
    pass.draw(0..3, 0..1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn intensity_is_clamped_to_effect_range() {
        assert_eq!((0.95_f32 + 0.1).clamp(0.0, 1.0), 1.0);
        assert_eq!((0.05_f32 - 0.1).clamp(0.0, 1.0), 0.0);
    }
}
