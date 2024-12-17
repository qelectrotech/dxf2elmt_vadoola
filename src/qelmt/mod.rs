use dxf::enums::Units;
use simple_xml_builder::XMLElement;
//use serde::{Deserialize, Serialize};
use uuid::Uuid;
//use std::str::FromStr;
//use strum::EnumString;
use dxf::entities::{Entity, EntityType};
use dxf::Drawing;
use hex_color::HexColor;
use std::convert::TryFrom;
use std::fmt::Display;
use itertools::Itertools;
use dxf::entities::{LwPolyline, Polyline};
use std::f64::consts::PI;

pub mod arc;
pub use arc::Arc;

pub mod line;
pub use line::Line;

pub mod text;
pub use text::Text;

pub mod dynamictext;
pub use dynamictext::DynamicText;

pub mod polygon;
pub use polygon::Polygon;

pub mod ellipse;
pub use ellipse::Ellipse;

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug)]
pub struct Definition {
    r#type: ItemType,
    width: f64,
    height: f64,
    hotspot_x: f64,
    hotspot_y: f64,
    version: String,
    link_type: LinkType,
    uuid: ElmtUuid,
    names: Names,
    element_infos: Option<ElemInfos>,
    informations: Option<String>,
    description: Description,
    //counts
}

trait ScaleEntity {
    //honestly would I ever want to scale the x and y by different dimensions?
    //I'm thinking maybe I just have a single scale factor, it always scales proportiontly
    fn scale(&mut self, fact_x: f64, fact_y: f64);
}

trait Circularity {
    fn is_circular(&self) -> bool;
}

impl Circularity for Polyline {
    fn is_circular(&self) -> bool {
        let poly_perim: f64 = {
            let tmp_pts: Vec<dxf::Point> = self.vertices().map(|v| v.clone().location).collect();
            let len = tmp_pts.len();
            tmp_pts.into_iter()
            .circular_tuple_windows()
            .map(|(fst, sec)| {
                ((fst.x - sec.x).powf(2.0) + (fst.y - sec.y).powf(2.0)).sqrt()
            })
            .take(len)
            .sum()
        };

        let poly_area = {
            //because instead of being able to access the Vec like in LwPolyline, verticies() returns
            //an iter of dxf Vertex's which don't implment clone so I can't use circular_tuple_windows
            //there is probably a cleaner way of iterating over this, but it's late, I'm getting tired
            //and just want to see if this basic idea will work on my sample file, or see if I'm chasing
            //up the wrong tree.
            let tmp_pts: Vec<dxf::Point> = self.vertices().map(|v| v.clone().location).collect();
            let len = tmp_pts.len();
            let mut poly_area: f64 = tmp_pts.into_iter()
            .circular_tuple_windows()
            .map(|(fst, sec)| {
                (fst.x * sec.y) - (fst.y * sec.x)
            })
            .take(len)
            .sum();
            poly_area /= 2.0;
            poly_area.abs()
        };
        let t_ratio = 4.0 * PI * poly_area / poly_perim.powf(2.0);

        (0.98..=1.02).contains(&t_ratio)
    }
}

impl Circularity for LwPolyline {
    fn is_circular(&self) -> bool {
        let poly_perim: f64 = self
            .vertices
            .iter()
            .circular_tuple_windows()
            .map(|(fst, sec)| {
                ((fst.x - sec.x).powf(2.0) + (fst.y - sec.y).powf(2.0)).abs().sqrt()
            })
            .take(self.vertices.len())
            .sum();

        let poly_area = {
            let mut poly_area: f64 = self
            .vertices
            .iter()
            .circular_tuple_windows()
            .map(|(fst, sec)| {
                (fst.x * sec.y) - (fst.y * sec.x)
            })
            .take(self.vertices.len())
            .sum();
            poly_area /= 2.0;
            poly_area.abs()
        };
        let t_ratio = 4.0 * PI * poly_area / poly_perim.powf(2.0);

        //this boundry of 2% has been chosen arbitrarily, I might adjust this later
        //I know in on of my sample files, I'm geting a value of 0.99....
        (0.98..=1.02).contains(&t_ratio)
    }
}

