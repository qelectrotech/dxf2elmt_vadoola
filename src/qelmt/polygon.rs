use super::{two_dec, ScaleEntity};
use dxf::entities::{LwPolyline, Polyline, Solid, Spline};
use simple_xml_builder::XMLElement;
use std::ops::{Add, Mul};

//wait Why do I have a coordinate AND a Point struct, that are
//essentially the same. It's been a couple of months, but I'm not
//seeing why I would have done this....almost makes me wondering
//if I started, then stopped, and then didn't realize where I left off
//and started again but used a different name...?
//Might need to take a closer look and clean this up.
#[derive(Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
pub struct Polygon {
    style: String,
    antialias: bool,
    pub coordinates: Vec<Coordinate>,
    closed: bool,
}

impl From<&Polyline> for Polygon {
    fn from(poly: &Polyline) -> Self {
        Polygon {
            coordinates: poly
                .__vertices_and_handles
                .iter()
                .map(|(vertex, _handle)| Coordinate {
                    x: vertex.location.x,
                    y: -vertex.location.y,
                })
                .collect(),
            closed: poly.is_closed(),
            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if poly.thickness > 0.1 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl From<&LwPolyline> for Polygon {
    fn from(poly: &LwPolyline) -> Self {
        Polygon {
            coordinates: poly
                .vertices
                .iter()
                .map(|vertex| Coordinate {
                    x: vertex.x,
                    y: -vertex.y,
                })
                .collect(),
            closed: poly.is_closed(),
            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if poly.thickness > 0.1 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl From<(&Spline, u32)> for Polygon {
    fn from((spline, spline_step): (&Spline, u32)) -> Self {
        let mut i: usize = 0;
        let mut points: Vec<Point> = Vec::new();
        for _a in &spline.control_points {
            points.push(Point::new(
                spline.control_points[i].x,
                spline.control_points[i].y,
            ));
            i += 1;
        }
        i = 0;
        let mut knots: Vec<f64> = Vec::new();
        for _a in &spline.knot_values {
            knots.push(spline.knot_values[i]);
            i += 1;
        }
        let curr_spline =
            bspline::BSpline::new(spline.degree_of_curve.try_into().unwrap(), points, knots);
        let step: f64 =
            (curr_spline.knot_domain().1 - curr_spline.knot_domain().0) / (spline_step as f64);

        //there is probably a way to clean up some of this logic and use iterators
        //although it looks like step_by doesn't work on a f64 range...hmmm
        //but I haven't inspected it too closely, and for now am pretty much just duplicating
        //it as antonioaja had it
        let coordinates = {
            let mut coords = Vec::with_capacity(
                ((curr_spline.knot_domain().1 - curr_spline.knot_domain().0) / step) as usize + 1,
            );
            let mut j: f64 = curr_spline.knot_domain().0;
            i = 0;
            while j < curr_spline.knot_domain().1 {
                coords.push(Coordinate {
                    x: curr_spline.point(j).x,
                    y: -curr_spline.point(j).y,
                });
                j += step;
                i += 1;
            }
            coords
        };

        Polygon {
            coordinates,
            closed: spline.is_closed(),
            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: "line-style:normal;line-weight:thin;filling:none;color:black".into(),
        }
    }
}

impl From<&Solid> for Polygon {
    fn from(solid: &Solid) -> Self {
        Polygon {
            coordinates: vec![
                Coordinate {
                    x: solid.first_corner.x,
                    y: -solid.first_corner.y,
                },
                Coordinate {
                    x: solid.second_corner.x,
                    y: -solid.second_corner.y,
                },
                Coordinate {
                    x: solid.third_corner.x,
                    y: -solid.third_corner.y,
                },
                Coordinate {
                    x: solid.fourth_corner.x,
                    y: -solid.fourth_corner.y,
                },
            ],
            closed: true,
            //in the original code antialias is always set to false...I'm guessing for performance
            //reasons...I'm trying to think if there is a time we might want to turn it on?
            antialias: false,
            style: if solid.thickness > 0.5 {
                "line-style:normal;line-weight:normal;filling:none;color:black"
            } else {
                "line-style:normal;line-weight:thin;filling:none;color:black"
            }
            .into(),
        }
    }
}

impl From<&Polygon> for XMLElement {
    fn from(poly: &Polygon) -> Self {
        let mut poly_xml: XMLElement = XMLElement::new("polygon");

        for (count, coord) in poly.coordinates.iter().enumerate() {
            poly_xml.add_attribute(format!("x{}", (count + 1)), two_dec(coord.x));
            poly_xml.add_attribute(format!("y{}", (count + 1)), two_dec(coord.y));
        }

        //closed defaults to true, don't need to write it out unless it's false
        if !poly.closed {
            poly_xml.add_attribute("closed", poly.closed);
        }

        poly_xml.add_attribute("antialias", poly.antialias);
        poly_xml.add_attribute("style", &poly.style);
        poly_xml
    }
}

impl ScaleEntity for Polygon {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.coordinates.iter_mut().for_each(|coord| {
            coord.x *= fact_x;
            coord.y *= fact_y;
        });
    }
}
