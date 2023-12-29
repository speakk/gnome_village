shader_type canvas_item;
render_mode unshaded;

uniform vec4 color : source_color;
uniform float angle : hint_range(0,360);
uniform float len : hint_range(0,1000) = 300;
uniform float fade_margin : hint_range(0,100) = 5;

void fragment() {

	float ang_rad = angle * 3.1416 / 360.0;
	vec2 dir = vec2(sin(ang_rad),cos(ang_rad));
	float max_dist = len;
	vec2 at = screen_uv_to_sdf(SCREEN_UV);
	float accum = 0.0;

	while(accum < max_dist) {
	    float d = texture_sdf(at);
	    accum+=d;
	    if (d < 0.01) {
	        break;
	    }
	    at += d * dir;
	}
	float alpha = 1.0-min(1.0,accum/max_dist);
	if (accum < fade_margin) {
		alpha *= max(0.0,accum / fade_margin);
	}

	COLOR = vec4(color.rgb,alpha * color.a);
}