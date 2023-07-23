use crate::core::physics::collide_body::CollideBody;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct Player {
    pub pos: Vecteur2D<f32>,
    pub collide_body: CollideBody,
    pub vitesse: f32
}

impl Player {
    pub fn new() -> Self {
        let position_de_depart = Vecteur2D::new(64f32, 64f32);
        Self {
            pos: position_de_depart.clone(),
            collide_body: CollideBody::basic(position_de_depart, 16f32),
            vitesse: 600f32
        }
    }
}