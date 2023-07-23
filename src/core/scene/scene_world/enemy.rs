use crate::core::elements::tilemap::TileMapHudge;
use crate::core::physics::collide_body::CollideBody;
use crate::core::scene::scene_world::player::Player;
use crate::core::sdd::vecteur2d::Vecteur2D;

#[derive(Clone)]
pub struct Enemy {
    pub collide_body: CollideBody,
    distance_attaque: f32,
    vitesse: f32
}

impl Enemy {
    pub fn new(position: Vecteur2D<f32>) -> Self {
        Self {
            collide_body: CollideBody::basic(
                position, 16f32
            ),
            distance_attaque: 300.0,
            vitesse: 20.0
        }
    }


    pub fn update(&mut self, dt: f32, _tilemap: &TileMapHudge, joueur: &Player) {

        let vec_enemie_joueur = Vecteur2D::<f32>::from_points(
            &self.collide_body.position,
            &joueur.pos);

        let distance_enemi_joueur = vec_enemie_joueur
            .norme();

        if self.distance_attaque > distance_enemi_joueur {
            // se deplace (attaque) vers le joueur
            let unitaire_opt = vec_enemie_joueur.unitaire();

            match unitaire_opt {
                Some(unitaire) => {
                    let deplacement_v = Vecteur2D::new(
                        unitaire.x * self.vitesse * dt,
                        unitaire.y * self.vitesse * dt
                    );
                    self.collide_body.position += deplacement_v;
                }
                None => ()
            }

        } else {
            // ne fait rien (est dans un etat bete pour le moment)
        }
        // todo update enemies
    }
}