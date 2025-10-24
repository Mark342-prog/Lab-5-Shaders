use image::{RgbImage, Rgb};
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

fn hash(mut x: u32) -> u32 {
    x = x.wrapping_mul(1664525).wrapping_add(1013904223);
    x ^= x >> 16;
    x
}
fn value_noise_1d(x: i32) -> f32 {
    let h = hash(x as u32);
    ((h & 0xFFFF) as f32) / 65536.0
}
fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
    let mut x = ((t - a) / (b - a)).clamp(0.0, 1.0);
    x = x * x * (3.0 - 2.0 * x);
    x
}
fn fractal_noise_1d(x: f32) -> f32 {
    let mut amp = 0.5;
    let mut freq = 1.0;
    let mut sum = 0.0;
    for _ in 0..5 {
        let xi = x * freq;
        let i0 = xi.floor() as i32;
        let i1 = i0 + 1;
        let t = xi - xi.floor();
        let v0 = value_noise_1d(i0);
        let v1 = value_noise_1d(i1);
        let v = v0 * (1.0 - t) + v1 * t;
        sum += v * amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum
}

fn dot(a:[f32;3], b:[f32;3]) -> f32 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }
fn length(v:[f32;3]) -> f32 { dot(v,v).sqrt() }
fn normalize(v:[f32;3]) -> [f32;3] {
    let l = length(v);
    if l==0.0 { [0.0,0.0,0.0] } else { [v[0]/l, v[1]/l, v[2]/l] }
}
fn subtract(a:[f32;3], b:[f32;3]) -> [f32;3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }
fn add(a:[f32;3], b:[f32;3]) -> [f32;3] { [a[0]+b[0], a[1]+b[1], a[2]+b[2]] }
fn mul(a:[f32;3], s:f32) -> [f32;3] { [a[0]*s, a[1]*s, a[2]*s] }

fn sphere_uv(normal: [f32;3]) -> (f32,f32) {
    let u = 0.5 + (normal[2].atan2(normal[0]) / (2.0*PI));
    let v = 0.5 - (normal[1].asin() / PI);
    (u, v)
}

fn reflect(v:[f32;3], n:[f32;3]) -> [f32;3] {
    let d = dot(v,n);
    [v[0] - 2.0*d*n[0], v[1] - 2.0*d*n[1], v[2] - 2.0*d*n[2]]
}

// --- Shaders ---

fn star_shader(surface_pos: [f32;3], normal: [f32;3], time: f32) -> [f32;3] {
    let r = length(surface_pos);
    let intensity = (1.0 / (0.5 + r*5.0)).clamp(0.0, 1.0);
    let pulse = 0.5 + 0.5 * (time * 3.0 + fractal_noise_1d(normal[0]*10.0 + normal[1]*7.0)).sin();
    let rim = (1.0 - normal[2].abs()).powf(6.0);
    let base = [1.0, 0.85, 0.3];
    let glow = [1.0, 0.5, 0.1];
    [
        (base[0]*intensity*0.7 + glow[0]*pulse*0.6*rim).min(1.0),
        (base[1]*intensity*0.7 + glow[1]*pulse*0.6*rim).min(1.0),
        (base[2]*intensity*0.5 + glow[2]*pulse*0.4*rim).min(1.0),
    ]
}

