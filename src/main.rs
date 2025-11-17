use three_d::*;
use std::f32::consts::PI;
use rand::Rng;

// Función para crear el campo de estrellas - usando cuadrados orientados
fn create_star_field(context: &Context, star_count: usize) -> Gm<Mesh, ColorMaterial> {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();
    
    let mut rng = rand::thread_rng();
    
    // Crear estrellas distribuidas en una esfera grande
    for i in 0..star_count {
        // Distribución en una esfera usando coordenadas esféricas
        let theta = rng.gen_range(0.0..2.0 * PI);
        let phi = rng.gen_range(0.0..PI);
        let radius = rng.gen_range(90.0..110.0); // Radio grande para el fondo
        
        let center_x = radius * phi.sin() * theta.cos();
        let center_y = radius * phi.cos();
        let center_z = radius * phi.sin() * theta.sin();
        
        // Variar el color ligeramente para algunas estrellas
        let brightness = rng.gen_range(0.7..1.0);
        let star_color = Srgba::new(
            (255.0 * brightness) as u8,
            (255.0 * brightness) as u8,
            (255.0 * brightness) as u8,
            255,
        );
        
        // Crear un pequeño cuadrado orientado hacia la cámara (billboard)
        let size = 0.5; // Tamaño más grande para mejor visibilidad
        let base_index = (i * 4) as u32;
        
        // Posiciones para un cuadrado centrado en la posición de la estrella
        positions.push(vec3(center_x - size, center_y - size, center_z));
        positions.push(vec3(center_x + size, center_y - size, center_z));
        positions.push(vec3(center_x + size, center_y + size, center_z));
        positions.push(vec3(center_x - size, center_y + size, center_z));
        
        // Índices para los dos triángulos que forman el cuadrado
        indices.push(base_index);
        indices.push(base_index + 1);
        indices.push(base_index + 2);
        indices.push(base_index);
        indices.push(base_index + 2);
        indices.push(base_index + 3);
        
        // Colores para cada vértice (todos del mismo color para la estrella)
        for _ in 0..4 {
            colors.push(star_color);
        }
    }
    
    let mesh = Mesh::new(
        context,
        &CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            colors: Some(colors),
            ..Default::default()
        },
    );
    
    let material = ColorMaterial {
        color: Srgba::WHITE,
        is_transparent: false,
        render_states: RenderStates::default(),
        texture: None,
    };
    
    Gm::new(mesh, material)
}

// Función para cargar un modelo OBJ (versión corregida para la API actual)
fn load_obj_model(context: &Context, path: &str, scale: f32) -> Result<Gm<Mesh, ColorMaterial>, Box<dyn std::error::Error>> {
    // Cargar el archivo OBJ
    let obj = obj::Obj::load(path)?;
    
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    
    // Procesar las posiciones del modelo OBJ
    for position in &obj.data.position {
        positions.push(vec3(
            position[0] * scale,
            position[1] * scale,
            position[2] * scale,
        ));
    }
    
    // Procesar normales si están disponibles
    for normal in &obj.data.normal {
        normals.push(vec3(normal[0], normal[1], normal[2]));
    }
    
    // Procesar los índices (caras) del modelo OBJ
    for object in &obj.data.objects {
        for group in &object.groups {
            for polygon in &group.polys {
                // Convertir polígonos a triángulos (triangulación simple)
                if polygon.0.len() >= 3 {
                    let first_index = polygon.0[0].0 as u32;
                    for i in 1..(polygon.0.len() - 1) {
                        indices.push(first_index);
                        indices.push(polygon.0[i].0 as u32);
                        indices.push(polygon.0[i + 1].0 as u32);
                    }
                }
            }
        }
    }
    
    // Si no hay normales, calcularlas básicas
    if normals.is_empty() {
        normals = vec![vec3(0.0, 1.0, 0.0); positions.len()];
    }
    
    // Crear la malla
    let mesh = Mesh::new(context, &CpuMesh {
        positions: Positions::F32(positions),
        normals: Some(normals),
        indices: Indices::U32(indices),
        ..Default::default()
    });
    
    let material = ColorMaterial {
        color: Srgba::new(180, 180, 200, 255),
        is_transparent: false,
        render_states: RenderStates::default(),
        texture: None,
    };
    
    Ok(Gm::new(mesh, material))
}

