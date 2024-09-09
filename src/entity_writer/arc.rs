use dxf::entities::Arc;
use simple_xml_builder::XMLElement;
use super::{two_dec, ToElemt};

impl ToElemt for Arc {
    fn to_elmt(&self) -> XMLElement {
        let mut arc_xml: XMLElement = XMLElement::new("arc");
        arc_xml.add_attribute("x", two_dec(self.center.x - self.radius));
        arc_xml.add_attribute("y", two_dec(-self.center.y - self.radius));
        arc_xml.add_attribute("width", two_dec(self.radius * 2.0));
        arc_xml.add_attribute("height", two_dec(self.radius * 2.0));
        if self.start_angle < 0.0 {
            arc_xml.add_attribute("start", two_dec(-self.start_angle));
        } else {
            arc_xml.add_attribute("start", two_dec(self.start_angle));
        }

        let temp = if self.start_angle > self.end_angle {
            (360.0 - self.start_angle) + self.end_angle
        } else {
            self.end_angle - self.start_angle
        };

        if temp < 0.0 {
            arc_xml.add_attribute("angle", two_dec(-temp));
        } else {
            arc_xml.add_attribute("angle", two_dec(temp));
        }
        arc_xml.add_attribute("antialias", "false");
        if self.thickness > 0.1 {
            arc_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:normal;filling:none;color:black",
            );
        } else {
            arc_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:thin;filling:none;color:black",
            );
        }
        arc_xml
    }
}
