use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass_properties: ColliderMassProperties,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player/placeholder.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Component)]
pub struct Wall;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(
                    (entity_instance.width / 2) as f32,
                    (entity_instance.height / 2) as f32,
                ),
                rigid_body: RigidBody::KinematicPositionBased,
                rotation_constraints,
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}
