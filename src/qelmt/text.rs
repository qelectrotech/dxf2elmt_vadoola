use dxf::entities;
use hex_color::HexColor;
use simple_xml_builder::XMLElement;
use super::two_dec;

#[derive(Debug)]
pub struct Text {
    rotation: f64,
    value: String,
    x: f64,
    y: f64,
    font: String,
    color: HexColor,
}

impl From<(&entities::Text, HexColor)> for Text {
    fn from((txt, color): (&entities::Text, HexColor)) -> Self {
        Text {
            x: txt.location.x,
            y: -txt.location.y,
            rotation: if txt.rotation.abs().round() as i64 % 360 != 0 {
                txt.rotation - 180.0
            } else {
                0.0
            },
            color,
            font: if &txt.text_style_name == "STANDARD" {
                "Arial Narrow".into()
            } else {
                txt.text_style_name.clone()
            },
            value: txt.value.clone(),
        }
    }
}

impl From<&Text> for XMLElement {
    fn from(txt: &Text) -> Self {
        let mut txt_xml: XMLElement = XMLElement::new("text");
        txt_xml.add_attribute("x", two_dec(txt.x));
        txt_xml.add_attribute("y", two_dec(txt.y));
        txt_xml.add_attribute("rotation", two_dec(txt.rotation));
        txt_xml.add_attribute("color", txt.color.display_rgb());
        txt_xml.add_attribute("font", &txt.font);
        txt_xml.add_attribute("text", &txt.value);
        txt_xml
    }
}