use dxf::entities::Polyline;

struct Coordinate {
    x: f64,
    y: f64,
}

pub struct Polygon {
    style: String,
    antialias: bool,
    coordinates: Vec<Coordinate>,
    closed: bool,
}

impl From<&Polyline> for Polygon {
    fn from(poly: &Polyline) -> Self {
        Polygon {
            coordinates: poly.__vertices_and_handles.iter().map(|(vertex, _handle)| {
                Coordinate {
                    x: vertex.location.x,
                    y: vertex.location.y,
                }
            }).collect(),
            closed: poly.get_is_closed(),
            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if poly.thickness > 0.1 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }.into(),
        }
    }
}