#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> fill_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> player_position: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let radius = 70.0;
    let amount = 1.0 - min(length(mesh.world_position.xy - player_position) / radius, 1.0);
    return mix(fill_color, vec4(1.0, 1.0, 1.0, 1.0), amount);
}
