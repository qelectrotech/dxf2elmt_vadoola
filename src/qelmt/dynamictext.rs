use dxf::entities;
use hex_color::HexColor;
use simple_xml_builder::XMLElement;
use uuid::Uuid;
use super::two_dec;

use super::{HAlignment, VAlignment};

#[derive(Debug)]
pub struct DynamicText {
    text: String,
    info_name: Option<String>,
    x: f64,
    y: f64,
    z: f64,
    rotation: f64,
    uuid: Uuid,
    h_alignment: HAlignment,
    font: String,
    text_from: String,
    v_alignment: VAlignment,
    frame: bool,
    text_width: i32,
    keep_visual_rotation: bool,
    color: HexColor,
}

impl From<(&entities::Text, HexColor)> for DynamicText {
    fn from((txt, color): (&entities::Text, HexColor)) -> Self {
        DynamicText {
            x: txt.location.x,
            y: -txt.location.y,
            z: txt.location.z,
            rotation: if txt.rotation.abs().round() as i64 % 360 != 0 {
                txt.rotation - 180.0
            } else {
                0.0
            },
            uuid: Uuid::new_v4(),
            font: if &txt.text_style_name == "STANDARD" {
                "Arial Narrow".into()
            } else {
                txt.text_style_name.clone()
            },

            //I don't recall off the top of my head if DXF Supports text alignment...check
            h_alignment: HAlignment::Center,
            v_alignment: VAlignment::Center,

            text_from: "UserText".into(),
            frame: false,
            text_width: -1, //why is this -1, does that just mean auto calculate?
            color,

            text: txt.value.clone(),
            keep_visual_rotation: false,
            info_name: None,
        }
    }
}

impl From<&DynamicText> for XMLElement {
    fn from(txt: &DynamicText) -> Self {
        let mut dtxt_xml: XMLElement = XMLElement::new("dynamic_text");
        dtxt_xml.add_attribute("x", two_dec(txt.x));
        dtxt_xml.add_attribute("y", two_dec(txt.y));
        dtxt_xml.add_attribute("z", two_dec(txt.z));
        dtxt_xml.add_attribute("rotation", two_dec(txt.rotation));
        dtxt_xml.add_attribute("uuid", format!("{{{}}}", txt.uuid));
        dtxt_xml.add_attribute("font", &txt.font);
        dtxt_xml.add_attribute("Halignment", &txt.h_alignment);
        dtxt_xml.add_attribute("Valignment", &txt.v_alignment);
        dtxt_xml.add_attribute("text_from", &txt.text_from);
        dtxt_xml.add_attribute("frame", txt.frame);
        dtxt_xml.add_attribute("text_width", txt.text_width);
        dtxt_xml.add_attribute("text", &txt.text);
        dtxt_xml.add_attribute("color", txt.color.display_rgb());

        if let Some(i_name) = &txt.info_name {
            dtxt_xml.add_attribute("info_name", i_name);
        }

        if txt.keep_visual_rotation {
            dtxt_xml.add_attribute("keep_visual_rotation", txt.keep_visual_rotation);
        }

        dtxt_xml
    }
}
