use xml::reader::{EventReader, XmlEvent};
use syntect::easy::HighlightLines;


#[derive(Debug)]
enum State {
  NotInBlock,
  MaybeStartBlock,
  Block(String),  // TODO: should it be `String` or `&str`?
  Failure(String),
}

#[derive(Debug)]
enum Event {
  StartPre,
  StartCode,
  EndCode,
  Other(String),  // TODO: &str? Something else?
}


impl State {
  fn next(self, event: Event) -> State {
    use State::*;
    use Event::*;

    match (self, event) {
      (NotInBlock, StartPre) => MaybeStartBlock,
      (MaybeStartBlock, StartCode) =>
        Block(String::with_capacity(1000)),
      (Block(string), Other(contents)) =>
        Block(string + &contents),
      (Block(string), EndCode) => {
        // TODO: do something with the data here!
        State::NotInBlock
      },
      (s, e) => State::Failure(format!("Invalid state/event: {:?} | {:?}", s, e)),
    }
  }
}


pub fn syntax_highlight(html_string: String) -> String {
  let events = EventReader::from_str(&html_string).into_iter();
  let updated = events.map(|event| {
    match event {
      Ok(XmlEvent::StartElement { name, .. }) => {}
      Ok(XmlEvent::EndElement { name, .. }) => {}
      Err(_) => {
        println!(":wat: error parsing XML");
        {}
      },
      _ => {}
    }
  });
  String::new()  // TODO: actually build this, obviously
}
