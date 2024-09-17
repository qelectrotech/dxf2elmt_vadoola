use super::two_dec;
use super::ToElemt;
use dxf::entities::Entity;
use dxf::entities::Text;
use simple_xml_builder::XMLElement;
use uuid::Uuid;

impl ToElemt for (&Text, &Entity, bool) {
    fn to_elmt(&self) -> XMLElement {
        let (text, e, dynamic_text) = *self;

        let mut text_xml: XMLElement = XMLElement::new("");

        if dynamic_text {
            text_xml = XMLElement::new("dynamic_text");

            text_xml.add_attribute("x", two_dec(text.location.x));
            text_xml.add_attribute("y", two_dec(-text.location.y));
            text_xml.add_attribute("z", two_dec(text.location.z));
            if text.rotation.abs().round() as i64 % 360 != 0 {
                text_xml.add_attribute("rotation", two_dec(text.rotation - 180.0));
            } else {
                text_xml.add_attribute("rotation", 0);
            }

            text_xml.add_attribute("uuid", format!("{{{}}}", Uuid::new_v4()));

            let mut tmp = &text.text_style_name[..];
            if tmp == "STANDARD" {
                tmp = "Arial Narrow";
            }
            text_xml.add_attribute(
                "font",
                format!(
                    "{},{},-1,5,0,0,0,0,0,0,normal",
                    tmp,
                    text.text_height.ceil()
                ),
            );

            text_xml.add_attribute("Halignment", "AlignHCenter");
            text_xml.add_attribute("Valignment", "AlignVCenter");
            text_xml.add_attribute("text_from", "UserText");
            text_xml.add_attribute("frame", "false");
            text_xml.add_attribute("text_width", "-1");

            let mut text_field_xml = XMLElement::new("text");
            text_field_xml.add_text(&text.value[..]);

            text_xml.add_child(text_field_xml);

            let temp_color: String = format!("{:x}", e.common.color_24_bit);
            let mut text_color: String = String::new();
            let mut i: usize = temp_color.chars().count();
            text_color += "#";
            loop {
                if i >= 6 {
                    break;
                }
                text_color += "0";
                i += 1;
            }
            text_color += &temp_color;

            let mut text_color_xml = XMLElement::new("color");
            text_color_xml.add_text(text_color);

            text_xml.add_child(text_color_xml);
        }

        if !dynamic_text {
            text_xml = XMLElement::new("text");

            text_xml.add_attribute("x", two_dec(text.location.x));
            text_xml.add_attribute("y", two_dec(-text.location.y));
            if text.rotation.abs().round() as i64 % 360 != 0 {
                text_xml.add_attribute("rotation", two_dec(text.rotation - 180.0));
            } else {
                text_xml.add_attribute("rotation", 0);
            }

            let temp_color: String = format!("{:x}", e.common.color_24_bit);
            let mut text_color: String = String::new();
            let mut i: usize = temp_color.chars().count();
            text_color += "#";
            loop {
                if i >= 6 {
                    break;
                }
                text_color += "0";
                i += 1;
            }
            text_color += &temp_color;
            text_xml.add_attribute("color", text_color);

            let mut tmp = &text.text_style_name[..];
            if tmp == "STANDARD" {
                tmp = "Arial Narrow";
            }
            text_xml.add_attribute("text", &text.value[..]);
            text_xml.add_attribute(
                "font",
                format!(
                    "{},{},-1,5,0,0,0,0,0,0,normal",
                    tmp,
                    text.text_height.ceil()
                ),
            );
        }
        text_xml
    }
}
