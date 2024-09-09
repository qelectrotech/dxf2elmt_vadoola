use simple_xml_builder::XMLElement;
use dxf::entities::{Entity, EntityType};

pub mod solid;
pub mod lwpolyline;
pub mod polyline;
pub mod ellipse;
pub mod text;
pub mod spline;
pub mod arc;
pub mod line;
pub mod circle;


pub trait ToElemt {
    fn to_elmt(&self) -> XMLElement;
}
//what commonalities between these to_elmt functions could be pulled into shared subfunction?

impl ToElemt for (&Entity, u32, bool) {
    fn to_elmt(&self) -> XMLElement {
        let (entity, spline_step, dtext) = *self;
        match entity.specific {
            EntityType::Circle(ref circle) => circle.to_elmt(),
            EntityType::Line(ref line) => line.to_elmt(),
            EntityType::Arc(ref arc) => arc.to_elmt(),
            EntityType::Spline(ref spline) => (spline, spline_step).to_elmt(),
            EntityType::Text(ref text) => (text, entity, dtext).to_elmt(),
            EntityType::Ellipse(ref ellipse) => ellipse.to_elmt(),
            EntityType::Polyline(ref polyline) => polyline.to_elmt(),
            EntityType::LwPolyline(ref lwpolyline) => lwpolyline.to_elmt(),
            EntityType::Solid(ref solid) => solid.to_elmt(),
            _ => todo!("Need to implement the rest of the entity types"),
        }
    }
}

pub fn is_implemented(entity: &Entity) -> bool {
    use EntityType::{Circle, Line, Arc, Spline, Text, Ellipse, Polyline, LwPolyline, Solid};
    matches!(entity.specific, Circle(_) | Line(_) | Arc(_) | Spline(_) | Text(_) | Ellipse(_) | Polyline(_) | LwPolyline(_) | Solid(_))
}

#[inline]
pub fn two_dec(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}