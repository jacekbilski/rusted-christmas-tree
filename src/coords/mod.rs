use std::f32::consts::FRAC_PI_2;

use cgmath::num_traits::Float;
use cgmath::Point3;

/// A point P in 3-dimensional space.
/// Unlike cgmath::Point3 it uses spherical coordinates instead of cartesian.
/// The coordinate system itself is setup as in OpenGL with X axis pointing to the right, Y axis pointing upwards and Z axis pointing towards the camera so it's right-handed.
/// r is the radial (Euclidean) distance between the P and O (0, 0, 0).
/// theta (θ) is the polar angle between the positive part of Y axis and the OP line segment.
/// phi (φ) is the azimuth or azimuthal angle, an angle between the positive part of Z axis and the orthogonal projection of the line segment OP on the OXZ plane.
/// All angles are given in radians
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct SphericalPoint3<T> {
    pub r: T,
    pub theta: T,
    pub phi: T,
}

impl<T> SphericalPoint3<T> {
    fn new(r: T, theta: T, phi: T) -> Self {
        SphericalPoint3 { r, theta, phi }
    }
}

impl<T: Float> Into<Point3<T>> for SphericalPoint3<T> {
    fn into(self) -> Point3<T> {
        let x = self.r * self.theta.sin() * self.phi.sin();
        let y = self.r * self.theta.cos();
        let z = self.r * self.theta.sin() * self.phi.cos();
        Point3::new(x, y, z)
    }
}

impl<T: Float> From<Point3<T>> for SphericalPoint3<T> {
    fn from(p: Point3<T>) -> Self {
        let zero = T::from(0.).unwrap();

        let r = (p.x.powi(2) + p.y.powi(2) + p.z.powi(2)).sqrt();
        let theta = if r == zero {
            zero
        } else {
            (p.y / r).acos()
        };
        let phi = if p.z == zero {
            if p.x == zero {
                zero
            } else {
                if p.x.is_sign_positive() {
                    T::from(FRAC_PI_2).unwrap()
                } else {
                    T::from(3. * FRAC_PI_2).unwrap()
                }
            }
        } else {
            (p.x / p.z).atan()
        };
        SphericalPoint3::new(r, theta, phi)
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts::FRAC_PI_2;
    use core::f32::consts::FRAC_PI_4;

    use cgmath::Point3;
    use rstest::*;

    use crate::coords::SphericalPoint3;

    #[rstest(sp, expected,
    case(SphericalPoint3::new(0., 0., 0.), Point3::new(0., 0., 0.)),
    case(SphericalPoint3::new(1., 0., 0.), Point3::new(0., 1., 0.)),
    case(SphericalPoint3::new(2., 0., 0.), Point3::new(0., 2., 0.)),
    case(SphericalPoint3::new(1., FRAC_PI_2, 0.), Point3::new(0., 0., 1.)),
    case(SphericalPoint3::new(3., FRAC_PI_2, FRAC_PI_2), Point3::new(3., 0., 0.)),
    case(SphericalPoint3::new(3., FRAC_PI_4, FRAC_PI_2), Point3::new((4.5 as f32).sqrt(), (4.5 as f32).sqrt(), 0.)),
    case(SphericalPoint3::new(3., FRAC_PI_2, FRAC_PI_4), Point3::new((4.5 as f32).sqrt(), 0., (4.5 as f32).sqrt())),
    case(SphericalPoint3::new(3., FRAC_PI_4, 0.), Point3::new(0., (4.5 as f32).sqrt(), (4.5 as f32).sqrt())),
    case(SphericalPoint3::new(5., FRAC_PI_4, FRAC_PI_4), Point3::new(2.5, (12.5 as f32).sqrt(), 2.5)),
    )]
    fn into_point3(sp: SphericalPoint3<f32>, expected: Point3<f32>) {
        let result: Point3<f32> = sp.into();
        let x_diff = (result.x - expected.x).abs();
        let y_diff = (result.y - expected.y).abs();
        let z_diff = (result.z - expected.z).abs();

        assert!(x_diff < 2. * f32::EPSILON, "x difference too high: {}", x_diff);
        assert!(y_diff < 2. * f32::EPSILON, "y difference too high: {}", y_diff);
        assert!(z_diff < 2. * f32::EPSILON, "z difference too high: {}", z_diff);
    }

    #[rstest(p, expected,
    case(Point3::new(0., 0., 0.), SphericalPoint3::new(0., 0., 0.)),
    case(Point3::new(0., 1., 0.), SphericalPoint3::new(1., 0., 0.)),
    case(Point3::new(0., 2., 0.), SphericalPoint3::new(2., 0., 0.)),
    case(Point3::new(0., 0., 1.), SphericalPoint3::new(1., FRAC_PI_2, 0.)),
    case(Point3::new(3., 0., 0.), SphericalPoint3::new(3., FRAC_PI_2, FRAC_PI_2)),
    case(Point3::new(3., 3., 0.), SphericalPoint3::new((18 as f32).sqrt(), FRAC_PI_4, FRAC_PI_2)),
    case(Point3::new(3., 0., 3.), SphericalPoint3::new((18 as f32).sqrt(), FRAC_PI_2, FRAC_PI_4)),
    case(Point3::new(0., 3., 3.), SphericalPoint3::new((18 as f32).sqrt(), FRAC_PI_4, 0.)),
    case(Point3::new(4., 4., 4.), SphericalPoint3::new((48 as f32).sqrt(), (4. / (48 as f32).sqrt()).acos(), FRAC_PI_4)),
    )]
    fn from_point3(p: Point3<f32>, expected: SphericalPoint3<f32>) {
        let result: SphericalPoint3<f32> = SphericalPoint3::from(p);
        let r_diff = (result.r - expected.r).abs();
        let theta_diff = (result.theta - expected.theta).abs();
        let phi_diff = (result.phi - expected.phi).abs();

        assert!(r_diff < 2. * f32::EPSILON, "r difference too high: {}, expected: {}, got: {}", r_diff, expected.r, result.r);
        assert!(theta_diff < 2. * f32::EPSILON, "theta difference too high: {}, expected: {}, got: {}", theta_diff, expected.theta, result.theta);
        assert!(phi_diff < 2. * f32::EPSILON, "phi difference too high: {}, expected: {}, got: {}", phi_diff, expected.phi, result.phi);
    }
}