impl Definition {
    pub fn new(name: impl Into<String>, spline_step: u32, drw: &Drawing) -> Self {
        let scale_factor = Self::scale_factor(drw.header.default_drawing_units);
        let description = {
            let mut description: Description = (drw, spline_step).into();
            description.scale(scale_factor, scale_factor);
            description
        };

        let width = ((drw.header.maximum_drawing_extents.x - drw.header.minimum_drawing_extents.x)
            * scale_factor)
            .round();
        Definition {
            r#type: ItemType::Element,
            width,
            height: ((drw.header.maximum_drawing_extents.y - drw.header.minimum_drawing_extents.y)
                * scale_factor)
                .round(),
            //need to go look up in QET source, exactly how the hot spot is calculated, but at the moment this seems to be somewhat accrurate..probably no worse than
            //the hard coded, 5, 5
            hotspot_x: width,
            hotspot_y: 0.0,
            version: "0.8.0".into(),
            link_type: LinkType::Simple,
            uuid: Uuid::new_v4().into(),
            names: Names {
                names: vec![Name {
                    lang: "en".into(),
                    value: name.into(), //need to truncate the extension
                }],
            },
            element_infos: None,
            informations: Some("Created using dxf2elmt!".into()),
            //description: (drw, spline_step).into(),
            description,
        }
    }

    fn scale_factor(unit: Units) -> f64 {
        //I'm not entirely sure this is the best way to determine how to scale the images.
        //but I need to convert from real world units to pixels some how, and Assuming an A4 Paper
        //makes sense....Another option would be assume a 96dpi monitor...but 700px8.2677165354 inches,
        //actually gives 84.6666 dpi...so it's not far off. So for now unless I come up with something better
        //Below is how I will determine scaling based on the dxf.
        //If based on an A4 sheet of paper for the default QET Template, that's
        //8 rows at 80px high = 640px, plus a 60px title block, that's a height of 700px
        //an A4 sheet of paper is 210mm high in landscape mode -> 700px / 210mm = 3⅓ px/mm
        //so I should scale the values in the drawing by 3⅓ if the drawing is in mm
        //conversion table for other units is below

        //unit conversions taken from: https://www.unitconverters.net/length-converter.html

        700.0
            / match unit {
                //Units::Unitless => 700.0, //if the drawing is unitless just assume it's in pixels, so we want to return 1.0 from the funciton
                Units::Unitless => 7.291_666_666_666_667, //actually if it's unitless should I assume a conversion of 96dpi? ..so 700/7.291666666666667 = 96
                Units::Inches => 8.267_716_535_4, //8.2677165354 is the Height (in landscape) of an A4 sheet of paper in inches
                Units::Feet => 0.688_976_378, //0.688976378 is the Height (in landscape) of an A4 sheet of paper in feet
                Units::Miles => 0.000_130_488, //0.000130488 is the Height (in landscape) of an A4 sheet of paper in miles
                Units::Millimeters => 210.0, //210 is the Height (in landscape) of an A4 sheet of paper in mm
                Units::Centimeters => 21.0, //21 is the Height (in landscape) of an A4 sheet of paper in cm
                Units::Meters => 0.21, //0.21 is the Height (in landscape) of an A4 sheet of paper in m
                Units::Kilometers => 0.00021, //0.00021 is the Height (in landscape) of an A4 sheet of paper in km
                Units::Microinches => todo!(),
                Units::Mils => todo!(),
                Units::Yards => 0.229_658_792_7, //0.2296587927 is the Height (in landscape) of an A4 sheet of paper in yards
                Units::Angstroms => 2.1e9, //2.1e9 is the Height (in landscape) of an A4 sheet of paper in angstroms
                Units::Nanometers => 2.1e8, //2.1e8 is the Height (in landscape) of an A4 sheet of paper in nanometers
                Units::Microns => 210_000.0, //210000 is the Height (in landscape) of an A4 sheet of paper in micron / micrometer
                Units::Decimeters => 2.1, //2.1 is the Height (in landscape) of an A4 sheet of paper in decimeter
                Units::Decameters => 0.021, //0.021 is the Height (in landscape) of an A4 sheet of paper in decameter
                Units::Hectometers => 0.0021, //0.0021 is the Height (in landscape) of an A4 sheet of paper in hectometer
                Units::Gigameters => 2.1e-10, //2.1e-10 is the Height (in landscape) of an A4 sheet of paper in gigameters
                Units::AstronomicalUnits => 1.403763295e-12, //1.403763295E-12 is the Height (in landscape) of an A4 sheet of paper in AU
                Units::LightYears => 2.219701751e-17, //2.219701751E-17 is the Height (in landscape) of an A4 sheet of paper in lightyears
                Units::Parsecs => 6.805636508e-18, //6.805636508E-18 is the Height (in landscape) of an A4 sheet of paper in parsecs
                Units::USSurveyFeet => todo!(),
                Units::USSurveyInch => todo!(),
                Units::USSurveyYard => todo!(),
                Units::USSurveyMile => todo!(),
            }
    }
}

