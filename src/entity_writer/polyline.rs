use dxf::entities::Polyline;
use simple_xml_builder::XMLElement;
use super::ToElemt;

impl ToElemt for Polyline {
    fn to_elmt(&self) -> XMLElement {
        let mut polyline_xml: XMLElement = XMLElement::new("polygon");
        self
            .__vertices_and_handles
            .iter()
            .enumerate()
            .for_each(|(j, _i)| {
                polyline_xml.add_attribute(
                    format!("x{}", (j + 1)),
                    self.__vertices_and_handles[j].0.location.x,
                );
                polyline_xml.add_attribute(
                    format!("y{}", (j + 1)),
                    -self.__vertices_and_handles[j].0.location.y,
                );
            });

        polyline_xml.add_attribute("closed", "false");
        polyline_xml.add_attribute("antialias", "false");

        if self.thickness > 0.1 {
            polyline_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:normal;filling:none;color:black",
            );
        } else {
            polyline_xml.add_attribute(
                "style",
                "line-style:normal;line-weight:thin;filling:none;color:black",
            );
        }

        polyline_xml
    }
}