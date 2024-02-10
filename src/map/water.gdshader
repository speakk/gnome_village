shader_type spatial;

uniform vec3 albedo: source_color;
uniform vec3 albedo2: source_color;
uniform float metallic : hint_range(0.0, 1.0);
uniform float roughness : hint_range(0.0, 1.0);
uniform sampler2D texture_normal;
uniform sampler2D texture_normal2;
uniform vec2 wave_direction = vec2(2.0, 0.0);
uniform vec2 wave_direction2 = vec2(0.0, 1.0);
uniform float time_scale : hint_range(0.0, 0.2, 0.005) = 0.025;

uniform sampler2D wave;
uniform float noise_scale = 10.0;
uniform float height_scale = 0.15;

uniform float light_start : hint_range(0.0, 1.0) = 0.275;
uniform float light_end : hint_range(0.0, 1.0) = 0.4;
uniform vec3 top_color: source_color;


varying float height;
varying vec3 world_pos;

float fresnel(float amount, vec3 normal, vec3 view)
{
	return pow((1.0 - clamp(dot(normalize(normal), normalize(view)), 0.0, 1.0)), amount);
}

vec3 soft_light(vec3 base, vec3 blend){
	vec3 limit = step(0.5, blend);
	return mix(2.0 * base * blend + base * base * (1.0 - 2.0 * blend), sqrt(base) * (2.0 * blend - 1.0) + (2.0 * base) * (1.0 - blend), limit);
}

void vertex() {
	world_pos = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
	height = texture(wave, world_pos.xz / noise_scale + TIME * time_scale).r;
	VERTEX.y += height * height_scale;
}

void fragment() {
	vec2 time = (TIME * wave_direction) * time_scale;
	vec2 time2 = (TIME * wave_direction2) * time_scale;
	
	float fresnel = fresnel(5.0, NORMAL, VIEW);
	vec3 surface_color = mix(albedo, albedo2, fresnel);
	
	
	vec3 normal_blend = mix(texture(texture_normal, world_pos.xz + time).rgb, texture(texture_normal2, world_pos.xz + time2).rgb, 0.5);
	vec3 dist_normal = texture(texture_normal, world_pos.xz * 0.05 + time * 0.4).rgb;
	vec3 dist_normal2 = texture(texture_normal2, world_pos.xz * 0.1 - time * 0.3).rgb;
	
	vec3 top_light = smoothstep(light_start, light_end, dist_normal.r * dist_normal2.r) * top_color;
	//ALPHA = 1.0;
	//ALBEDO = soft_light(_screen, surface_color);
	ALBEDO = surface_color + top_light;
	METALLIC = metallic;
	ROUGHNESS = roughness;
	NORMAL_MAP = normal_blend;
}

//void light() {
	// Called for every pixel for every light affecting the material.
	// Uncomment to replace the default light processing function with this one.
//}
