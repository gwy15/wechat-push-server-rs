use super::errors::CallbackError;
use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_xml_string(s: String) -> Result<HashMap<String, String>, CallbackError> {
    let parser = EventReader::from_str(&s);
    let mut map = HashMap::new();
    let mut stack = Vec::new();
    // extract event
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                stack.push(name.local_name.clone());
            }
            Ok(XmlEvent::EndElement { .. }) => {
                if stack.len() == 1 {
                    break;
                }
                let value = stack.pop().ok_or(CallbackError::Xml)?;
                let key = stack.pop().ok_or(CallbackError::Xml)?;
                map.insert(key, value);
            }
            Ok(XmlEvent::CData(value)) | Ok(XmlEvent::Characters(value)) => {
                stack.push(value);
            }
            Ok(XmlEvent::Whitespace(_)) => {}
            Err(e) => {
                log::error!("Error: {}", e);
                return Err(CallbackError::Xml)?;
            }
            item => log::warn!("unknown item '{:?}', ignore", item),
        }
    }
    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xml_parse() {
        let s = r#"<xml>
                <ToUserName><![CDATA[toUser]]></ToUserName>
                <FromUserName><![CDATA[FromUser]]></FromUserName>
                <CreateTime>123456789</CreateTime>
                <MsgType><![CDATA[event]]></MsgType>
                <Event><![CDATA[subscribe]]></Event>
            </xml>"#
            .to_owned();
        let m = parse_xml_string(s).unwrap();
        assert_eq!(m["ToUserName"], "toUser");
        assert_eq!(m["FromUserName"], "FromUser");
        assert_eq!(m["CreateTime"], "123456789");
    }
}
