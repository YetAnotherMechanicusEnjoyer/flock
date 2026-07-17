#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var screen_texture: texture_2d<f32>;
@group(2) @binding(1) var screen_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    var crt_uv = uv * 2.0 - 1.0;

    let curvature_factor = 6.0;
    let offset = crt_uv.yx / curvature_factor;
    crt_uv = crt_uv + crt_uv * offset * offset;

    var final_uv = crt_uv * 0.5 + 0.5;

    if (final_uv.x < 0.0 || final_uv.x > 1.0 ||
        final_uv.y < 0.0 || final_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let time = globals.time;

    let roll_speed = 0.15;
    let roll_y = fract(time * roll_speed);
    let roll_dist = abs(final_uv.y - roll_y);

    var sample_uv = final_uv;

    if (roll_dist < 0.01) {
        sample_uv.x += 0.0015 * sin(time * 50.0);
    }

    let dist_to_center = length(final_uv - vec2<f32>(0.5, 0.5));

    let base_aberration = 0.0055 * pow(dist_to_center, 2.0);

    let uv_r = clamp(
        sample_uv + vec2<f32>(base_aberration, 0.0),
        vec2<f32>(0.0),
        vec2<f32>(1.0),
    );

    let uv_g = clamp(
        sample_uv,
        vec2<f32>(0.0),
        vec2<f32>(1.0),
    );

    let uv_b = clamp(
        sample_uv - vec2<f32>(base_aberration, 0.0),
        vec2<f32>(0.0),
        vec2<f32>(1.0),
    );

    let r = textureSample(screen_texture, screen_sampler, uv_r).r;
    let g = textureSample(screen_texture, screen_sampler, uv_g).g;
    let b = textureSample(screen_texture, screen_sampler, uv_b).b;

    var color = vec3<f32>(r, g, b);

    let luminance = dot(color, vec3<f32>(0.299, 0.587, 0.114));

    let text_factor = smoothstep(0.45, 0.8, luminance);

    if (roll_dist < 0.01) {
        color *= 1.15;
    }

    color *= 1.0 - 0.01 * sin(time * 100.0);

    let scanline_intensity = 0.12;
    let scanline = sin(final_uv.y * 900.0);
    let scanline_factor =
        1.0 - (scanline * 0.5 + 0.5) * scanline_intensity;

    color *= mix(scanline_factor, 1.0, text_factor);

    let border_x = max(0.0, abs(crt_uv.x) - 0.95);
    let border_y = max(0.0, abs(crt_uv.y) - 0.95);

    let border_dist = length(vec2<f32>(border_x, border_y));

    let rounded_corners =
        1.0 - smoothstep(0.0, 0.04, border_dist);

    color *= rounded_corners;

    let vignette =
        1.0 - smoothstep(0.5, 1.4, length(crt_uv));

    color *= vignette;

    color *= 1.2;

    return vec4<f32>(color, 1.0);
}
