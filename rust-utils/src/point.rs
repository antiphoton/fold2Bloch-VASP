use num_rational::Rational;
use euclid::Vector3D;
pub type Point = Vector3D<Rational, ()>;


pub fn fold(mut p: Point) -> Point {
    fn fix(a: &mut Rational) {
        let b = *a - Rational::new(1, 2);
        *a -= b.ceil();
    }
    fix(&mut p.x);
    fix(&mut p.y);
    fix(&mut p.z);
    p
}

