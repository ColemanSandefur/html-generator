use crate::html_tags::HTMLElement;
use crate::prelude::*;
use chrono::DateTime;
use chrono::TimeZone;
use chrono_tz::Tz;

#[derive(Clone)]
pub struct HTMLTime {
    element: DateTime<Tz>,
    time_zone: Tz,
    format: String,
}

impl HTMLTime {
    pub fn new<T: TimeZone>(time: DateTime<T>, time_zone: Tz, format: String) -> Self {
        let element = time.with_timezone(&time_zone);

        Self {
            element,
            time_zone,
            format,
        }
    }
}

impl HTMLRendering for HTMLTime {
    fn render(self: &Self) -> String {
        let mut output = String::from("<time>");

        let format = self.element.format(&self.format);
        output.push_str(&format!("{}", format));

        output.push_str("</time>");

        output
    }
}

impl From<HTMLTime> for HTMLElement {
    fn from(t: HTMLTime) -> Self {
        HTMLElement::Time(t)
    }
}
