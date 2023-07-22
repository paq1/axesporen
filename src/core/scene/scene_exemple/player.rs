use crate::core::physics::collide_body::CollideBody;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct Player {
    pub pos: Vecteur2D<f32>,
    pub collide_body: CollideBody,
    pub vitesse: f32
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vecteur2D::new(300f32, 300f32),
            collide_body: CollideBody::basic(Vecteur2D::new(300f32, 300f32), 16f32),
            vitesse: 600f32
        }
    }
}