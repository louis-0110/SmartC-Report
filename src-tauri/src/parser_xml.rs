use quick_xml::{events::Event, Reader};
pub fn parser(value: &str) -> Vec<String> {
    let mut reader = Reader::from_str(value);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut cur_tag_name = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                cur_tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string()
            }
            Ok(Event::Text(e)) => {
                if cur_tag_name == "msg" {
                    txt.push(e.unescape().unwrap().into_owned())
                }
            }
            _ => (),
        }
        buf.clear();
    }
    println!("{:?}", txt);
    txt
}