// Función alternativa para crear una nave simple si no se puede cargar el OBJ
fn create_simple_ship(context: &Context) -> Gm<Mesh, ColorMaterial> {
    let ship_mesh = Mesh::new(
        context,
        &CpuMesh {
            positions: Positions::F32(vec![
                // Cuerpo principal más aerodinámico
                vec3(-0.4, -0.15, -1.2), vec3(0.4, -0.15, -1.2), vec3(0.4, 0.15, -1.2), vec3(-0.4, 0.15, -1.2),
                vec3(-0.3, -0.1, 1.0), vec3(0.3, -0.1, 1.0), vec3(0.3, 0.1, 1.0), vec3(-0.3, 0.1, 1.0),
                // Ala izquierda extendida
                vec3(-1.0, -0.1, -0.3), vec3(-0.4, -0.1, -0.3), vec3(-0.4, 0.0, -0.3), vec3(-1.0, 0.0, -0.3),
                vec3(-1.0, -0.1, 0.3), vec3(-0.4, -0.1, 0.3), vec3(-0.4, 0.0, 0.3), vec3(-1.0, 0.0, 0.3),
                // Ala derecha extendida
                vec3(1.0, -0.1, -0.3), vec3(0.4, -0.1, -0.3), vec3(0.4, 0.0, -0.3), vec3(1.0, 0.0, -0.3),
                vec3(1.0, -0.1, 0.3), vec3(0.4, -0.1, 0.3), vec3(0.4, 0.0, 0.3), vec3(1.0, 0.0, 0.3),
                // Cabina más prominente
                vec3(-0.25, 0.15, -0.8), vec3(0.25, 0.15, -0.8), vec3(0.25, 0.5, -0.8), vec3(-0.25, 0.5, -0.8),
                vec3(-0.25, 0.15, -0.3), vec3(0.25, 0.15, -0.3), vec3(0.25, 0.5, -0.3), vec3(-0.25, 0.5, -0.3),
                // Motores
                vec3(-0.2, -0.15, 1.0), vec3(-0.1, -0.15, 1.0), vec3(-0.1, -0.3, 1.0), vec3(-0.2, -0.3, 1.0),
                vec3(0.2, -0.15, 1.0), vec3(0.1, -0.15, 1.0), vec3(0.1, -0.3, 1.0), vec3(0.2, -0.3, 1.0),
            ]),
            indices: Indices::U32(vec![
                // Cuerpo principal
                0,1,2, 2,3,0,  4,5,6, 6,7,4,  0,4,7, 7,3,0,  1,5,6, 6,2,1,  0,1,5, 5,4,0,  3,2,6, 6,7,3,
                // Alas izquierda
                8,9,10, 10,11,8,  12,13,14, 14,15,12,  8,12,15, 15,11,8,  9,13,14, 14,10,9,  8,9,13, 13,12,8,  11,10,14, 14,15,11,
                // Alas derecha
                16,17,18, 18,19,16,  20,21,22, 22,23,20,  16,20,23, 23,19,16,  17,21,22, 22,18,17,  16,17,21, 21,20,16,  19,18,22, 22,23,19,
                // Cabina
                24,25,26, 26,27,24,  28,29,30, 30,31,28,  24,28,31, 31,27,24,  25,29,30, 30,26,25,  24,25,29, 29,28,24,  27,26,30, 30,31,27,
                // Motores
                32,33,34, 34,35,32,  36,37,38, 38,39,36,
            ]),
            normals: Some(vec![
                vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0),
                vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0),
                vec3(-1.0,0.0,0.0), vec3(-1.0,0.0,0.0), vec3(-1.0,0.0,0.0), vec3(-1.0,0.0,0.0),
                vec3(1.0,0.0,0.0), vec3(1.0,0.0,0.0), vec3(1.0,0.0,0.0), vec3(1.0,0.0,0.0),
                vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0),
                vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0),
                vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0),
                vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0),
                vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0), vec3(0.0,0.0,-1.0),
                vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0), vec3(0.0,0.0,1.0),
            ]),
            ..Default::default()
        },
    );

    let ship_material = ColorMaterial {
        color: Srgba::new(180, 180, 200, 255),
        is_transparent: false,
        render_states: RenderStates::default(),
        texture: None,
    };

    Gm::new(ship_mesh, ship_material)
}

// Esfera simple
fn create_sphere(radius: f32, segments: usize) -> CpuMesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=segments {
        let theta = i as f32 * PI / segments as f32;
        for j in 0..=segments {
            let phi = j as f32 * 2.0 * PI / segments as f32;
            
            let x = radius * theta.sin() * phi.cos();
            let y = radius * theta.cos();
            let z = radius * theta.sin() * phi.sin();
            
            positions.push(vec3(x, y, z));
            normals.push(vec3(x, y, z).normalize());
        }
    }

    for i in 0..segments {
        for j in 0..segments {
            let first = i * (segments + 1) + j;
            let second = first + segments + 1;
            
            indices.push(first as u32);
            indices.push(second as u32);
            indices.push((first + 1) as u32);
            
            indices.push(second as u32);
            indices.push((second + 1) as u32);
            indices.push((first + 1) as u32);
        }
    }

    CpuMesh {
        positions: Positions::F32(positions),
        normals: Some(normals),
        indices: Indices::U32(indices),
        ..Default::default()
    }
}

