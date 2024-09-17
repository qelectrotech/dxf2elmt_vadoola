//use serde::{Deserialize, Serialize};
use uuid::Uuid;
//use std::str::FromStr;
//use strum::EnumString;
use dxf::entities::{Entity, EntityType};
use dxf::Drawing;
use hex_color::HexColor;
use std::convert::TryFrom;

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
}

enum Objects {
    Arc(Arc),
    Ellipse(Ellipse),
    Polygon(Polygon),
    DynamicText(DynamicText),
    Text(Text),
    Line(Line),
    Terminal(Terminal),
}

impl TryFrom<Entity> for Objects {
    type Error = &'static str; //add better error later

    fn try_from(ent: Entity) -> Result<Self, Self::Error> {
        match ent.specific {
            EntityType::Circle(ref circle) => Ok(Objects::Ellipse(circle.into())),
            EntityType::Line(ref line) => Ok(Objects::Line(line.into())),
            EntityType::Arc(ref arc) => Ok(Objects::Arc(arc.into())),
            EntityType::Spline(ref spline) => Ok(Objects::Polygon(
                (
                    spline,
                    100, /*need to passin spline value from cli, just hard codding value for now */
                )
                    .into(),
            )),
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
                        Objects::Text(
                            (text, HexColor::from_u32(ent.common.color_24_bit as u32)).into(),
                        )
                    } else {
                        Objects::DynamicText(
                            (text, HexColor::from_u32(ent.common.color_24_bit as u32)).into(),
                        )
                    },
                )
            }
            EntityType::Ellipse(ref ellipse) => Ok(Objects::Ellipse(ellipse.into())),
            EntityType::Polyline(ref polyline) => Ok(Objects::Polygon(polyline.into())),
            EntityType::LwPolyline(ref lwpolyline) => Ok(Objects::Polygon(lwpolyline.into())),
            EntityType::Solid(ref solid) => Ok(Objects::Polygon(solid.into())),
            _ => todo!("Need to implement the rest of the entity types"),
        }
    }
}

//Does it make sense to have all these seperate vectors?
//or a vec of enums of. With the enum being one of each type....I guess that might keep the ordering better?
//since they could be interleaved.
//it could have polygon, text, line, polygon. Right now they would get jumbled with all the seperate Vecs...
pub struct Description {
    objects: Vec<Objects>,
}

//probably don't need to worry about this as they won't exist in the dxf...
pub struct Terminal {
    x: f64,
    y: f64,
    uuid: Uuid,
    name: String,
    orientation: TermOrient,
    //type?
    //  Generic
    //  Indoor Terminal Block
    //  External Terminal Block
}

pub struct Names {
    names: Vec<Name>,
}

pub struct Name {
    lang: String, //should this be an enum of language shorts at some point, maybe not worth it
    value: String,
}

pub struct ElmtUuid {
    uuid: String,
}

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

enum HAlignment {
    Left,
    Center,
    Right,
}

enum VAlignment {
    Top,
    Center,
    Bottom,
}

enum LineEnd {
    None,
    SimpleArrow,
    TriangleArrow,
    Circle,
    Diamond,
}

enum TermOrient {
    North,
    East,
    South,
    West,
}

enum LinkType {
    Simple,
    Master,
    Slave,
    NextReport,
    PrevReport,
    TermBlock,
    Thumbnail,
}

pub struct ElemInfos {
    elem_info: Vec<ElemInfo>,
}

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
