use super::two_dec;
use super::LineEnd;
use dxf::entities;
use simple_xml_builder::XMLElement;

#[derive(Debug)]
pub struct Line {
    length2: f64,
    end2: LineEnd,
    length1: f64,

    //need to brush up on my Rust scoping rules, isn't there a way to make this pub to just the module?
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,

    style: String,
    end1: LineEnd,
    antialias: bool,
}

impl From<&entities::Line> for Line {
    fn from(line: &entities::Line) -> Self {
        Line {
            x1: line.p1.x,
            y1: -line.p1.y,
            length1: 1.5, //why is this statically set at 1.5?
            end1: LineEnd::None,
            x2: line.p2.x,
            y2: -line.p2.y,
            length2: 1.5, //why is this statically set at 1.5?
            end2: LineEnd::None,

            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if line.thickness > 0.5 {
                "line-style:normal;line-weight:normal;filling:none;color:black}"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl From<&Line> for XMLElement {
    fn from(line: &Line) -> Self {
        let mut line_xml: XMLElement = XMLElement::new("line");
        line_xml.add_attribute("x1", two_dec(line.x1));
        line_xml.add_attribute("y1", two_dec(line.y1));
        line_xml.add_attribute("length1", two_dec(line.length1));
        line_xml.add_attribute("end1", &line.end1);
        line_xml.add_attribute("x2", two_dec(line.x2));
        line_xml.add_attribute("y2", two_dec(line.y2));
        line_xml.add_attribute("length2", two_dec(line.length2));
        line_xml.add_attribute("end2", &line.end2);
        line_xml.add_attribute("antialias", line.antialias);
        line_xml.add_attribute("style", &line.style);
        line_xml
    }
}