// Crear un anillo para las órbitas
fn create_ring(radius: f32, thickness: f32, segments: usize) -> CpuMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        
        // Crear un pequeño cuadrilátero en cada segmento
        positions.push(vec3(x - thickness, 0.0, z - thickness));
        positions.push(vec3(x + thickness, 0.0, z - thickness));
        positions.push(vec3(x + thickness, 0.0, z + thickness));
        positions.push(vec3(x - thickness, 0.0, z + thickness));
    }

    // Crear índices para los cuadriláteros
    for i in 0..segments {
        let base = (i * 4) as u32;
        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
    }

    CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        ..Default::default()
    }
}

// Crear anillos planetarios (como los de Saturno)
fn create_planet_rings(context: &Context, inner_radius: f32, outer_radius: f32, segments: usize) -> Gm<Mesh, ColorMaterial> {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        
        // Punto interior
        let x_inner = inner_radius * angle.cos();
        let z_inner = inner_radius * angle.sin();
        positions.push(vec3(x_inner, 0.0, z_inner));
        
        // Punto exterior
        let x_outer = outer_radius * angle.cos();
        let z_outer = outer_radius * angle.sin();
        positions.push(vec3(x_outer, 0.0, z_outer));
    }

    // Crear índices para formar triángulos
    for i in 0..segments {
        let base = (i * 2) as u32;
        indices.extend_from_slice(&[
            base, base + 1, base + 3,
            base, base + 3, base + 2,
        ]);
    }

    let ring_mesh = Mesh::new(context, &CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        ..Default::default()
    });

    let ring_material = ColorMaterial {
        color: Srgba::new(210, 180, 140, 180),
        is_transparent: true,
        render_states: RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        },
        texture: None,
    };

    Gm::new(ring_mesh, ring_material)
}

// Crear efecto de corona solar con múltiples capas
fn create_sun_corona(context: &Context, base_radius: f32) -> Vec<Gm<Mesh, ColorMaterial>> {
    let mut corona_layers = Vec::new();
    
    let layers = [
        (base_radius * 1.1, Srgba::new(255, 255, 100, 80)),
        (base_radius * 1.2, Srgba::new(255, 200, 50, 60)),
        (base_radius * 1.3, Srgba::new(255, 150, 30, 40)),
        (base_radius * 1.4, Srgba::new(255, 100, 20, 20)),
    ];
    
    for (radius, color) in layers {
        let mesh = Mesh::new(context, &create_sphere(radius, 24));
        let material = ColorMaterial {
            color,
            is_transparent: true,
            render_states: RenderStates {
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
            texture: None,
        };
        corona_layers.push(Gm::new(mesh, material));
    }
    
    corona_layers
}

// Crear planeta con color base
fn create_textured_sphere(context: &Context, radius: f32, base_color: Srgba) -> Gm<Mesh, ColorMaterial> {
    let mesh = Mesh::new(context, &create_sphere(radius, 32));
    
    let material = ColorMaterial {
        color: base_color,
        is_transparent: false,
        render_states: RenderStates::default(),
        texture: None,
    };
    
    Gm::new(mesh, material)
}

// Crear efecto de atmósfera para planetas terrestres
fn create_atmosphere(context: &Context, planet_radius: f32) -> Gm<Mesh, ColorMaterial> {
    let atmosphere_radius = planet_radius * 1.1;
    let mesh = Mesh::new(context, &create_sphere(atmosphere_radius, 32));
    
    let material = ColorMaterial {
        color: Srgba::new(100, 150, 255, 30),
        is_transparent: true,
        render_states: RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        },
        texture: None,
    };
    
    Gm::new(mesh, material)
}

// Crear atmósfera densa para Venus
fn create_dense_atmosphere(context: &Context, planet_radius: f32) -> Gm<Mesh, ColorMaterial> {
    let atmosphere_radius = planet_radius * 1.15;
    let mesh = Mesh::new(context, &create_sphere(atmosphere_radius, 32));
    
    let material = ColorMaterial {
        color: Srgba::new(255, 220, 150, 50),
        is_transparent: true,
        render_states: RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        },
        texture: None,
    };
    
    Gm::new(mesh, material)
}

