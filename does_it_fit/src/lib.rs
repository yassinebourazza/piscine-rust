pub mod areas_volumes;

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit((x, y): (usize, usize),kind: areas_volumes::GeometricalShapes,times: usize,(a, b): (usize, usize),) -> bool {
    
    let res: f64 = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
    };
    (x*y) as f64 >= res * (times as f64)
}

pub fn volume_fit((x, y, z): (usize, usize, usize),kind: areas_volumes::GeometricalVolumes,times: usize,(a, b, c): (usize, usize, usize),) -> bool {

    let res : f64 = match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::TriangularPyramid => areas_volumes::triangular_pyramid_volume(a as f64, b),
    };

    (x * y * z) as f64 >= res * (times as f64)


}
