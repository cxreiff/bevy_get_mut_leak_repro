use bevy::{
    asset::Assets,
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Query, ResMut},
    },
    input::common_conditions::input_pressed,
    prelude::Mesh2d,
    prelude::*,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update.run_if(input_pressed(KeyCode::Space)))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_indices(Indices::U32(Vec::new()));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());

    commands.spawn(Mesh2d(meshes.add(mesh)));
}

fn update(mut q_term: Query<&Mesh2d>, mut meshes: ResMut<Assets<Mesh>>) {
    for mesh_handle in &mut q_term {
        let mesh = meshes
            .get_mut(&mesh_handle.0.clone())
            .expect("Error getting terminal mesh");

        let tile_count = 90_000;

        let Some(Indices::U32(indices)) = mesh.indices_mut() else {
            panic!();
        };
        indices.resize(tile_count * 6, 0);

        let Some(VertexAttributeValues::Float32x3(verts)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!();
        };
        verts.resize(tile_count * 4, [0.0; 3]);
    }
}