// Crear bandas simples para planetas gaseosos
fn create_gas_giant_bands(context: &Context, planet_radius: f32) -> Gm<Mesh, ColorMaterial> {
    // Crear anillos concéntricos simples para simular bandas
    let ring = create_ring(planet_radius * 1.05, 0.08, 32);
    let mesh = Mesh::new(context, &ring);
    
    let material = ColorMaterial {
        color: Srgba::new(255, 255, 255, 80),
        is_transparent: true,
        render_states: RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        },
        texture: None,
    };
    
    Gm::new(mesh, material)
}

// Crear anillos de latitud simples
fn create_latitude_bands(context: &Context, planet_radius: f32) -> Gm<Mesh, ColorMaterial> {
    // Crear un anillo simple en el ecuador
    let ring = create_ring(planet_radius * 1.02, 0.02, 32);
    let mesh = Mesh::new(context, &ring);
    
    let material = ColorMaterial {
        color: Srgba::new(200, 180, 160, 100),
        is_transparent: true,
        render_states: RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        },
        texture: None,
    };
    
    Gm::new(mesh, material)
}

// Estructura para manejar lunas
struct Moon {
    mesh: Gm<Mesh, ColorMaterial>,
    orbit_radius: f32,
    orbit_speed: f32,
    current_angle: f32,
}

impl Moon {
    fn new(context: &Context, radius: f32, orbit_radius: f32, orbit_speed: f32, color: Srgba) -> Self {
        let mesh = Mesh::new(context, &create_sphere(radius, 16));
        let material = ColorMaterial {
            color,
            is_transparent: false,
            render_states: RenderStates::default(),
            texture: None,
        };

        Self {
            mesh: Gm::new(mesh, material),
            orbit_radius,
            orbit_speed,
            current_angle: 0.0,
        }
    }

    fn update(&mut self, planet_position: Vec3, _time: f32) {
        self.current_angle += self.orbit_speed * 0.02;
        
        let x = self.orbit_radius * self.current_angle.cos();
        let z = self.orbit_radius * self.current_angle.sin();
        
        self.mesh.set_transformation(
            Mat4::from_translation(planet_position + vec3(x, 0.0, z))
        );
    }
}

// Estructura para manejar planetas con efectos únicos
struct UniquePlanet {
    mesh: Gm<Mesh, ColorMaterial>,
    base_color: Srgba,
    orbit_radius: f32,
    orbit_speed: f32,
    rotation_speed: f32,
    current_angle: f32,
    collision_radius: f32,
    name: String,
    planet_type: PlanetType,
    rings: Option<Gm<Mesh, ColorMaterial>>,
    moons: Vec<Moon>,
    special_features: Vec<Gm<Mesh, ColorMaterial>>,
    atmosphere: Option<Gm<Mesh, ColorMaterial>>,
    detail_level: u8,
}

#[derive(Clone)]
enum PlanetType {
    Rocky,
    Gaseous,
    IceGiant,
    Terrestrial,
}

impl UniquePlanet {
    fn new(context: &Context, radius: f32, orbit_radius: f32, orbit_speed: f32, 
           rotation_speed: f32, color: Srgba, name: &str, planet_type: PlanetType) -> Self {
        
        let mesh = create_textured_sphere(context, radius, color);
        
        let mut planet = Self {
            mesh: mesh,
            base_color: color,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            current_angle: 0.0,
            collision_radius: radius * 1.2,
            name: name.to_string(),
            planet_type: planet_type.clone(),
            rings: None,
            moons: Vec::new(),
            special_features: Vec::new(),
            atmosphere: None,
            detail_level: 1,
        };

        // Agregar características especiales basadas en el tipo de planeta
        match planet_type {
            PlanetType::Gaseous => {
                planet.detail_level = 2;
                
                // Saturno - agregar anillos
                if name == "Saturno" {
                    planet.rings = Some(create_planet_rings(context, radius * 1.3, radius * 2.2, 64));
                    planet.special_features.push(create_latitude_bands(context, radius));
                }
                // Júpiter - agregar banda de nubes
                else if name == "Júpiter" {
                    planet.special_features.push(create_gas_giant_bands(context, radius));
                    planet.special_features.push(create_latitude_bands(context, radius));
                }
                // Urano y Neptuno - colores más distintivos
                else if name == "Urano" || name == "Neptuno" {
                    planet.mesh.material.color = if name == "Urano" {
                        Srgba::new(150, 200, 230, 255)
                    } else {
                        Srgba::new(80, 120, 255, 255)
                    };
                    planet.special_features.push(create_latitude_bands(context, radius));
                }
            },
            PlanetType::Terrestrial => {
                planet.detail_level = 2;
                
                // Tierra - agregar atmósfera y luna
                if name == "Tierra" {
                    planet.atmosphere = Some(create_atmosphere(context, radius));
                    planet.moons.push(Moon::new(
                        context, 
                        radius * 0.25, 
                        radius * 3.0, 
                        1.5, 
                        Srgba::new(180, 180, 180, 255)
                    ));
                }
                // Marte - color más rojizo y lunas
                else if name == "Marte" {
                    planet.mesh.material.color = Srgba::new(200, 100, 80, 255);
                    planet.moons.push(Moon::new(
                        context, 
                        radius * 0.1, 
                        radius * 2.5, 
                        2.0, 
                        Srgba::new(140, 120, 100, 255)
                    ));
                    planet.moons.push(Moon::new(
                        context, 
                        radius * 0.08, 
                        radius * 3.5, 
                        1.2, 
                        Srgba::new(120, 110, 90, 255)
                    ));
                }
                // Venus - atmósfera densa y color amarillento
                else if name == "Venus" {
                    planet.atmosphere = Some(create_dense_atmosphere(context, radius));
                    planet.mesh.material.color = Srgba::new(240, 200, 120, 255);
                }
            },
            PlanetType::Rocky => {
                // Mercurio - superficie craterizada
                if name == "Mercurio" {
                    planet.mesh.material.color = Srgba::new(160, 140, 120, 255);
                }
            },
            PlanetType::IceGiant => {
                planet.detail_level = 2;
                
                // Urano - anillos tenues
                if name == "Urano" {
                    let uranus_rings = create_planet_rings(context, radius * 1.2, radius * 1.8, 48);
                    let mut rings_gm = uranus_rings;
                    rings_gm.material.color = Srgba::new(180, 220, 240, 100);
                    planet.rings = Some(rings_gm);
                    planet.special_features.push(create_latitude_bands(context, radius));
                }
            },
        }

        planet
    }

