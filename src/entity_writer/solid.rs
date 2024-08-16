use dxf::entities::Solid;
use simple_xml_builder::XMLElement;
use super::ToElemt;

impl ToElemt for Solid {
    fn to_elmt(&self) -> XMLElement {
        let mut solid_xml: XMLElement = XMLElement::new("polygon");
        solid_xml.add_attribute("x1", self.first_corner.x);
        solid_xml.add_attribute("y1", -self.first_corner.y);
        solid_xml.add_attribute("x2", self.second_corner.x);
        solid_xml.add_attribute("y2", -self.second_corner.y);
        solid_xml.add_attribute("x3", self.third_corner.x);
        solid_xml.add_attribute("y3", -self.third_corner.y);
        solid_xml.add_attribute("x4", self.fourth_corner.x);
        solid_xml.add_attribute("y4", -self.fourth_corner.y);
        solid_xml.add_attribute("closed", "true");
        solid_xml.add_attribute("antialias", "false");
        if self.thickness > 0.5 {
            solid_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:normal;filling:none;color:black",
            );
        } else {
            solid_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:thin;filling:none;color:black",
            );
        }
        solid_xml
    }
}