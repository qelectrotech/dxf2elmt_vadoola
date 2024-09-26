use super::two_dec;
use dxf::entities::{self, Circle};
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Ellipse {
    height: f64,
    width: f64,
    style: String,

    //need to brush up on my Rust scoping rules, isn't there a way to make this pub to just the module?
    pub x: f64,
    pub y: f64,

    antialias: bool,
}

impl From<&Circle> for Ellipse {
    fn from(circ: &Circle) -> Self {
        Ellipse {
            x: circ.center.x - circ.radius,
            y: -circ.center.y - circ.radius,
            height: circ.radius * 2.0,
            width: circ.radius * 2.0,

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if circ.thickness > 0.5 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl From<&entities::Ellipse> for Ellipse {
    fn from(ellipse: &entities::Ellipse) -> Self {
        Ellipse {
            x: ellipse.center.x - ellipse.major_axis.x,
            y: -ellipse.center.y - ellipse.major_axis.x * ellipse.minor_axis_ratio,
            height: ellipse.major_axis.x * 2.0,
            width: ellipse.major_axis.x * 2.0 * ellipse.minor_axis_ratio,

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: "line-style:normal;line-weight:thin;filling:none;color:black".into(),
        }
    }
}

impl From<&Ellipse> for XMLElement {
    fn from(ell: &Ellipse) -> Self {
        let mut ell_xml: XMLElement = XMLElement::new("ellipse");
        ell_xml.add_attribute("x", two_dec(ell.x));
        ell_xml.add_attribute("y", two_dec(ell.y));
        ell_xml.add_attribute("width", two_dec(ell.width));
        ell_xml.add_attribute("height", two_dec(ell.height));
        ell_xml.add_attribute("antialias", ell.antialias);
        ell_xml.add_attribute("style", &ell.style);
        ell_xml
    }
}
