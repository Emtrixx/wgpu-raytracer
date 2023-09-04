const infinity: f32 = 10000000.0;

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
@group (2) @binding(0) var<storage> spheres: array<Sphere>;
struct Sphere {
    center: vec3<f32>,
    radius: f32,
};


// Materials
@group (3) @binding(0) var<uniform> materialMetadata: MaterialMetadata;
@group (3) @binding(1) var<storage, read> materials: array<Material>;
struct MaterialMetadata {
    count: u32,
}
struct Material {
    color: vec3<f32>,
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


    let view_params: vec3<f32> = camera.view_params;

    let viewPointLocal: vec3<f32> = vec3<f32>(uv - 0.5, 1.0) * view_params;

    let viewPointWorld: vec3<f32> = vec3<f32>((vec4<f32>(viewPointLocal, 1.0) * camera.rotation).xyz) + camera.eye;


    // Ray
    let origin: vec3<f32> = camera.eye;
    let ray: Ray = Ray (
        origin,
        normalize(viewPointWorld - origin),
    );

    var color: vec3<f32> = vec3<f32>(0.2, 0.2, 0.24);
    var closestHitInfo: HitInfo = HitInfo (
        false,
        infinity,
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
    );

    for (var i = 0u; i < 4u; i++) {
        let sphere = spheres[i];

        var hitInfo = sphereIntersect(ray, sphere.center, sphere.radius);
        if (hitInfo.hit && hitInfo.distance < closestHitInfo.distance) {
            closestHitInfo = hitInfo;
            color = materials[1].color;
        }
    }

    textureStore(color_buffer, globals.globalInvocationId.xy, vec4<f32>(color, 1.0));
}


/*
 * Functions
 */
struct HitInfo {
    hit: bool,
    distance: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
};

fn sphereIntersect(ray: Ray, sphere_center: vec3<f32>, sphere_radius: f32) -> HitInfo {
    var hitInfo: HitInfo = HitInfo (
        false,
        infinity,
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
    );

    let offsetRayOrigin = ray.origin - sphere_center;

    let a: f32 = dot(ray.direction, ray.direction);
    let b: f32 = 2.0 * dot(offsetRayOrigin, ray.direction);
    let c: f32 = dot(offsetRayOrigin, offsetRayOrigin) - sphere_radius * sphere_radius;

    let discriminant: f32 = b * b - 4.0 * a * c;


    if (discriminant >= 0.0) {
        let distance = (-b - sqrt(discriminant)) / (2.0 * a);

        if (distance >= 0.0) {
            hitInfo.hit = true;
            hitInfo.distance = distance;
            hitInfo.position = ray.origin + ray.direction * hitInfo.distance;
            hitInfo.normal = normalize(hitInfo.position - sphere_center);
        }
    }

    return hitInfo;
}

// Utils

fn rand(co: vec2 <f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
