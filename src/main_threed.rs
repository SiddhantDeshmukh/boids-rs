use boids::*;

use ::rand::{rngs::ThreadRng, Rng};
use three_d::*;


fn init(rng: &mut ThreadRng) -> (Params, Vec<Boid>) {
    // Initialise parameters and boid positions
    let params = Params::default();
    let bounds = Bounds::whole_domain();
    // Create boids
    let num_boids = 50;
    let boids_pop: Vec<Boid> = (0..num_boids).map(|_| random_boid(rng, &bounds)).collect::<Vec<Boid>>();
    return (params, boids_pop);
}


pub fn main() {
        let window = Window::new(WindowSettings {
        title: "Boids".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(X_SIZE + 5., Y_SIZE + 2.,Z_SIZE +  2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 10,
                },
                ..Default::default()
            },
        ),
    );
    // Transform the cube to be the world size
    cube.set_transformation(Mat4::from_translation(vec3(X_SIZE / 2., Y_SIZE / 2., Z_SIZE / 2.)) * Mat4::from_nonuniform_scale(X_SIZE / 2., Y_SIZE / 2., Z_SIZE / 2.));
    let axes = Axes::new(&context, 0.1, X_SIZE);
    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    // Spawn boids
    let mut rng: ThreadRng = ::rand::thread_rng();
    let mut params: Params;
    let mut boids_pop: Vec<Boid>;
    (params, boids_pop) = init(&mut rng);
    let mut boid_models = Vec::new();
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);
        // Update boids
        boids_pop = update_boids(&params, &boids_pop);
        // Create boid models to render
        boid_models.clear();
        for boid in boids_pop.iter() {
             let mut gm = Gm::new(
                Mesh::new(&context, &CpuMesh::cone(3)),
                PhysicalMaterial::new(
                    &context,
                    &CpuMaterial {
                        albedo: Srgba {
                            r: 50,
                            g: 128,
                            b: 230,
                            a: 255
                        },
                    ..Default::default()
                    },
                )
            );
            let rotation = Quaternion::from_arc(Vec3::unit_z(), boid.velocity.normalize(), None);
            let transformation = Mat4::from_translation(boid.position) * Mat4::from(rotation);
            gm.set_transformation(transformation);
            boid_models.push(gm);
        }
        let screen = frame_input.screen();
        screen 
            .clear(ClearState::color_and_depth(0.3, 0.3, 0.3, 1.0, 1.0))
            .render(
                &camera,
                &boid_models,
                &[&light0, &light1],
            );
        // screen.render(
        //     &camera,
        //     cube.into_iter().chain(&axes),
        //     &[&light0, &light1]
        // );

        FrameOutput::default()
    });
}


/* TODO
    - basic spawning
    - following behaviour
    - different colors
*/