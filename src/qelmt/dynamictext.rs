use dxf::entities;
use hex_color::HexColor;
use uuid::Uuid;

use super::{HAlignment, VAlignment};

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

            //I don't recall off the top of my head if DXF Supports tet alignment...check
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