    fn update_shadow(&mut self, sun_position: Vec3, camera_position: Vec3, time: f32) {
        let planet_position = vec3(
            self.orbit_radius * self.current_angle.cos(),
            0.0,
            self.orbit_radius * self.current_angle.sin()
        );

        let to_sun = (sun_position - planet_position).normalize();
        let to_camera = (camera_position - planet_position).normalize();
        let dot_product = to_sun.dot(to_camera);
        let mut light_intensity = (dot_product * 0.5 + 0.5).max(0.3).min(1.0);

        match self.planet_type {
            PlanetType::Gaseous => {
                light_intensity = light_intensity * 0.8 + 0.2;
                if self.name == "Júpiter" {
                    let pulse = (time * 0.5).sin() * 0.1 + 1.0;
                    light_intensity *= pulse;
                }
            },
            PlanetType::IceGiant => {
                light_intensity = light_intensity.max(0.5);
            },
            _ => {}
        }
        
        let new_color = Srgba::new(
            (self.base_color.r as f32 * light_intensity) as u8,
            (self.base_color.g as f32 * light_intensity) as u8,
            (self.base_color.b as f32 * light_intensity) as u8,
            self.base_color.a,
        );
        
        self.mesh.material.color = new_color;
    }

    fn set_transformation(&mut self, transformation: Mat4) {
        self.mesh.set_transformation(transformation);
        
        if let Some(atmosphere) = &mut self.atmosphere {
            atmosphere.set_transformation(transformation);
        }
        
        if let Some(rings) = &mut self.rings {
            rings.set_transformation(transformation);
        }
        
        for feature in &mut self.special_features {
            feature.set_transformation(transformation * Mat4::from_angle_y(Rad(self.rotation_speed * 2.0)));
        }
    }

    fn transformation(&self) -> Mat4 {
        self.mesh.transformation()
    }

    fn update_moons(&mut self, planet_position: Vec3, time: f32) {
        for moon in &mut self.moons {
            moon.update(planet_position, time);
        }
    }

    fn mesh(&self) -> &Gm<Mesh, ColorMaterial> {
        &self.mesh
    }

    fn rings(&self) -> Option<&Gm<Mesh, ColorMaterial>> {
        self.rings.as_ref()
    }

    fn moons(&self) -> Vec<&Gm<Mesh, ColorMaterial>> {
        self.moons.iter().map(|m| &m.mesh).collect()
    }

    fn special_features(&self) -> &Vec<Gm<Mesh, ColorMaterial>> {
        &self.special_features
    }

    fn atmosphere(&self) -> Option<&Gm<Mesh, ColorMaterial>> {
        self.atmosphere.as_ref()
    }
}

struct Sun {
    core: Gm<Mesh, ColorMaterial>,
    corona: Vec<Gm<Mesh, ColorMaterial>>,
    rotation_speed: f32,
    collision_radius: f32,
}

