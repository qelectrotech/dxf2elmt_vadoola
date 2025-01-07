use super::{two_dec, FontInfo, ScaleEntity};
use dxf::entities;
use hex_color::HexColor;
use simple_xml_builder::XMLElement;
use uuid::Uuid;

use super::{HAlignment, VAlignment};

#[derive(Debug)]
pub struct DynamicText {
    text: String,
    info_name: Option<String>,
    pub x: f64,
    pub y: f64,
    z: f64,
    rotation: f64,
    uuid: Uuid,
    h_alignment: HAlignment,
    font: FontInfo,
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
                //"Arial Narrow".into()
                Default::default()
            } else {
                //txt.text_style_name.clone()
                Default::default()
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

impl From<(&entities::MText, HexColor)> for DynamicText {
    fn from((txt, color): (&entities::MText, HexColor)) -> Self {
        /*dbg!(&txt.insertion_point);
        dbg!(&txt.text);
        for t in &txt.extended_text {
            dbg!(t);
        }*/

        DynamicText {
            x: txt.insertion_point.x,
            y: -txt.insertion_point.y,
            z: txt.insertion_point.z,
            rotation: if txt.rotation_angle.abs().round() as i64 % 360 != 0 {
                txt.rotation_angle - 180.0
            } else {
                0.0
            },
            uuid: Uuid::new_v4(),
            font: if &txt.text_style_name == "STANDARD" {
                //"Arial Narrow".into()
                Default::default()
            } else {
                //txt.text_style_name.clone()
                Default::default()
            },

            //I don't recall off the top of my head if DXF Supports text alignment...check
            h_alignment: HAlignment::Center,
            v_alignment: VAlignment::Center,

            text_from: "UserText".into(),
            frame: false,
            text_width: -1, //why is this -1, does that just mean auto calculate?
            color,

            //There are 2 text fields on MTEXT, .text a String and .extended_text a Vec<String>
            //Most of the example files I have at the moment are single line MTEXT.
            //I edited one of them in QCad, and added a few lines. The value came through in the text field
            //with extended_text being empty, and the newlines were deliniated by '\\P'...I might need to look
            //the spec a bit to determine what it says for MTEXT, but for now, I'll just assume this is correct
            text: txt.text.replace("\\P", "\n"),
            keep_visual_rotation: false,
            info_name: None,
        }
    }
}

impl From<&DynamicText> for XMLElement {
    fn from(txt: &DynamicText) -> Self {
        let mut dtxt_xml = XMLElement::new("dynamic_text");
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
        dtxt_xml.add_attribute("color", txt.color.display_rgb());

        //If I ever add support for other text_from types, element and composite text
        //I'll need to add more smarts here, as there may be some other children components
        //for now since it only supports user_text I'm just statically adding the single child
        //component needed
        //match txt.text_from
        let mut text_xml = XMLElement::new("text");
        text_xml.add_text(&txt.text);
        dtxt_xml.add_child(text_xml);

        if let Some(i_name) = &txt.info_name {
            dtxt_xml.add_attribute("info_name", i_name);
        }

        if txt.keep_visual_rotation {
            dtxt_xml.add_attribute("keep_visual_rotation", txt.keep_visual_rotation);
        }

        dtxt_xml
    }
}

impl ScaleEntity for DynamicText {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x *= fact_x;
        self.y *= fact_y;

        //right now there is no processing of the font string
        //the logic for the font string is just statically generating it
        //as origionally done by Antonio. I will have to add some sort of processing
        //of the font string and store it's components to make it easier to manipulate
        //such as scaling of the fonts etc.
        //todo!();
        //font_size *= factX.min(factY);
    }

    fn left_bound(&self) -> f64 {
        self.x
    }

    fn right_bound(&self) -> f64 {
        todo!()
    }

    fn top_bound(&self) -> f64 {
        self.y
    }

    fn bot_bound(&self) -> f64 {
        todo!()
    }
}
