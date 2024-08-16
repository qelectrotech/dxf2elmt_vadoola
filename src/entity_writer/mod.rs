use simple_xml_builder::XMLElement;

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