const INFINITY: f32 = 10000000.0;
const PI: f32 = 3.14159;
const BACKGROUND_COLOR: vec3<f32> = vec3<f32>(0.2, 0.2, 0.2);
const MAX_BOUNCE_COUNT: i32 = 5;
var<private> state: f32;


/*
 * Bindings
 */

struct Globals {
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
    @builtin(global_invocation_id) globalInvocationId: vec3<u32>,
};

// Color buffer
@group(0) @binding(0) var color_buffer: texture_storage_2d<rgba8unorm, write>;

// Camera
@group(1) @binding(0)
var<uniform> camera: Camera;
struct Camera {
   rotation: mat4x4<f32>,
   eye: vec3<f32>,
   _padding: u32,
   view_params: vec3<f32>,
   _padding2: u32,
}


// Spheres
@group (2) @binding(0) var<uniform> sphereMetadata: SphereMetadata;
@group (2) @binding(1) var<storage> spheres: array<Sphere>;
struct SphereMetadata {
    count: u32,
}
struct Sphere {
    position: vec3<f32>,
    radius: f32,
    material_id: u32,
};


// Materials
@group (3) @binding(0) var<uniform> materialMetadata: MaterialMetadata;
@group (3) @binding(1) var<storage, read> materials: array<Material>;
struct MaterialMetadata {
    count: u32,
}
struct Material {
    color: vec3<f32>,
    emission_color: vec3<f32>,
    emission_strength: f32,
};
//struct MaterialStorage {
//    count: u32,
//    materials: array<Material>,
//};


/*
 * Internal types
 */
struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

//const materials: array<Material,3> = array<Material, 3>(
//    Material (
//        vec3<f32>(1.0, 0.0, 0.0),
//    ),
//    Material (
//        vec3<f32>(0.0, 1.0, 0.0),
//    ),
//    Material (
//        vec3<f32>(0.0, 0.0, 1.0),
//    ));


/*
* Main
*/
@compute @workgroup_size(1,1,1)
fn main(globals: Globals) {

    let dimensions: vec2<u32> = textureDimensions(color_buffer);
    let uv: vec2<f32> = vec2<f32>(f32(globals.globalInvocationId.x) / f32(dimensions.x), 1.0 - f32(globals.globalInvocationId.y) / f32(dimensions.y));
    state = fract(sin(dot(uv.xy ,vec2(12.9898,78.233))) * 43758.5453);

    let view_params: vec3<f32> = camera.view_params;

    let viewPointLocal: vec3<f32> = vec3<f32>(uv - 0.5, 1.0) * view_params;

    let viewPointWorld: vec3<f32> = vec3<f32>((vec4<f32>(viewPointLocal, 1.0) * camera.rotation).xyz) + camera.eye;

    let pixel_width = 1.0 / f32(dimensions.x);
    let pixel_height = 1.0 / f32(dimensions.y);

    // Ray
    let origin: vec3<f32> = camera.eye;
//    var ray: Ray =

    let ray_count: u32 = 10u;
    var incoming_light: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var i = 0u; i < ray_count; i++) {

        let offset: vec3<f32> = vec3<f32>(rand(&state) * pixel_width, rand(&state) * pixel_height, 0.0);
//        let offset: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

        incoming_light += trace_path(
            Ray (
                 origin + offset,
                 normalize(viewPointWorld - origin),
            )
        );
    }
//    incoming_light /= f32(ray_count);
    incoming_light /= rand_direction(&state);

    textureStore(color_buffer, globals.globalInvocationId.xy, vec4<f32>(incoming_light, 1.0));
}


/*
 * Functions
 */