fn rocky_shader(normal: [f32;3], uv: (f32,f32), light_dir: [f32;3], time: f32) -> [f32;3] {
    let lat = (normal[1]).clamp(-1.0,1.0);
    let base_rock = [
        0.35 + 0.15*(lat+1.0)/2.0,
        0.28 + 0.12*(lat+1.0)/2.0,
        0.25 + 0.05*(lat+1.0)/2.0,
    ];
    let n = fractal_noise_1d(uv.0*8.0 + uv.1*12.0 + time*0.01);
    let continents_mask = smoothstep(0.35, 0.6, n);
    let land_color = [0.12, 0.5, 0.18];
    let sea_color = [0.02, 0.08, 0.18];
    let surface_color = [
        base_rock[0]*(1.0 - continents_mask) + land_color[0]*continents_mask,
        base_rock[1]*(1.0 - continents_mask) + land_color[1]*continents_mask,
        base_rock[2]*(1.0 - continents_mask) + land_color[2]*continents_mask,
    ];
    let view_dir = normalize([0.0,0.0,-1.0]);
    let reflect_dir = reflect(mul(light_dir, -1.0), normal);
    let spec = (dot(view_dir, reflect_dir)).max(0.0).powf(32.0);
    let cloud_noise = fractal_noise_1d((uv.0+0.1)*20.0 + (uv.1-0.05)*17.0 - time*0.02);
    let clouds = smoothstep(0.65, 0.82, cloud_noise) * 0.6;
    let cloud_color = [1.0,1.0,1.0];
    let final_color = [
        (surface_color[0]*0.9 + sea_color[0]*0.1) + spec*0.6 + clouds*cloud_color[0],
        (surface_color[1]*0.9 + sea_color[1]*0.1) + spec*0.6 + clouds*cloud_color[1],
        (surface_color[2]*0.95 + sea_color[2]*0.1) + spec*0.6 + clouds*cloud_color[2],
    ];
    [
        final_color[0].clamp(0.0,1.0),
        final_color[1].clamp(0.0,1.0),
        final_color[2].clamp(0.0,1.0),
    ]
}

fn gas_giant_shader(normal: [f32;3], uv: (f32,f32), light_dir: [f32;3], time: f32) -> [f32;3] {
    let lat = normal[1];
    let bands = fractal_noise_1d(lat * 12.0 + time*0.1) * 0.6 + (lat*6.0).sin()*0.4;
    let band_t = smoothstep(-0.7, 0.7, bands);
    let color_a = [0.9, 0.75, 0.55];
    let color_b = [0.25, 0.55, 0.7];
    let base = [
        color_a[0]*(1.0-band_t) + color_b[0]*band_t,
        color_a[1]*(1.0-band_t) + color_b[1]*band_t,
        color_a[2]*(1.0-band_t) + color_b[2]*band_t,
    ];
    let storms = smoothstep(0.85, 0.95, fractal_noise_1d(uv.0*40.0 + uv.1*40.0 - time*0.2));
    let storm_color = [0.05, 0.02, 0.02];
    let mixed = [
        base[0]*(1.0-storms) + storm_color[0]*storms,
        base[1]*(1.0-storms) + storm_color[1]*storms,
        base[2]*(1.0-storms) + storm_color[2]*storms,
    ];
    let ndotl = dot(normalize(light_dir), normal).max(0.0);
    [
        (mixed[0]*(0.2+0.8*ndotl)).clamp(0.0,1.0),
        (mixed[1]*(0.2+0.8*ndotl)).clamp(0.0,1.0),
        (mixed[2]*(0.2+0.8*ndotl)).clamp(0.0,1.0),
    ]
}

// --- Render ---

