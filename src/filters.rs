use std::any::{Any, TypeId};
use geo::{Geometry, LineString, Point, Polygon};

pub fn get_filter(pattern: &str) -> fn(Geometry) -> bool {
    match pattern {
        "." => |geo: Geometry| TypeId::of::<Point>() == geo.type_id(),
        "/" => |geo: Geometry| TypeId::of::<LineString>() == geo.type_id(),
        "O" => |geo: Geometry| TypeId::of::<Polygon>() == geo.type_id(),
        _ => |geo: Geometry| true,
    }
}