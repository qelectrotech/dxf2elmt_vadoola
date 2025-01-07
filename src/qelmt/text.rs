use super::{two_dec, FontInfo, ScaleEntity};
use dxf::entities;
use hex_color::HexColor;
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Text {
    rotation: f64,
    value: String,
    pub x: f64,
    pub y: f64,
    font: FontInfo,
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
                //"Arial Narrow".into()
                Default::default()
            } else {
                //txt.text_style_name.clone()
                Default::default()
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

impl ScaleEntity for Text {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.x *= fact_x;
        self.y *= fact_y;

        //right now there is no processing of the font string
        //the logic for the font string is just statically generating it
        //as origionally done by Antonio. I will have to add some sort of processing
        //of the font string and store it's components to make it easier to manipulate
        //such as scaling of the fonts etc.
        todo!();
        //font_size *= factX.min(factY);
    }

    fn left_bound(&self) -> f64 {
        self.x
    }

    fn top_bound(&self) -> f64 {
        self.y
    }

    fn right_bound(&self) -> f64 {
        //need to be able to measure text size to get this
        todo!()
    }

    fn bot_bound(&self) -> f64 {
        //need to be able to measure text size to get this
        todo!()
    }
}
