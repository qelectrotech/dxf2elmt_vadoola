use dxf::entities::Line;
use simple_xml_builder::XMLElement;
use super::{two_dec, ToElemt};

impl ToElemt for Line {
    fn to_elmt(&self) -> XMLElement {
        let mut line_xml: XMLElement = XMLElement::new("line");
        line_xml.add_attribute("x1", two_dec(self.p1.x));
        line_xml.add_attribute("y1", two_dec(-self.p1.y));
        line_xml.add_attribute("length1", 1.5);
        line_xml.add_attribute("end1", "none");
        line_xml.add_attribute("x2", two_dec( self.p2.x));
        line_xml.add_attribute("y2", two_dec(-self.p2.y));
        line_xml.add_attribute("length2", 1.5);
        line_xml.add_attribute("end2", "none");
        line_xml.add_attribute("antialias", "false");
        if self.thickness > 0.5 {
            line_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:normal;filling:none;color:black}",
            );
        } else {
            line_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:thin;filling:none;color:black",
            );
        }
        line_xml
    }
}