impl Sun {
    fn new(context: &Context) -> Self {
        let core_mesh = Mesh::new(context, &create_sphere(1.0, 32));
        let core_material = ColorMaterial {
            color: Srgba::new(255, 255, 100, 255),
            is_transparent: false,
            render_states: RenderStates::default(),
            texture: None,
        };
        
        let corona = create_sun_corona(context, 1.0);
        
        Self {
            core: Gm::new(core_mesh, core_material),
            corona,
            rotation_speed: 0.03,
            collision_radius: 1.5,
        }
    }
    
    fn set_transformation(&mut self, transformation: Mat4) {
        self.core.set_transformation(transformation);
        for layer in &mut self.corona {
            layer.set_transformation(transformation);
        }
    }
    
    fn update(&mut self, time: f32) {
        let pulse = (time * 3.0).sin() * 0.05 + 1.0;
        let pulse_transform = Mat4::from_scale(pulse);
        let rotation = Mat4::from_angle_y(Rad(self.rotation_speed * time));
        self.set_transformation(rotation * pulse_transform);
    }
    
    fn position(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }
}

struct Spaceship {
    meshes: Vec<Gm<Mesh, ColorMaterial>>,
    position: Vec3,
    following_camera: bool,
    rotation: f32,
}

impl Spaceship {
    fn new(context: &Context) -> Self {
        // Intentar cargar el modelo OBJ, si falla usar la nave simple
        let ship_mesh = match load_obj_model(context, "nave.obj", 0.5) {
            Ok(mesh) => {
                println!("Nave cargada desde OBJ exitosamente");
                mesh
            },
            Err(e) => {
                println!("Error cargando nave.obj: {}. Usando nave simple.", e);
                create_simple_ship(context)
            }
        };

        Self {
            meshes: vec![ship_mesh],
            position: vec3(0.0, 0.5, 2.0),
            following_camera: true,
            rotation: 0.0,
        }
    }

    fn update(&mut self, camera_position: Vec3, time: f32) {
        if self.following_camera {
            self.position = camera_position + vec3(0.0, -0.5, 1.5);
            
            // Efecto de flotación suave para la nave
            let float_offset = (time * 3.0).sin() * 0.05;
            self.position.y += float_offset;
            
            // Rotación lenta de la nave
            self.rotation += 0.01;
        }
        
        for mesh in &mut self.meshes {
            mesh.set_transformation(
                Mat4::from_translation(self.position) *
                Mat4::from_angle_y(Rad(self.rotation)) *
                Mat4::from_angle_x(Rad(0.2)) *
                Mat4::from_scale(0.3) // Escala ajustada para la nave
            );
        }
    }
}

struct SolarSystem {
    sun: Sun,
    planets: Vec<UniquePlanet>,
    spaceship: Spaceship,
    camera_controller: CameraController,
    orbit_rings: Vec<Gm<Mesh, ColorMaterial>>,
    star_field: Gm<Mesh, ColorMaterial>,
    time: f32,
}

impl SolarSystem {
    fn new(context: &Context) -> Self {
        // Crear sol
        let sun = Sun::new(context);

        // Crear planetas con características únicas
        let mut planets = Vec::new();
        let planet_data = [
            (0.3, 3.0, 0.8, 0.5, Srgba::new(160, 140, 120, 255), "Mercurio", PlanetType::Rocky),
            (0.4, 5.0, 0.6, 0.4, Srgba::new(240, 200, 120, 255), "Venus", PlanetType::Rocky),
            (0.5, 7.0, 0.4, 0.3, Srgba::new(70, 130, 200, 255), "Tierra", PlanetType::Terrestrial),
            (0.4, 9.0, 0.3, 0.6, Srgba::new(200, 100, 80, 255), "Marte", PlanetType::Terrestrial),
            (0.8, 12.0, 0.2, 0.2, Srgba::new(220, 180, 140, 255), "Júpiter", PlanetType::Gaseous),
            (0.7, 15.0, 0.15, 0.3, Srgba::new(210, 190, 160, 255), "Saturno", PlanetType::Gaseous),
            (0.5, 18.0, 0.1, 0.4, Srgba::new(150, 200, 230, 255), "Urano", PlanetType::IceGiant),
            (0.5, 21.0, 0.08, 0.5, Srgba::new(80, 120, 255, 255), "Neptuno", PlanetType::IceGiant),
        ];

        for (radius, orbit_radius, orbit_speed, rotation_speed, color, name, planet_type) in planet_data {
            planets.push(UniquePlanet::new(
                context, radius, orbit_radius, orbit_speed, rotation_speed, color, name, planet_type
            ));
        }

        // Crear nave espacial
        let spaceship = Spaceship::new(context);

        // Crear anillos de órbita
        let mut orbit_rings = Vec::new();
        for planet in &planets {
            let ring_mesh = Mesh::new(context, &create_ring(planet.orbit_radius, 0.015, 128));
            let ring_material = ColorMaterial {
                color: Srgba::new(150, 150, 200, 60),
                is_transparent: true,
                render_states: RenderStates {
                    blend: Blend::TRANSPARENCY,
                    ..Default::default()
                },
                texture: None,
            };
            orbit_rings.push(Gm::new(ring_mesh, ring_material));
        }

        // Crear campo de estrellas CORREGIDO
        let star_field = create_star_field(context, 2000);

        let camera_controller = CameraController::new();

        SolarSystem {
            sun,
            planets,
            spaceship,
            camera_controller,
            orbit_rings,
            star_field,
            time: 0.0,
        }
    }

