use quick_xml::{Reader, events::Event};

pub enum ReadContext {
    Readable(Option<Box<ReadContext>>),
    Unreadable(Option<Box<ReadContext>>),
}

impl ReadContext {
    pub fn root(self) -> Option<Box<ReadContext>> {
        match self {
            Self::Readable(root) => root,
            Self::Unreadable(root) => root,
        }
    }

    pub fn is_readable(&self) -> bool {
        match self {
            Self::Readable(_) => true,
            _ => false,
        }
    }
}

pub fn parse_document(document: &str) -> Vec<String> {
    let mut reader = Reader::from_str(document);
    reader.config_mut().trim_text(true);

    let mut buf = vec![];
    let mut texts = vec![];

    let mut read_cx = Box::new(ReadContext::Unreadable(None));

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("an error occured [:{}]: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"link" | b"head" | b"script" => {
                    read_cx = Box::new(ReadContext::Unreadable(Some(read_cx)))
                }
                b"div" | b"p" | b"h1" | b"h2" | b"a" => {
                    read_cx = Box::new(ReadContext::Readable(Some(read_cx)))
                }
                _ => read_cx = Box::new(ReadContext::Unreadable(Some(read_cx))),
            },

            Ok(Event::Text(e)) => {
                if read_cx.is_readable() {
                    texts.push(e.unescape().unwrap().into_owned());
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                _ => read_cx = read_cx.root().unwrap(),
            },
            _ => (),
        }
        buf.clear();
    }

    texts
}
