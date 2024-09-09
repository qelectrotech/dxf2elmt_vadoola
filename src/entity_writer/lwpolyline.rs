use dxf::entities::LwPolyline;
use simple_xml_builder::XMLElement;
use super::{two_dec, ToElemt};

impl ToElemt for LwPolyline {
    fn to_elmt(&self) -> XMLElement {
        let mut lwpolyline_xml: XMLElement = XMLElement::new("polygon");
        self.vertices.iter().enumerate().for_each(|(j, _i)| {
            lwpolyline_xml.add_attribute(format!("x{}", (j + 1)), two_dec(self.vertices[j].x));
            lwpolyline_xml.add_attribute(format!("y{}", (j + 1)), two_dec(-self.vertices[j].y));
        });
        
        if !self.get_is_closed() {
            lwpolyline_xml.add_attribute("closed", false);
        }

        lwpolyline_xml.add_attribute("antialias", "false");
        if self.thickness > 0.1 {
            lwpolyline_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:normal;filling:none;color:black",
            );
        } else {
            lwpolyline_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:thin;filling:none;color:black",
            );
        }
        lwpolyline_xml
    }
}