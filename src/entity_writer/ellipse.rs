use dxf::entities::Ellipse;
use simple_xml_builder::XMLElement;
use super::ToElemt;

impl ToElemt for Ellipse {
    fn to_elmt(&self) -> XMLElement {
        let mut ellipse_xml: XMLElement = XMLElement::new("ellipse");
        ellipse_xml.add_attribute("x", self.center.x - self.major_axis.x);
        ellipse_xml.add_attribute(
            "y",
            -self.center.y - self.major_axis.x * self.minor_axis_ratio,
        );
        ellipse_xml.add_attribute("height", self.major_axis.x * 2.0);
        ellipse_xml.add_attribute(
            "width",
            self.major_axis.x * 2.0 * self.minor_axis_ratio,
        );
        ellipse_xml.add_attribute("antialias", "false");
        ellipse_xml.add_attribute(
            "style",
            "line-style:normal;line-weight:thin;filling:none;color:black",
        );
        ellipse_xml
    }
}