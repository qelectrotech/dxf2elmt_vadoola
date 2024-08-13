use dxf::entities::Text;
use dxf::entities::Entity;
use simple_xml_builder::XMLElement;
use uuid::Uuid;

pub fn add(text: &Text, e: &Entity, description: &mut XMLElement, text_count: &mut u32, dynamic_text: bool) {
    let mut text_xml: XMLElement = XMLElement::new("");

    if dynamic_text{
        text_xml = XMLElement::new("dynamic_text");

        text_xml.add_attribute("x", text.location.x);
        text_xml.add_attribute("y", -text.location.y);
        text_xml.add_attribute("z", text.location.z);
        if text.rotation.abs().round() as i64 % 360 != 0 {
            text_xml.add_attribute("rotation", text.rotation - 180.0);
        } else {
            text_xml.add_attribute("rotation", 0);
        }
        
        text_xml.add_attribute("uuid", format!("{{{}}}", Uuid::new_v4()));

        let mut tmp = &text.text_style_name[..];
        if tmp == "STANDARD" {
            tmp = "Arial Narrow";
        }
        text_xml.add_attribute(
            "font",
            format!(
                "{},{},-1,5,0,0,0,0,0,0,normal",
                tmp,
                text.text_height.ceil()
            ),
        );

        text_xml.add_attribute("Halignment", "AlignHCenter");
        text_xml.add_attribute("Valignment", "AlignVCenter");
        text_xml.add_attribute("text_from", "UserText");
        text_xml.add_attribute("frame", "false");
        text_xml.add_attribute("text_width", "-1");
        
        let mut text_field_xml = XMLElement::new("text");
        text_field_xml.add_text(&text.value[..]);

        text_xml.add_child(text_field_xml);

        let temp_color: String = format!("{:x}", e.common.color_24_bit);
        let mut text_color: String = String::new();
        let mut i: usize = temp_color.chars().count();
        text_color += "#";
        loop {
            if i >= 6 {
                break;
            }
            text_color += "0";
            i += 1;
        }
        text_color += &temp_color;
  
        let mut text_color_xml = XMLElement::new("color");
        text_color_xml.add_text(text_color);

        text_xml.add_child(text_color_xml);
   
    }

    if !dynamic_text{
        text_xml = XMLElement::new("text");
    
        text_xml.add_attribute("x", text.location.x);
        text_xml.add_attribute("y", -text.location.y);
        if text.rotation.abs().round() as i64 % 360 != 0 {
            text_xml.add_attribute("rotation", text.rotation - 180.0);
        } else {
            text_xml.add_attribute("rotation", 0);
        }
    
        let temp_color: String = format!("{:x}", e.common.color_24_bit);
        let mut text_color: String = String::new();
        let mut i: usize = temp_color.chars().count();
        text_color += "#";
        loop {
            if i >= 6 {
                break;
            }
            text_color += "0";
            i += 1;
        }
        text_color += &temp_color;
        text_xml.add_attribute("color", text_color);
    
        let mut tmp = &text.text_style_name[..];
        if tmp == "STANDARD" {
            tmp = "Arial Narrow";
        }
        text_xml.add_attribute("text", &text.value[..]);
        text_xml.add_attribute(
            "font",
            format!(
                "{},{},-1,5,0,0,0,0,0,0,normal",
                tmp,
                text.text_height.ceil()
            ),
        );
    }

    description.add_child(text_xml);
    *text_count += 1;
}