fn trace_path(ray_param: Ray) -> vec3<f32> {
    var ray: Ray = ray_param;
    var ray_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    var incoming_light: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0; i < MAX_BOUNCE_COUNT; i++) {

        var closestHitInfo: HitInfo = HitInfo (
            false,
            INFINITY,
            vec3<f32>(0.0, 0.0, 0.0),
            vec3<f32>(0.0, 0.0, 0.0),
            0u,
        );

        for (var i = 0u; i < sphereMetadata.count; i++) {
            let sphere = spheres[i];

            var hitInfo = sphereIntersect(ray, sphere);
            if (hitInfo.hit && hitInfo.distance < closestHitInfo.distance) {
                closestHitInfo = hitInfo;
            }
        }


        if (closestHitInfo.hit) {

//            incoming_light = closestHitInfo.normal;
//            break;

            let dir = rand_hemisphere_direction(&state, closestHitInfo.normal);

            let bounce_ray: Ray = Ray (
                closestHitInfo.position,
                dir,
            );
            ray = bounce_ray;

            let material = materials[closestHitInfo.material_id];
            let emission_color: vec3<f32> = material.emission_color * material.emission_strength;
            incoming_light += emission_color * ray_color;
            ray_color *= material.color;

        } else {
            incoming_light += get_environment_light(ray) * ray_color;
            break;
        }
    }
    return incoming_light;
}

struct HitInfo {
    hit: bool,
    distance: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
    material_id: u32,
};

fn sphereIntersect(ray: Ray, sphere: Sphere) -> HitInfo {

    var hitInfo: HitInfo = HitInfo (
        false,
        INFINITY,
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
        0u,
    );

    let offsetRayOrigin = ray.origin - sphere.position;

    let a: f32 = dot(ray.direction, ray.direction);
    let b: f32 = 2.0 * dot(offsetRayOrigin, ray.direction);
    let c: f32 = dot(offsetRayOrigin, offsetRayOrigin) - sphere.radius * sphere.radius;

    let discriminant: f32 = b * b - 4.0 * a * c;


    if (discriminant >= 0.0) {
        let distance = (-b - sqrt(discriminant)) / (2.0 * a);

        if (distance >= 0.0) {

            let position = ray.origin + (ray.direction * distance);
            let normal = normalize(position - sphere.position);

            hitInfo.hit = true;
            hitInfo.distance = distance;
            hitInfo.position = position + normal * 0.00001;
            hitInfo.normal = normal;
            hitInfo.material_id = sphere.material_id;
        }
    }

    return hitInfo;
}

/*
// Utils
*/

// Light
fn get_environment_light(ray: Ray) -> vec3<f32> {
    // Ground
    let ground_color = vec3<f32>(0.24, 0.2, 0.18);
    // Sky
    let sky_gradient_t = pow(smoothstep(0.0, 0.4, ray.direction.y), 0.35);
    let sky_color_zenith = vec3<f32>(0.4, 0.6, 1.0);
    let sky_color_horizon = vec3<f32>(0.8, 0.8, 0.8);
    let sky_color = mix(sky_color_horizon, sky_color_zenith, sky_gradient_t);
    // Mix
    let ground_to_sky_t = smoothstep(-0.01, 0.0, ray.direction.y);
    return mix(ground_color, sky_color, ground_to_sky_t);
//    return vec3<f32>(0.2, 0.2, 0.2);
}

// Random
fn rand(seed: ptr<private,f32>) -> f32 {
    *seed = fract(sin(*seed) * 43758.5453);
    return *seed;
}

fn rand_normal(seed: ptr<private,f32>) -> f32 {
    let theta = 2.0 * PI * rand(seed);
    let r = sqrt(-2.0 * log(rand(seed)));
    return r * cos(theta);
}

fn rand_direction(seed: ptr<private,f32>) -> vec3<f32> {
    let x = rand_normal(seed);
    let y = rand_normal(seed);
    let z = rand_normal(seed);
    return  normalize(vec3<f32>(x, y, z));
}

fn rand_hemisphere_direction(seed: ptr<private,f32>, normal: vec3<f32>) -> vec3<f32> {
    var direction = rand_direction(seed);

    if (dot(direction, normal) < 0.0) {
        return -direction;
    };

    return direction;
}