impl From<&Definition> for XMLElement {
    fn from(def: &Definition) -> Self {
        let mut def_xml = XMLElement::new("definition");
        def_xml.add_attribute("height", def.height);
        def_xml.add_attribute("width", def.width);
        def_xml.add_attribute("hotspot_x", def.hotspot_x);
        def_xml.add_attribute("hotspot_y", def.hotspot_y);
        def_xml.add_attribute("version", &def.version);
        def_xml.add_attribute("link_type", &def.link_type);
        def_xml.add_attribute("type", &def.r#type);
        def_xml.add_child((&def.uuid).into());

        def_xml.add_child((&def.names).into());

        let mut info_elmt = XMLElement::new("informations");
        info_elmt.add_text("Created using dxf2elmt!");
        def_xml.add_child(info_elmt);

        def_xml.add_child((&def.description).into());

        def_xml
    }
}

#[derive(Debug)]
enum Objects {
    Arc(Arc),
    Ellipse(Ellipse),
    Polygon(Polygon),
    DynamicText(DynamicText),
    Text(Text),
    Line(Line),
    //Terminal(Terminal),
    Block(Vec<Objects>),
}

impl ScaleEntity for Objects {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        match self {
            Objects::Arc(arc) => arc.scale(fact_x, fact_y),
            Objects::Ellipse(ellipse) => ellipse.scale(fact_x, fact_y),
            Objects::Polygon(polygon) => polygon.scale(fact_x, fact_y),
            Objects::DynamicText(dynamic_text) => dynamic_text.scale(fact_x, fact_y),
            Objects::Text(text) => text.scale(fact_x, fact_y),
            Objects::Line(line) => line.scale(fact_x, fact_y),
            Objects::Block(vec) => vec.iter_mut().for_each(|ob| ob.scale(fact_x, fact_y)),
        }
    }
}

impl TryFrom<(&Entity, u32, f64, f64)> for Objects {
    type Error = &'static str; //add better error later