fn render(width: u32, height: u32, time: f32) -> RgbImage {
    let mut img = RgbImage::new(width, height);

    let fov = 60.0_f32.to_radians();
    let aspect = width as f32 / height as f32;
    let camera_pos = [0.0, 0.0, 6.0];
    let light_dir = normalize([-0.6, 0.4, -1.0]);

    // --- posiciones más amplias ---
    let star_center = [-4.0, 1.5, 0.0];
    let rocky_center = [0.8, -0.4, 0.0];
    let gas_center = [4.5, 0.5, -0.5];

    let star_radius = 0.8;
    let rocky_radius = 0.7;
    let gas_radius = 1.2;

    // --- luna más lejos ---
    let moon_radius = 0.14;
    let moon_orbit = 1.6;
    let moon_angle = time * 0.5;
    let moon_center = [
        rocky_center[0] + moon_orbit * moon_angle.cos() * 0.8,
        rocky_center[1] + moon_orbit * moon_angle.sin() * 0.4 + 0.15,
        rocky_center[2] + moon_orbit * moon_angle.sin() * 0.5,
    ];

    for y in 0..height {
        for x in 0..width {
            let nx = (x as f32 + 0.5) / width as f32;
            let ny = (y as f32 + 0.5) / height as f32;
            let px = (2.0*nx - 1.0) * (fov/2.0).tan() * aspect;
            let py = (1.0 - 2.0*ny) * (fov/2.0).tan();
            let dir = normalize([px, py, -1.0]);

            let mut color = [0.01, 0.01, 0.02];
            let mut closest_t = f32::INFINITY;
            let mut hit_color = None;

            // Star
            if let Some(t) = ray_sphere(camera_pos, dir, star_center, star_radius) {
                if t < closest_t {
                    closest_t = t;
                    let p = add(camera_pos, mul(dir, t));
                    let n = normalize(subtract(p, star_center));
                    hit_color = Some(star_shader(subtract(p, star_center), n, time));
                }
            }

            // Rocky planet
            if let Some(t) = ray_sphere(camera_pos, dir, rocky_center, rocky_radius) {
                if t < closest_t {
                    closest_t = t;
                    let p = add(camera_pos, mul(dir, t));
                    let n = normalize(subtract(p, rocky_center));
                    let uv = sphere_uv(n);
                    let c = rocky_shader(n, uv, light_dir, time);
                    let lam = dot(normalize(light_dir), n).max(0.0)*0.9+0.1;
                    hit_color = Some([c[0]*lam, c[1]*lam, c[2]*lam]);
                }
            }

            // Moon
            if let Some(t) = ray_sphere(camera_pos, dir, moon_center, moon_radius) {
                if t < closest_t {
                    closest_t = t;
                    let p = add(camera_pos, mul(dir, t));
                    let n = normalize(subtract(p, moon_center));
                    let (u,v) = sphere_uv(n);
                    let cr = fractal_noise_1d(u*40.0 + v*40.0 - time*0.05);
                    let base = 0.45 + 0.15*cr;
                    let c = [base, base*0.95, base*0.9];
                    let lam = dot(normalize(light_dir), n).max(0.0)*0.9+0.1;
                    hit_color = Some([c[0]*lam, c[1]*lam, c[2]*lam]);
                }
            }

            // Gas giant
            if let Some(t) = ray_sphere(camera_pos, dir, gas_center, gas_radius) {
                if t < closest_t {
                    closest_t = t;
                    let p = add(camera_pos, mul(dir, t));
                    let n = normalize(subtract(p, gas_center));
                    let uv = sphere_uv(n);
                    hit_color = Some(gas_giant_shader(n, uv, light_dir, time));
                }
            }

            if let Some(c) = hit_color {
                color = c;
            } else {
                let star_dir = normalize(subtract(star_center, camera_pos));
                let star_angle = dot(dir, star_dir).max(0.0);
                let glow = star_angle.powf(10.0)*2.5;
                color = [
                    color[0]+glow,
                    color[1]+glow*0.6,
                    color[2]+glow*0.2,
                ];
            }

            let gamma = 1.0/2.2;
            let col = [
                color[0].clamp(0.0,1.0).powf(gamma),
                color[1].clamp(0.0,1.0).powf(gamma),
                color[2].clamp(0.0,1.0).powf(gamma),
            ];
            img.put_pixel(x, y, Rgb([
                (col[0]*255.0) as u8,
                (col[1]*255.0) as u8,
                (col[2]*255.0) as u8,
            ]));
        }
    }
    img
}

fn ray_sphere(orig:[f32;3], dir:[f32;3], center:[f32;3], radius:f32) -> Option<f32> {
    let oc = subtract(orig, center);
    let a = dot(dir, dir);
    let b = 2.0*dot(oc, dir);
    let c = dot(oc, oc)-radius*radius;
    let disc = b*b-4.0*a*c;
    if disc < 0.0 { return None; }
    let sq = disc.sqrt();
    let t0 = (-b - sq)/(2.0*a);
    let t1 = (-b + sq)/(2.0*a);
    let t = if t0 > 0.001 { t0 } else if t1 > 0.001 { t1 } else { return None; };
    Some(t)
}

fn main() {
    let width = 1400;
    let height = 900;
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f32();
    let time = (start % 10000.0) as f32;
    println!("Rendering {}x{}, time={}", width, height, time);
    let img = render(width, height, time);
    img.save("output.png").expect("Failed to save image");
    println!("Saved output.png");
}