    fn update(&mut self, time: f32) {
        self.time = time;
        
        // Actualizar sol (rotación y pulso)
        self.sun.update(time);

        // Actualizar planetas
        for planet in &mut self.planets {
            planet.current_angle += planet.orbit_speed * 0.01;
            
            let x = planet.orbit_radius * planet.current_angle.cos();
            let z = planet.orbit_radius * planet.current_angle.sin();
            let planet_position = vec3(x, 0.0, z);
            
            planet.set_transformation(
                Mat4::from_translation(planet_position) *
                Mat4::from_angle_y(Rad(planet.rotation_speed * time))
            );
            
            // Actualizar sombras basadas en la posición del sol y la cámara
            planet.update_shadow(
                self.sun.position(),
                self.camera_controller.camera.position(),
                time
            );
            
            // Actualizar lunas
            planet.update_moons(planet_position, time);
        }

        // Actualizar nave
        self.spaceship.update(self.camera_controller.camera.position(), time);
    }

    fn handle_events(&mut self, events: &[Event]) {
        let celestial_bodies = self.get_celestial_bodies();
        self.camera_controller.handle_events(events, &celestial_bodies);
    }

    fn get_celestial_bodies(&self) -> Vec<(String, Vec3, f32)> {
        let mut bodies = vec![
            ("Sol".to_string(), vec3(0.0, 0.0, 0.0), self.sun.collision_radius)
        ];
        
        for planet in &self.planets {
            let transform = planet.transformation();
            let pos = transform.w.truncate();
            bodies.push((planet.name.clone(), pos, planet.collision_radius));
            
            for moon in &planet.moons {
                let moon_transform = moon.mesh.transformation();
                let moon_pos = moon_transform.w.truncate();
                bodies.push((format!("Luna de {}", planet.name), moon_pos, 0.1));
            }
        }
        
        bodies
    }

    fn all_renderables(&self) -> Vec<&Gm<Mesh, ColorMaterial>> {
        let mut renderables = Vec::new();
        
        for planet in &self.planets {
            renderables.push(planet.mesh());
            
            if let Some(atmosphere) = planet.atmosphere() {
                renderables.push(atmosphere);
            }
            
            if let Some(rings) = planet.rings() {
                renderables.push(rings);
            }
            
            for feature in planet.special_features() {
                renderables.push(feature);
            }
            
            renderables.extend(planet.moons());
        }
        
        renderables
    }
}

struct CameraController {
    camera: Camera,
    target_position: Vec3,
    warping: bool,
    warp_progress: f32,
    keys: Keys,
}

impl CameraController {
    fn new() -> Self {
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 3.0, 8.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(60.0),
            0.1,
            1000.0,
        );