    fn try_from(
        (ent, spline_step, offset_x, offset_y): (&Entity, u32, f64, f64),
    ) -> Result<Self, Self::Error> {
        match &ent.specific {
            EntityType::Circle(ref circle) => {
                let mut ellipse: Ellipse = circle.into();
                ellipse.x += offset_x;
                ellipse.y -= offset_y;
                Ok(Objects::Ellipse(ellipse))
            }
            EntityType::Line(ref line) => {
                let mut line: Line = line.into();
                line.x1 += offset_x;
                line.y1 -= offset_y;

                line.x2 += offset_x;
                line.y2 -= offset_y;

                Ok(Objects::Line(line))
            }
            EntityType::Arc(ref arc) => {
                let mut arc: Arc = arc.into();
                arc.x += offset_x;
                arc.y -= offset_y;

                Ok(Objects::Arc(arc))
            }
            EntityType::Spline(ref spline) => {
                let mut poly: Polygon = (spline, spline_step).into();

                match poly.coordinates.len() {
                    0 | 1 => Err("Error removing empty Spline"),
                    //I'll need to improve my understanding of splines and the math here
                    //to make sure I do this correclty.
                    //2 => //convert to line
                    _ => {
                        for cord in &mut poly.coordinates {
                            cord.x += offset_x;
                            cord.y -= offset_y;
                        }
                        Ok(Objects::Polygon(poly))
                    }
                }
            }
            EntityType::Text(ref text) => {
                Ok(
                    //right now the dxf2elmt defaults to making all text Static Text...
                    //it was requested by the QET devs to add in support for Dynamic text
                    //which was added, but it defaults to OFF, and QET doesn't pass the parameter
                    //to enable it...I'm wondering if it makes more sense to default to use dynamic text
                    //for now I'll set it to use dynamic text, and once I get the CLI flag passing through
                    //I might change the default parameter to use Dynamic Text
                    if false {
                        //how best to pass in the flag for dynamic text or not....should the flag also default to true?
                        let mut text: Text =
                            (text, HexColor::from_u32(ent.common.color_24_bit as u32)).into();
                        text.x += offset_x;
                        text.y -= offset_y;
                        Objects::Text(text)
                    } else {
                        let mut dtext: DynamicText =
                            (text, HexColor::from_u32(ent.common.color_24_bit as u32)).into();
                        dtext.x += offset_x;
                        dtext.y += offset_y;
                        Objects::DynamicText(dtext)
                    },
                )
            }
            EntityType::Ellipse(ref ellipse) => {
                let mut ellipse: Ellipse = ellipse.into();
                ellipse.x += offset_x;
                ellipse.y -= offset_y;
                Ok(Objects::Ellipse(ellipse))
            }
            EntityType::MText(ref mtext) => {
                Ok(
                    //right now the dxf2elmt defaults to making all text Static Text...
                    //it was requested by the QET devs to add in support for Dynamic text
                    //which was added, but it defaults to OFF, and QET doesn't pass the parameter
                    //to enable it...I'm wondering if it makes more sense to default to use dynamic text
                    //for now I'll set it to use dynamic text, and once I get the CLI flag passing through
                    //I might change the default parameter to use Dynamic Text
                    if false {
                        //how best to pass in the flag for dynamic text or not....should the flag also default to true?
                        /*let mut text: Text =
                            (mtext, HexColor::from_u32(ent.common.color_24_bit as u32)).into();
                            text.x += offset_x;
                            text.y -= offset_y;
                        Objects::Text(text)*/
                        todo!();
                    } else {
                        let mut dtext: DynamicText =
                            (mtext, HexColor::from_u32(ent.common.color_24_bit as u32)).into();
                        dtext.x += offset_x;
                        dtext.y += offset_y;
                        Objects::DynamicText(dtext)
                    },
                )
            }
            EntityType::Polyline(ref polyline) => match polyline.__vertices_and_handles.len() {
                0 | 1 => Err("Error empty Polyline"),
                2 => {
                    let mut line = Line::try_from(polyline)?;
                    line.x1 += offset_x;
                    line.y1 -= offset_y;

                    line.x2 += offset_x;
                    line.y2 -= offset_y;

                    Ok(Objects::Line(line))
                }
                _ => {
                    if let Ok(mut ellipse) = Ellipse::try_from(polyline) {
                        ellipse.x += offset_x;
                        ellipse.y -= offset_y;
                        Ok(Objects::Ellipse(ellipse))
                    } else {
                        let mut poly: Polygon = polyline.into();
                        for cord in &mut poly.coordinates {
                            cord.x += offset_x;
                            cord.y -= offset_y;
                        }
                        Ok(Objects::Polygon(poly))
                    }
                }
            },
            EntityType::LwPolyline(ref lwpolyline) => match lwpolyline.vertices.len() {
                0 | 1 => Err("Error empty LwPolyline"),
                2 => {
                    let mut line = Line::try_from(lwpolyline)?;
                    line.x1 += offset_x;
                    line.y1 -= offset_y;

                    line.x2 += offset_x;
                    line.y2 -= offset_y;

                    Ok(Objects::Line(line))
                }
                _ => {
                    if let Ok(mut ellipse) = Ellipse::try_from(lwpolyline) {
                        ellipse.x += offset_x;
                        ellipse.y -= offset_y;
                        Ok(Objects::Ellipse(ellipse))
                    } else {
                        let mut poly: Polygon = lwpolyline.into();
                        for cord in &mut poly.coordinates {
                            cord.x += offset_x;
                            cord.y -= offset_y;
                        }
                        Ok(Objects::Polygon(poly))
                    }
                }
            },
            EntityType::Solid(ref solid) => {
                let mut poly: Polygon = solid.into();

                for cord in &mut poly.coordinates {
                    cord.x += offset_x;
                    cord.y -= offset_y;
                }
                Ok(Objects::Polygon(poly))
            }
            _ => {
                //dbg!(&ent.specific);
                Err("Need to implement the rest of the entity types")
            }
        }
    }
}

impl TryFrom<(&Entity, u32)> for Objects {
    type Error = &'static str; //add better error later

    fn try_from((ent, spline_step): (&Entity, u32)) -> Result<Self, Self::Error> {
        Objects::try_from((ent, spline_step, 0f64, 0f64))
    }
}

