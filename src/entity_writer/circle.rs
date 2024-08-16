use dxf::entities::Circle;
use simple_xml_builder::XMLElement;
use super::ToElemt;

impl ToElemt for Circle {
    fn to_elmt(&self) -> XMLElement {
        let mut circle_xml: XMLElement = XMLElement::new("ellipse");
        circle_xml.add_attribute("x", self.center.x - self.radius);
        circle_xml.add_attribute("y", -self.center.y - self.radius);
        circle_xml.add_attribute("height", self.radius * 2.0);
        circle_xml.add_attribute("width", self.radius * 2.0);
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