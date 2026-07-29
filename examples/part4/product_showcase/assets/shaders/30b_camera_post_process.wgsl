struct PostProcessSettings {
    intensity: f32,
    vignette: f32,
    time: f32,
    padding: f32,
}

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var screen_sampler: sampler;

@group(0) @binding(2)
var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<f32>(textureDimensions(screen_texture));
    let uv = position.xy / size;
    let source = textureSample(screen_texture, screen_sampler, uv);

    let centered = uv * 2.0 - 1.0;
    let edge = smoothstep(0.25, 1.25, dot(centered, centered));
    let vignette = 1.0 - edge * settings.vignette * settings.intensity;

    let pulse = 0.96 + 0.04 * sin(settings.time * 1.5);
    let graded = vec3<f32>(
        source.r * (1.0 + 0.18 * settings.intensity),
        source.g * (1.0 + 0.04 * settings.intensity),
        source.b * (1.0 - 0.12 * settings.intensity),
    );

    return vec4<f32>(
        mix(source.rgb, graded * vignette * pulse, settings.intensity),
        source.a,
    );
}
