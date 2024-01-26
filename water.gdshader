shader_type canvas_item;

uniform sampler2D screen_texture : hint_screen_texture, repeat_disable, filter_nearest;

uniform sampler2D noise1 : repeat_enable;
uniform sampler2D noise2 : repeat_enable;
uniform vec2 scroll1 = vec2(0.05, 0.05);
uniform vec2 scroll2 = vec2(-0.05, -0.05);

uniform float distortion_strength : hint_range(-1.0, 1.0) = 0.2;

uniform float light_start : hint_range(0.0, 1.0) = 0.275;
uniform float light_end : hint_range(0.0, 1.0) = 0.4;

uniform vec4 base_texture_colour : source_color;
uniform vec4 top_color : source_color;

void fragment() {
	float depth = texture(noise1, UV + scroll1 * TIME).r * texture(noise2, UV + scroll2 * TIME).r;
	vec4 sample_col = textureLod(TEXTURE, UV + distortion_strength * vec2(depth), 0.0) * base_texture_colour;
	vec4 top_light = smoothstep(light_start, light_end, depth) * top_color;
	COLOR = sample_col + top_light;
	COLOR.a = 1.0;
}