impl From<&Objects> for Either<XMLElement, Vec<XMLElement>> {
    fn from(obj: &Objects) -> Self {
        match obj {
            Objects::Arc(arc) => Either::Left(arc.into()),
            Objects::Ellipse(ell) => Either::Left(ell.into()),
            Objects::Polygon(poly) => Either::Left(poly.into()),
            Objects::DynamicText(dtext) => Either::Left(dtext.into()),
            Objects::Text(txt) => Either::Left(txt.into()),
            Objects::Line(line) => Either::Left(line.into()),
            Objects::Block(block) => Either::Right(
                block
                    .iter()
                    .filter_map(|obj| XMLElement::try_from(obj).ok())
                    .collect(),
            ),
        }
    }
}

impl TryFrom<&Objects> for XMLElement {
    type Error = &'static str; // add better error later

    fn try_from(obj: &Objects) -> Result<Self, Self::Error> {
        match obj {
            Objects::Arc(arc) => Ok(arc.into()),
            Objects::Ellipse(ell) => Ok(ell.into()),
            Objects::Polygon(poly) => Ok(poly.into()),
            Objects::DynamicText(dtext) => Ok(dtext.into()),
            Objects::Text(txt) => Ok(txt.into()),
            Objects::Line(line) => Ok(line.into()),
            _ => Err("Unsupported"),
        }
    }
}

#[derive(Debug)]
pub struct Description {
    objects: Vec<Objects>,
}

impl ScaleEntity for Description {
    fn scale(&mut self, fact_x: f64, fact_y: f64) {
        self.objects
            .iter_mut()
            .for_each(|ob| ob.scale(fact_x, fact_y));
    }
}

impl From<&Description> for XMLElement {
    fn from(desc: &Description) -> Self {
        let mut desc_xml = XMLElement::new("description");
        for obj in &desc.objects {
            match obj.into() {
                Either::Left(elem) => desc_xml.add_child(elem),
                Either::Right(vec) => vec.into_iter().for_each(|elem| desc_xml.add_child(elem)),
            }
        }
        desc_xml
    }
}

/*impl TryFrom<Drawing> for Description {
    type Error = &'static str; //add better error later

    fn try_from(drw: Drawing) -> Result<Self, Self::Error> {
        drw.entities().filter_map(|ent| Objects::try_from(ent).ok()).collect();
    }
}*/
impl From<(&Drawing, u32)> for Description {
    fn from((drw, spline_step): (&Drawing, u32)) -> Self {
        /*println!("{:?}", drw.header.alternate_dimensioning_scale_factor);
        println!("{:?}", drw.header.alternate_dimensioning_units);
        println!("{:?}", drw.header.default_drawing_units);
        println!("{:?}", drw.header.drawing_units);
        println!("{}", drw.header.file_name);
        println!("{}", drw.header.project_name);
        println!("{:?}", drw.header.unit_format);*/

        Self {
            objects: drw
                .entities()
                .filter_map(|ent| {
                    match &ent.specific {
                        EntityType::Insert(ins) => {
                            //this is ugly there has to be a cleaner way to filter this....but for my first attempt at pulling the
                            //blocks out of the drawing it works.
                            //I mean would this ever return more than 1? I would assume block names have to be unique?
                            //but maybe not, the blocks have a handle, which is a u64. There is a get by handle function
                            //but not a get by name function....maybe the handle is what is unique and there can be duplicate names?
                            //a quick glance through the dxf code it looks like the handle might be given to the library user when inserting
                            //and entity? So I don't think there is any easy way to get the handle
                            let block = drw
                                .blocks()
                                .filter(|bl| bl.name == ins.name)
                                .take(1)
                                .next()
                                .unwrap();
                            let offset_x = ins.location.x;
                            let offset_y = ins.location.y;

                            Some(Objects::Block(
                                block
                                    .entities
                                    .iter()
                                    .filter_map(|ent| {
                                        Objects::try_from((ent, spline_step, offset_x, offset_y))
                                            .ok()
                                    })
                                    .collect(),
                            ))
                        }
                        _ => Objects::try_from((ent, spline_step)).ok(),
                    }
                })
                .collect(),
        }
    }
}

//probably don't need to worry about this as they won't exist in the dxf...
/*pub struct Terminal {
    x: f64,
    y: f64,
    uuid: Uuid,
    name: String,
    orientation: TermOrient,
    //type?
    //  Generic
    //  Indoor Terminal Block
    //  External Terminal Block
}*/

