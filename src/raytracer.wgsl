@group(0) @binding(0) var color_buffer: texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(0) var<uniform> camera_rotation: mat4x4<f32>;
@group(1) @binding(1) var<uniform> camera_eye: vec3<f32>;

const infinity: f32 = 10000000.0;

struct Globals {
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
    @builtin(global_invocation_id) globalInvocationId: vec3<u32>,
};

struct Camera {
   rotation: mat4x4<f32>,
   eye: vec3<f32>,
}

//struct CameraRotation {
//    rotation: mat4x4<f32>,
//};
//@group(1) @binding(0)
//var<uniform> cameraRotation: CameraRotation;
//
//struct CameraEye {
//    eye: vec3<f32>,
//};
//@group(1) @binding(1)
//var<uniform> cameraEye: CameraRotation;


struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    material: Material,
};

struct Material {
    color: vec3<f32>,
};

// Sphere parameters
const spheres: array<Sphere, 3> = array<Sphere, 3>(
    Sphere (
        vec3<f32>(0.0, 2.0, 0.0),
        1.0,
        Material (
            vec3<f32>(0.0, 1.0, 0.0),
        ),
    ),
    Sphere (
        vec3<f32>(2.0, 0.0, 0.0),
        1.0,
        Material (
            vec3<f32>(1.0, 0.0, 0.0),
        ),
    ),
    Sphere (
        vec3<f32>(0.0, 0.0, 2.0),
        1.0,
        Material (
            vec3<f32>(0.0, 0.0, 1.0),
        ),
    ),
);

@compute @workgroup_size(1,1,1)
fn main(globals: Globals) {

    let dimensions: vec2<u32> = textureDimensions(color_buffer);
//    let screen_pos: vec2<i32> = vec2<i32>(i32(globals.globalInvocationId.x), i32(globals.globalInvocationId.y));
//    let uv: vec2<f32> = vec2<f32>(f32(screen_pos.x) / f32(dimensions.x), f32(screen_pos.y) / f32(dimensions.y));
    let uv: vec2<f32> = vec2<f32>(f32(globals.globalInvocationId.x) / f32(dimensions.x), 1.0 - f32(globals.globalInvocationId.y) / f32(dimensions.y));

    // Calculate ray direction for current pixel

    // Camera parameters
    let camera: Camera = Camera (
        camera_rotation,
        camera_eye,
    );
//    let eye: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
//    let cameraTarget: vec3<f32> = vec3<f32>(0.0, 0.0, -1.0);
//    let forward: vec3<f32> = normalize(cameraTarget - eye);

    // TODO put view_params into uniform buffer
    let near: f32 = 0.1;
    let fov: f32 = 70.0;
    let aspect: f32 = f32(dimensions.x) / f32(dimensions.y);

    let plane_height: f32 = 2.0 * tan(fov * 0.5 * 3.14159 / 180.0) * near;
    let plane_width: f32 = plane_height * aspect;
//    With UV
//
    let view_params: vec3<f32> = vec3<f32>(plane_width, plane_height, near);

    let viewPointLocal: vec3<f32> = vec3<f32>(uv - 0.5, 1.0) * view_params;

    let viewPointWorld: vec3<f32> = vec3<f32>((vec4<f32>(viewPointLocal, 1.0) * camera.rotation).xyz) + camera.eye;
//    let viewPointWorld: vec3<f32> = vec3<f32>((vec4<f32>(viewPointLocal, 1.0)).xyz) + camera.eye;

    // Other way
//    let pixel_width: f32 = plane_width / f32(dimensions.x);
//    let pixel_height: f32 = plane_height / f32(dimensions.y);
//    let pixel_pos: vec2<f32> = vec2<f32>(f32(screen_pos.x) * pixel_width - (plane_width / 2.0), f32(screen_pos.y) * pixel_height - (plane_height / 2.0));
//    let pixel_pos_world: vec3<f32> = vec3<f32>((camera.rotation * vec4<f32>(pixel_pos, near, 1.0)).xyz) + camera.eye;
//    let pixel_pos_world: vec3<f32> = vec3<f32>((vec4<f32>(pixel_pos, near, 1.0)).xyz) ;




    // Ray
//    let origin: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let origin: vec3<f32> = camera.eye;
    let ray: Ray = Ray (
        origin,
        normalize(viewPointWorld - origin),
    );
//    let ray: Ray = Ray (
//        origin,
//        normalize(pixel_pos_world - origin),
//    );

//    textureStore(color_buffer, globals.globalInvocationId.xy, vec4<f32>(ray.direction, 1.0));

    // Check for intersection with sphere
    var color: vec3<f32> = ray.direction;
    var closestHitInfo: HitInfo = HitInfo (
        false,
        infinity,
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
    );

//   for (var i : u32 = 0u; i < 3u; i = i + 1u) {
//        let sphere: Sphere = spheres[i];
//        let hitInfo = sphereIntersect(ray, sphere.center, sphere.radius);
//        if (hitInfo.hit && hitInfo.distance < closestHitInfo.distance) {
//            closestHitInfo = hitInfo;
//            color = spheres[i].material.color;
//        }
//    }
var sphere: Sphere = spheres[0];
var hitInfo = sphereIntersect(ray, sphere.center, sphere.radius);
if (hitInfo.hit && hitInfo.distance < closestHitInfo.distance) {
    closestHitInfo = hitInfo;
    color = sphere.material.color;
}
sphere = spheres[1];
hitInfo = sphereIntersect(ray, sphere.center, sphere.radius);
if (hitInfo.hit && hitInfo.distance < closestHitInfo.distance) {
    closestHitInfo = hitInfo;
    color = sphere.material.color;
}
sphere = spheres[2];
hitInfo = sphereIntersect(ray, sphere.center, sphere.radius);
if (hitInfo.hit && hitInfo.distance < closestHitInfo.distance) {
    closestHitInfo = hitInfo;
    color = sphere.material.color;
}

    textureStore(color_buffer, globals.globalInvocationId.xy, vec4<f32>(color, 1.0));


//    let ray_dir: vec3<f32> = normalize(vec3<f32>(pixel_pos.x, pixel_pos.y, -near));
//    let pixel_pos_world: vec3<f32> = eye + ray_dir;

    // Check for intersection with sphere
//    let sphere_to_pixel: vec3<f32> = pixel_pos_world - sphere_center;
//    let a: f32 = dot(ray_dir, ray_dir);
//    let b: f32 = 2.0 * dot(sphere_to_pixel, ray_dir);
//    let c: f32 = dot(sphere_to_pixel, sphere_to_pixel) - sphere_radius * sphere_radius;
//    let discriminant: f32 = b * b - 4.0 * a * c;
//    if (discriminant < 0.0) {
//        // No intersection, set pixel color to black
//        textureStore(color_buffer, screen_pos, vec4<f32>(0.0, 0.0, 0.0, 1.0));
//    } else {
//        // Intersection, set pixel color to green
//        textureStore(color_buffer, screen_pos, vec4<f32>(0.0, 1.0, 0.0, 1.0));
//    }
}


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