shader_type spatial;
render_mode cull_disabled;

uniform vec3 color : source_color;
uniform vec3 color2 : source_color;
uniform sampler2D noise;
uniform float noise_scale = 20.0;

varying vec3 world_pos;

void vertex() {
	world_pos = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
}

void fragment() {
	vec3 noise_level = texture(noise, world_pos.xz / noise_scale).rgb;
	ALBEDO = mix(color, color2, UV.y) * mix(color, color2, noise_level.r);

	if (!FRONT_FACING) {
		NORMAL = -NORMAL;
	}
}