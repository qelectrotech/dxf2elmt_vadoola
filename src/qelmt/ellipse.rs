use dxf::entities::{self, Circle};

pub struct Ellipse {
    height: f64,
    width: f64,
    style: String,
    x: f64,
    y: f64,
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
