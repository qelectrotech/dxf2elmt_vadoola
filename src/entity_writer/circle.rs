use super::{two_dec, ToElemt};
use dxf::entities::Circle;
use simple_xml_builder::XMLElement;

impl ToElemt for Circle {
    fn to_elmt(&self) -> XMLElement {
        let mut circle_xml: XMLElement = XMLElement::new("ellipse");
        circle_xml.add_attribute("x", two_dec(self.center.x - self.radius));
        circle_xml.add_attribute("y", two_dec(-self.center.y - self.radius));
        circle_xml.add_attribute("height", two_dec(self.radius * 2.0));
        circle_xml.add_attribute("width", two_dec(self.radius * 2.0));
        circle_xml.add_attribute("antialias", "false");
        if self.thickness > 0.5 {
            circle_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:normal;filling:none;color:black",
            );
        } else {
            circle_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:thin;filling:none;color:black",
            );
        }

        circle_xml
    }
}
