use super::LineEnd;
use dxf::entities;

pub struct Line {
    length2: f64,
    end2: LineEnd,
    length1: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
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
            y2: line.p2.y,
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

//impl From<Line> for XMLElement