        Self {
            camera,
            target_position: vec3(0.0, 3.0, 8.0),
            warping: false,
            warp_progress: 0.0,
            keys: Keys::default(),
        }
    }

    fn handle_events(&mut self, events: &[Event], celestial_bodies: &[(String, Vec3, f32)]) {
        self.keys.update(events);

        for event in events {
            if let Event::KeyPress { kind, .. } = event {
                match kind {
                    Key::Num1 => self.start_warp_planet(0, celestial_bodies),
                    Key::Num2 => self.start_warp_planet(1, celestial_bodies),
                    Key::Num3 => self.start_warp_planet(2, celestial_bodies),
                    Key::Num4 => self.start_warp_planet(3, celestial_bodies),
                    Key::Num5 => self.start_warp_planet(4, celestial_bodies),
                    Key::Num6 => self.start_warp_planet(5, celestial_bodies),
                    Key::Num7 => self.start_warp_planet(6, celestial_bodies),
                    Key::Num8 => self.start_warp_planet(7, celestial_bodies),
                    Key::T => self.toggle_view(),
                    Key::R => self.reset_view(),
                    _ => {}
                }
            }
        }

        if self.warping {
            self.warp_progress += 0.03;
            let t = smooth_step(self.warp_progress);
            let new_pos = self.camera.position().lerp(self.target_position, t);
            
            self.camera.set_view(
                new_pos,
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            );

            if self.warp_progress >= 1.0 {
                self.warping = false;
            }
        } else {
            self.handle_free_movement(celestial_bodies);
        }
    }

    fn start_warp_planet(&mut self, planet_index: usize, celestial_bodies: &[(String, Vec3, f32)]) {
        if planet_index < celestial_bodies.len() {
            let (_, pos, radius) = &celestial_bodies[planet_index];
            self.target_position = *pos + vec3(0.0, 2.0, *radius + 4.0);
            self.warping = true;
            self.warp_progress = 0.0;
        }
    }

    fn toggle_view(&mut self) {
        self.target_position = if self.camera.position().y > 10.0 {
            vec3(0.0, 3.0, 8.0)
        } else {
            vec3(0.0, 15.0, 0.0)
        };
        self.warping = true;
        self.warp_progress = 0.0;
    }

    fn reset_view(&mut self) {
        self.target_position = vec3(0.0, 3.0, 8.0);
        self.warping = true;
        self.warp_progress = 0.0;
    }

    fn handle_free_movement(&mut self, celestial_bodies: &[(String, Vec3, f32)]) {
        let speed = 0.2;
        let mut new_pos = self.camera.position();
        
        if self.keys.down(Key::W) { new_pos.z -= speed; }
        if self.keys.down(Key::S) { new_pos.z += speed; }
        if self.keys.down(Key::A) { new_pos.x -= speed; }
        if self.keys.down(Key::D) { new_pos.x += speed; }
        if self.keys.down(Key::Q) { new_pos.y -= speed; }
        if self.keys.down(Key::E) { new_pos.y += speed; }

        let safe_pos = self.prevent_collisions(new_pos, celestial_bodies);
        self.camera.set_view(safe_pos, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
    }

    fn prevent_collisions(&self, position: Vec3, celestial_bodies: &[(String, Vec3, f32)]) -> Vec3 {
        let mut safe_position = position;
        for (_, body_pos, radius) in celestial_bodies {
            let distance = position.distance(*body_pos);
            if distance < *radius * 1.5 {
                let direction = (position - *body_pos).normalize();
                safe_position = *body_pos + direction * *radius * 1.5;
            }
        }
        safe_position
    }

    fn update_viewport(&mut self, viewport: Viewport) {
        self.camera.set_viewport(viewport);
    }
}

#[derive(Default)]
struct Keys {
    pressed: std::collections::HashSet<Key>,
}

impl Keys {
    fn update(&mut self, events: &[Event]) {
        for event in events {
            match event {
                Event::KeyPress { kind, .. } => { self.pressed.insert(*kind); }
                Event::KeyRelease { kind, .. } => { self.pressed.remove(kind); }
                _ => {}
            }
        }
    }

    fn down(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
}

fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn main() {
    let window = Window::new(WindowSettings {
        title: "Sistema Solar con Campo de Estrellas".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    let context = window.gl();
    let mut solar_system = SolarSystem::new(&context);

    window.render_loop(move |frame_input| {
        solar_system.camera_controller.update_viewport(frame_input.viewport);
        solar_system.update(frame_input.accumulated_time as f32);
        solar_system.handle_events(&frame_input.events);

        let screen = frame_input.screen();
        
        // Fondo negro para el espacio
        screen.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0));

        // Renderizar en orden para efectos visuales correctos:
        
        // 1. Primero el campo de estrellas (fondo)
        screen.render(&solar_system.camera_controller.camera, &solar_system.star_field, &[]);
        
        // 2. Luego las órbitas
        screen.render(&solar_system.camera_controller.camera, &solar_system.orbit_rings, &[]);
        
        // 3. Luego la corona del sol (efecto de glow)
        for corona_layer in &solar_system.sun.corona {
            screen.render(&solar_system.camera_controller.camera, corona_layer, &[]);
        }
        
        // 4. El núcleo del sol
        screen.render(&solar_system.camera_controller.camera, &solar_system.sun.core, &[]);
        
        // 5. Todos los elementos de los planetas (planetas, anillos, lunas, características especiales)
        screen.render(
            &solar_system.camera_controller.camera, 
            &solar_system.all_renderables(), 
            &[]
        );
        
        // 6. Finalmente la nave (más cerca de la cámara)
        screen.render(&solar_system.camera_controller.camera, &solar_system.spaceship.meshes, &[]);

        FrameOutput::default()
    });
}