use cgmath::Point3;

/// A point P in 3-dimensional space.
/// Unlike cgmath::Point3 it uses spherical coordinates instead of cartesian.
/// The coordinate system itself is setup as in OpenGL with X axis pointing to the right, Y axis pointing upwards and Z axis pointing towards the camera so it's right-handed.
/// r is the radial (Euclidean) distance between the P and O (0, 0, 0).
/// theta (θ) is the polar angle between the positive part of Y axis and the OP line segment.
/// phi (φ) is the azimuth or azimuthal angle, an angle between the positive part of Z axis and the orthogonal projection of the line segment OP on the OXZ plane.
/// All angles are given in radians
pub struct SphericalPoint3<T> {
    r: T,
    theta: T,
    phi: T,
}

impl<T> SphericalPoint3<T> {
    fn new(r: T, theta: T, phi: T) -> Self {
        SphericalPoint3 { r, theta, phi }
    }
}

impl<T> Into<Point3<T>> for SphericalPoint3<T> {
    fn into(self) -> Point3<T> {
        Point3::new(self.r, self.theta, self.phi)
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Point3;

    use crate::coords::SphericalPoint3;

    #[test]
    fn all_zeroes_become_all_zeroes() {
        let sp: SphericalPoint3<f32> = SphericalPoint3::new(0., 0., 0.);
        let result: Point3<f32> = sp.into();
        let expected: Point3<f32> = Point3::new(0., 0., 0.);

        assert_eq!(result, expected);
    }
}
