use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Time,
    prelude::*,
    renderer::{
        plugins::{RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        mtl, 
        rendy::texture, 
        types,
        palette::{LinSrgba},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
    },
    utils::application_root_dir,
    input::{
        is_close_requested, is_key_down, InputBundle, InputEvent, ScrollDirection, StringBindings,
    },
    winit::VirtualKeyCode,
    core::{
        Transform,
        math::Vector3,
    },
    
};

mod custom_pass;

use custom_pass::{CustomUniformArgs, RenderCustom, Triangle};


pub struct CustomShaderState;

impl SimpleState for CustomShaderState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(
            Time::default()
        );
        // Add some triangles
        world
            .create_entity()
            .with(Triangle {
                points: [[0., 0.], [0., 1.], [1., 0.0]],
                colors: [[1., 0., 0., 1.], [0., 1., 0., 1.], [0., 0., 1., 1.]],
            })
            .build();
        world
            .create_entity()
            .with(Triangle {
                points: [[-1., -1.], [0., -1.], [-1., 1.0]],
                colors: [[1., 1., 0., 1.], [0., 1., 1., 1.], [1., 0., 1., 1.]],

            })
            .build();
        world
            .create_entity()
            .with(Triangle {
                points: [[0.2, -0.7], [0.4, -0.1], [0.8, -1.5]],
                colors: [[1., 0., 0., 1.], [0., 0., 0., 1.], [1., 1., 1., 1.]],
            })
            .build();

        world
            .create_entity()
            .with(Triangle {
                points: [[-0.2, 0.7], [-0.4, 0.1], [-0.8, 0.5]],
                colors: [
                    [0.337, 0.176, 0.835, 1.],
                    [0.337, 0.176, 0.835, 1.],
                    [0.337, 0.176, 0.835, 1.],
                ],
            })
            .build();
        initialise_cube(world,Vector3::zeros());
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            // Using the Mouse Wheel to control the scale
            StateEvent::Input(input) => {
                if let InputEvent::MouseWheelMoved(dir) = input {
                    let mut scale = data.world.write_resource::<CustomUniformArgs>();
                    match dir {
                        ScrollDirection::ScrollUp => (*scale).scale *= 1.1,
                        ScrollDirection::ScrollDown => (*scale).scale /= 1.1,
                        _ => {}
                    }
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?,
                )
                // Add our custom render plugin to the rendering bundle.
                .with_plugin(RenderCustom::default())
        )?;

    let mut game = Application::new(assets_dir, CustomShaderState, game_data)?;

    game.run();
    Ok(())
}



pub fn initialise_cube(world : &mut World, _pos : Vector3<f32>){
    let mesh = {
        let mesh_data: types::MeshData = Shape::Cube
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                1.0, 1.0, 1.0,
            )))
            .into();

        create_mesh(world, mesh_data)
    };
    let mat = create_material(
        world,
        LinSrgba::new(0.0, 1.0, 0.0, 1.0),
        1.0, // Metallic
        0.4, // Roughness
    );
    world
        .create_entity()
        .with(mesh)
        .with(mat)
        .with(Transform::default())
        .build();

}
pub fn create_mesh(world: &World, mesh_data: types::MeshData) -> Handle<types::Mesh> {
    // Mesh creation
    let loader = world.read_resource::<Loader>();
    let asset_storage = world.read_resource::<AssetStorage<types::Mesh>>();
    loader.load_from_data(mesh_data, (), &asset_storage)
}

pub fn create_material(
    world: &World,
    color: LinSrgba,
    metallic: f32,
    roughness: f32,
) -> Handle<mtl::Material> {
    let loader = world.read_resource::<Loader>();

    // Material creation
    let asset_storage = world.read_resource::<AssetStorage<types::Texture>>();
    let albedo = loader.load_from_data(
        texture::palette::load_from_linear_rgba(color).into(),
        (),
        &asset_storage,
    );

    let metallic_roughness = loader.load_from_data(
        texture::palette::load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0))
            .into(),
        (),
        &asset_storage,
    );

    let asset_storage = world.read_resource::<AssetStorage<mtl::Material>>();
    let mat_defaults = world.read_resource::<mtl::MaterialDefaults>().0.clone();

    loader.load_from_data(
        mtl::Material {
            albedo,
            metallic_roughness,
            ..mat_defaults
        },
        (),
        &asset_storage,
    )
}