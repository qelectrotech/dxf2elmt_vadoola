use dxf::entities;

pub struct Arc {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    start: f64,
    angle: f64,
    style: String,
    antialias: bool,
}

impl From<&entities::Arc> for Arc {
    fn from(arc: &entities::Arc) -> Self {
        let temp_angle = if arc.start_angle > arc.end_angle {
            (360.0 - arc.start_angle) + arc.end_angle
        } else {
            arc.end_angle - arc.start_angle
        };
        
        Arc {
            x: arc.center.x - arc.radius,
            y: -arc.center.y - arc.radius,
            height: arc.radius * 2.0,
            width: arc.radius * 2.0,
            start: if arc.start_angle < 0.0 {
                    -arc.start_angle
                } else {
                    arc.start_angle
                },
            angle: if temp_angle < 0.0 {
                    -temp_angle
                } else {
                    temp_angle
                },


            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if arc.thickness > 0.1 {
                    "line-style:normal;line-weight:normal;filling:none;color:black"
                } else {
                    "line-style:normal;line-weight:thin;filling:none;color:black"
                }.into(),
        }
    }
}