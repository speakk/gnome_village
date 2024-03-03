shader_type canvas_item;
render_mode unshaded;

uniform sampler2D screen_texture : hint_screen_texture, repeat_disable, filter_nearest;

uniform float daylight_amount : hint_range(0.0, 1.0);
uniform float yellow_amount : hint_range(0.0, 1.0);

void fragment() {
		//$CanvasModulate.color = Color(red_green, red_green - yellow_amount * 0.1, 1.0 - yellow_amount * 0.4)

	vec4 sampled = textureLod(screen_texture, SCREEN_UV, 0.0);

	vec4 final_color = vec4(daylight_amount, daylight_amount - yellow_amount * 0.1,  1.0 - yellow_amount * 0.4, 1.0) * sampled + vec4(daylight_amount * 0.01, daylight_amount * 0.01, 0.0, 0.0);

	COLOR = final_color;
}