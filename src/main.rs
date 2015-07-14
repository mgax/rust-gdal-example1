extern crate geo;
extern crate gdal;

use std::env;
use std::path::Path;
use geo::{ToGeo, Geometry, Point, LineString, Polygon, Coordinate};
use gdal::vector::{Driver, Dataset, ToGdal};

fn points(g: Geometry) -> Vec<Point> {
    return match g {
        Geometry::Point(p) => vec!(p),
        Geometry::LineString(LineString(ls)) => ls,
        _ => panic!()
    };
}

fn min_max(mut values: Vec<f64>) -> (f64, f64) {
    use std::cmp::Ordering;
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    return (*values.first().unwrap(), *values.last().unwrap());
}

fn extent(g: Geometry) -> Geometry {
    let points = points(g);
    let (x0, x1) = min_max(points.iter().map(|&Point(c)| c.x).collect());
    let (y0, y1) = min_max(points.iter().map(|&Point(c)| c.y).collect());
    let extent = LineString(vec!(
        Point(Coordinate{x: x0, y: y0}),
        Point(Coordinate{x: x1, y: y0}),
        Point(Coordinate{x: x1, y: y1}),
        Point(Coordinate{x: x0, y: y1}),
        Point(Coordinate{x: x0, y: y0}),
    ));
    return Geometry::Polygon(Polygon(extent, vec!()));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dataset_in = Dataset::open(Path::new(&args[1])).unwrap();
    let layer_in = dataset_in.layer(0).unwrap();

    let geojson = Driver::get("GeoJSON").unwrap();
    let mut dataset_out = geojson.create(Path::new(&args[2])).unwrap();
    let mut layer_out = dataset_out.create_layer();

    for feature in layer_in.features() {
        let geometry = feature.geometry().to_geo();
        let extent = extent(geometry);
        layer_out.create_feature(extent.to_gdal());
    }
}
