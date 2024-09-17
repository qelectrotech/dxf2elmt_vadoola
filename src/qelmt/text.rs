use dxf::entities;
use hex_color::HexColor;

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
