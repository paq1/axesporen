use std::ops;

#[derive(Clone)]
pub struct Vecteur2D<S> {
    pub x: S,
    pub y: S
}

impl<S> Vecteur2D<S> {
    pub fn new(x: S, y: S) -> Self {
        Self {x, y}
    }

    pub fn from_points(a: &Vecteur2D<f32>, b: &Vecteur2D<f32>) -> Vecteur2D<f32> {
        Vecteur2D::new(
            b.x - a.x,
            b.y - a.y
        )
    }
}

impl Vecteur2D<f32> {
    pub fn norme(&self) -> f32 {
        f32::sqrt(
            self.x * self.x + self.y * self.y
        )
    }

    pub fn unitaire(&self) -> Option<Vecteur2D<f32>> {
        let norme = self.norme();

        if norme == 0.0 {
            None
        } else {
            Some(
                Vecteur2D::new(
                    self.x / norme,
                    self.y / norme
                )
            )
        }
    }
}

impl<S> ops::Add<Vecteur2D<S>> for Vecteur2D<S>
    where
        S: ops::Add<Output = S>
{
    type Output = Vecteur2D<S>;

    fn add(self, rhs: Vecteur2D<S>) -> Self::Output {
        Vecteur2D::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl<S> ops::AddAssign<Vecteur2D<S>> for Vecteur2D<S>
    where
        S: ops::AddAssign
{
    fn add_assign(&mut self, rhs: Vecteur2D<S>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<S> ops::SubAssign<Vecteur2D<S>> for Vecteur2D<S>
    where
        S: ops::SubAssign
{
    fn sub_assign(&mut self, rhs: Vecteur2D<S>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}