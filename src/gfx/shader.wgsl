struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertex(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.position = vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
