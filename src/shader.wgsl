struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) color: vec3<f32>
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(1) color: vec3<f32>
}

@vertex
fn vert_main(model: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.position = vec4<f32>(model.position, 1.0);
  out.color = model.color;
  return out;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}
