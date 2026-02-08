use std::{f32::consts::PI, marker::PhantomData};

use avian2d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    math::{self, rotate_vec2},
    physics::ObjectLayer,
};

#[derive(Clone, Debug)]
pub struct Sector {
    pub radius: f32,
    pub arc_angle: f32,
    pub center_angle: f32,
    pub min_edges_per_radian: f32,
    mesh_points: Vec<Vec2>,
    triangle_indices: Vec<[u32; 3]>,
}

impl Sector {
    pub fn new(radius: f32, arc_angle: f32, center_angle: f32, min_edges_per_radian: f32) -> Self {
        let half_span = arc_angle.clamp(0.0, 2.0 * PI) / 2.0;
        let unit = rotate_vec2(Vec2::X, center_angle - half_span);
        let initial_position = radius * unit;

        let arc_point_count = (arc_angle * min_edges_per_radian).ceil() as usize;
        let arc_angle = arc_angle / (arc_point_count - 1) as f32;

        let mut mesh_points: Vec<Vec2> = vec![Vec2::ZERO; arc_point_count + 1];
        mesh_points[1] = initial_position;
        for i in 1..arc_point_count {
            mesh_points[i + 1] = math::rotate_vec2(mesh_points[1], arc_angle * i as f32);
        }

        let mut triangle_indices: Vec<[u32; 3]> = vec![[0, 0, 0]; arc_point_count - 1];
        for i in 0..(arc_point_count - 1) {
            triangle_indices[i][0] = 0;
            triangle_indices[i][1] = (i + 1) as u32;
            triangle_indices[i][2] = (i + 2) as u32;
        }

        Sector {
            radius,
            arc_angle,
            center_angle,
            min_edges_per_radian,
            mesh_points,
            triangle_indices,
        }
    }

    fn into_mesh(&self) -> Mesh {
        let mesh_points: Vec<Vec3> = self
            .mesh_points
            .clone()
            .iter()
            .map(|v| v.extend(0.0))
            .collect();

        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_points)
            .with_inserted_indices(Indices::U32(self.triangle_indices.clone().into_flattened()))
    }

    fn into_collider(&self) -> Collider {
        Collider::trimesh(self.mesh_points.clone(), self.triangle_indices.clone())
    }
}

// TODO: implement a derive macro for this
// TODO: rename this to a more general message trait
pub trait SectorMessage: Send + Sync + Message + From<Entity> + 'static {} // Plus L plus Ratio

// TODO: rename this to sector sensor or some bullshit like that
#[derive(Component)]
#[require(Transform)]
pub struct SectorTrigger<M> {
    pub mesh: Mesh,
    pub collider: Collider,
    pub sector: Sector,
    pub mask: LayerMask,
    _message_type: PhantomData<M>,
}

impl<M> SectorTrigger<M>
where
    M: SectorMessage,
{
    pub fn new(sector: Sector) -> Self {
        Self {
            mesh: sector.into_mesh(),
            collider: sector.into_collider(),
            sector,
            mask: LayerMask::NONE,
            _message_type: PhantomData::default(),
        }
    }

    pub fn into_bundle(self, meshes: &mut ResMut<Assets<Mesh>>) -> impl Bundle {
        (
            Mesh2d(meshes.add(self.mesh.clone())),
            self.collider.clone(),
            CollisionLayers::new(LayerMask(ObjectLayer::None.to_bits()), self.mask),
            self,
        )
    }

    pub fn with_sector(self, sector: Sector) -> Self {
        Self { sector, ..self }
    }

    pub fn with_collider(self, collider: Collider) -> Self {
        Self { collider, ..self }
    }

    pub fn with_mask(self, mask: LayerMask) -> Self {
        Self { mask, ..self }
    }
}

pub struct SectorPlugin<M> {
    _message_type: PhantomData<M>,
}

impl<M> Plugin for SectorPlugin<M>
where
    M: SectorMessage,
{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, sector_trigger_detection::<M>);
    }
}

impl<M> Default for SectorPlugin<M> {
    fn default() -> Self {
        Self {
            _message_type: PhantomData::default(),
        }
    }
}

fn sector_trigger_detection<M>(
    spatial_query: Res<SpatialQueryPipeline>,
    triggers: Query<(&SectorTrigger<M>, &GlobalTransform)>,
    mut message_writer: MessageWriter<M>,
) where
    M: Message + From<Entity>,
{
    for (trigger, global_transform) in &triggers {
        let filter = SpatialQueryFilter::from_mask(trigger.mask);

        let (_, rotation, translation) = global_transform.to_scale_rotation_translation();
        let rotation_angle = rotation.to_euler(EulerRot::XYZ).2;

        for entity in spatial_query.shape_intersections(
            &trigger.collider,
            translation.xy(),
            rotation_angle,
            &filter,
        ) {
            message_writer.write(M::from(entity));
        }
    }
}