#[derive(Debug)]
pub struct Names {
    names: Vec<Name>,
}

impl From<&Names> for XMLElement {
    fn from(nme: &Names) -> Self {
        let mut names_elmt = XMLElement::new("names");
        for name in &nme.names {
            let mut name_elmt = XMLElement::new("name");
            name_elmt.add_attribute("lang", &name.lang);
            name_elmt.add_text(&name.value);
            names_elmt.add_child(name_elmt);
        }
        names_elmt
    }
}

#[derive(Debug)]
pub struct Name {
    lang: String, //should this be an enum of language shorts at some point, maybe not worth it
    value: String,
}

#[derive(Debug)]
pub struct ElmtUuid {
    //uuid: String,
    uuid: Uuid,
}

impl From<Uuid> for ElmtUuid {
    fn from(uuid: Uuid) -> Self {
        ElmtUuid { uuid }
    }
}

impl From<&ElmtUuid> for XMLElement {
    fn from(uuid: &ElmtUuid) -> Self {
        let mut uuid_xml = XMLElement::new("uuid");
        uuid_xml.add_attribute("uuid", format!("{{{}}}", uuid.uuid));
        uuid_xml
    }
}

#[derive(Debug)]
enum ItemType {
    Element = 1,
    ElementsCategory = 2,
    ElementsCollection = 4,
    ElementsContainer = 6,
    ElementsCollectionItem = 7,
    TitleBlockTemplate = 8,
    TitleBlockTemplatesCollection = 16,
    TitleBlockTemplatesCollectionItem = 24,
    Diagram = 32,
    Project = 64,
    All = 127,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Element => "element",
                Self::ElementsCategory | Self::ElementsContainer | Self::ElementsCollectionItem =>
                    "elements category",
                Self::ElementsCollection => "element",
                Self::TitleBlockTemplate | Self::TitleBlockTemplatesCollectionItem =>
                    "title block template",
                Self::TitleBlockTemplatesCollection => "title block templates collection",
                Self::Diagram => "diagram",
                Self::Project => "project",
                Self::All => "All",
            }
        )
    }
}

#[derive(Debug)]
enum HAlignment {
    Left,
    Center,
    Right,
}

impl Display for HAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "AlignLeft",
                Self::Center => "AlignHCenter",
                Self::Right => "AlignRight",
            }
        )
    }
}

#[derive(Debug)]
enum VAlignment {
    Top,
    Center,
    Bottom,
}

impl Display for VAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Top => "AlignTop",
                Self::Center => "AlignVCenter",
                Self::Bottom => "AlignBottom",
            }
        )
    }
}

#[derive(Debug)]
enum LineEnd {
    None,
    SimpleArrow,
    TriangleArrow,
    Circle,
    Diamond,
}

impl Display for LineEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::SimpleArrow => "simple",
                Self::TriangleArrow => "triangle",
                Self::Circle => "circle",
                Self::Diamond => "diamond",
            }
        )
    }
}

enum TermOrient {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum LinkType {
    Simple,
    Master,
    Slave,
    NextReport,
    PrevReport,
    TermBlock,
    Thumbnail,
}

impl Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Simple => "simple",
                Self::Master => "master",
                Self::Slave => "slave",
                Self::NextReport => "next_report",
                Self::PrevReport => "previous_report",
                Self::TermBlock => "terminal",
                Self::Thumbnail => "thumbnail",
            }
        )
    }
}

#[derive(Debug)]
pub struct ElemInfos {
    elem_info: Vec<ElemInfo>,
}

#[derive(Debug)]
pub struct ElemInfo {
    //there seems to be a list in the editor with the following values (per the XML)
    //  * supplier
    //  * description
    //  * machine_manufacturer_reference
    //  * manufacturer_reference
    //  * quantity
    //  * manufacturer
    //  * label
    //  * unity
    //  * plant
    //  * comment
    //  * designation
    // But can it only ever be these values? Might need to dig into the code. For now I'll use a string
    name: String,

    //I would assume show would be a bool...but instead of a true value I'm getting a "1"  in the XML
    //generated by the element editor. Maybe this means something else? I'll use an i32 for now
    show: i32,

    value: String,
}

#[inline]
pub fn two_dec(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}
