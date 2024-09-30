use dxf::objects::Object;
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
    width: i32,
    height: i32,
    hotspot_x: i32,
    hotspot_y: i32,
    version: String,
    link_type: LinkType,
    uuid: ElmtUuid,
    names: Names,
    element_infos: Option<ElemInfos>,
    informations: Option<String>,
    description: Description,
    //counts
}

impl Definition {
    pub fn new(name: impl Into<String>, spline_step: u32, drw: &Drawing) -> Self {
        Definition {
            r#type: ItemType::Element,

            //The original code had the height, and width hard coded to 10
            //and the hotspots hard coded to 5. I'm not sure why this is?
            //Maybe actually calculating the size wasn't worth it? I'm not sure
            //if that info is part of the dxf. And maybe when you open he elemnt
            //in the elemtent editor it corrects it anyway. Just look into it, and
            //se if this is something that needs to get adjusted.
            width: 10,
            height: 10,
            hotspot_x: 5,
            hotspot_y: 5,
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
            description: (drw, spline_step).into(),
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

                for cord in &mut poly.coordinates {
                    cord.x += offset_x;
                    cord.y -= offset_y;
                }
                Ok(Objects::Polygon(poly))
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
            EntityType::Polyline(ref polyline) => {
                let mut poly: Polygon = polyline.into();
                for cord in &mut poly.coordinates {
                    cord.x += offset_x;
                    cord.y -= offset_y;
                }
                Ok(Objects::Polygon(poly))
            }
            EntityType::LwPolyline(ref lwpolyline) => {
                let mut poly: Polygon = lwpolyline.into();
                for cord in &mut poly.coordinates {
                    cord.x += offset_x;
                    cord.y -= offset_y;
                }
                Ok(Objects::Polygon(poly))
            }
            EntityType::Solid(ref solid) => {
                let mut poly: Polygon = solid.into();
                for cord in &mut poly.coordinates {
                    cord.x += offset_x;
                    cord.y -= offset_y;
                }
                Ok(Objects::Polygon(poly))
            }
            _ => {
                dbg!(&ent.specific);
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
