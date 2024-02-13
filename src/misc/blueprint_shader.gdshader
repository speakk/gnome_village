shader_type spatial;

void fragment() {
	ALBEDO = vec3(ALBEDO.r * 0.3, ALBEDO.g * 0.3, 1.0);
	//ALBEDO = vec3(0.2, 0.2, 1.0);
	ALPHA = 0.4;
}
