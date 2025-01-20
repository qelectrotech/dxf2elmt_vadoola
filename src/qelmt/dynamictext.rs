use super::{two_dec, FontInfo, ScaleEntity, TextEntity};
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
    }

    fn left_bound(&self) -> f64 {
        self.x
    }

    fn right_bound(&self) -> f64 {
        //todo!()
        1.0
    }

    fn top_bound(&self) -> f64 {
        self.y
    }

    fn bot_bound(&self) -> f64 {
        //todo!()
        1.0
    }
}

pub struct DTextBuilder<'a> {
    text: TextEntity<'a>,
    color: Option<HexColor>,
    txt_sc_factor: Option<f64>,
}

impl<'a> DTextBuilder<'a> {
    pub fn from_text(text: &'a entities::Text) -> Self {
        Self {
            text: TextEntity::Text(text),
            color: None,
            txt_sc_factor: None,
        }
    }

    pub fn from_mtext(text: &'a entities::MText) -> Self {
        Self {
            text: TextEntity::MText(text),
            color: None,
            txt_sc_factor: None,
        }
    }

    pub fn color(self, color: HexColor) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }

    pub fn scaling(self, txt_sc_factor: f64) -> Self {
        Self {
            txt_sc_factor: Some(txt_sc_factor),
            ..self
        }
    }

    pub fn build(self) -> DynamicText {
        let (x, y, z, rotation, style_name, text_height, value) = match self.text {
            TextEntity::Text(txt) => (
                txt.location.x,
                -txt.location.y,
                txt.location.z,
                txt.rotation,
                &txt.text_style_name,
                txt.text_height,
                txt.value.clone(),
            ),
            TextEntity::MText(mtxt) => {
                (
                    mtxt.insertion_point.x,
                    -mtxt.insertion_point.y,
                    mtxt.insertion_point.z,
                    mtxt.rotation_angle,
                    &mtxt.text_style_name,
                    //I'm not sure what the proper value is here for Mtext
                    //becuase I haven't actually finished supporting it.
                    //I'll put initial text height for now. But i'm not certain
                    //exactly what this correlates to. There is also vertical_height,
                    //which I would guess is the total vertical height for all the lines
                    //it's possible I would need to take the vertical height and divide
                    //by the number of lines to get the value I need....I'm not sure yet
                    mtxt.initial_text_height,
                    //There are 2 text fields on MTEXT, .text a String and .extended_text a Vec<String>
                    //Most of the example files I have at the moment are single line MTEXT.
                    //I edited one of them in QCad, and added a few lines. The value came through in the text field
                    //with extended_text being empty, and the newlines were deliniated by '\\P'...I might need to look
                    //the spec a bit to determine what it says for MTEXT, but for now, I'll just assume this is correct
                    mtxt.text.replace("\\P", "\n"),
                )
            }
        };

        dbg!(&value);
        dbg!(&y);
        dbg!(&self.text);
        DynamicText {
            x,
            y,
            z,
            rotation: if rotation.abs().round() as i64 % 360 != 0 {
                rotation - 180.0
            } else {
                0.0
            },
            uuid: Uuid::new_v4(),
            font: if style_name == "STANDARD" {
                FontInfo {
                    point_size: if let Some(tsf) = self.txt_sc_factor {
                        text_height * tsf
                    } else {
                        //if we don't have a scaling factor default to 12pt font
                        12.0
                    },
                    ..Default::default()
                }
            } else {
                //clearly right now this is exactly the same as the main body of the if
                //I'm jus putting this in for now, to compile while I get he font handling
                //working correctly
                FontInfo {
                    point_size: if let Some(tsf) = self.txt_sc_factor {
                        text_height * tsf
                    } else {
                        //if we don't have a scaling factor default to 12pt font
                        12.0
                    },
                    ..Default::default()
                }
            },
            //I don't recall off the top of my head if DXF Supports text alignment...check
            h_alignment: HAlignment::Center,
            v_alignment: VAlignment::Center,

            text_from: "UserText".into(),
            frame: false,
            text_width: -1, //why is this -1, does that just mean auto calculate?
            color: self.color.unwrap_or(HexColor::BLACK),

            text: value,
            keep_visual_rotation: false,
            info_name: None,
        }
    }
}
