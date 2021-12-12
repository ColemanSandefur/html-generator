mod body;
mod heading;
mod paragraph;
mod row;
mod table;
mod table_data;
mod table_header;
mod time;

pub use body::*;
pub use heading::*;
pub use paragraph::*;
pub use row::*;
pub use table::*;
pub use table_data::*;
pub use table_header::*;
pub use time::*;

use crate::prelude::*;

#[derive(Clone)]
pub enum HTMLElement {
    Body(HTMLBody),
    Heading(HTMLHeading),
    Row(HTMLRow),
    Paragraph(HTMLParagraph),
    Simple(String),
    Table(HTMLTable),
    TableData(HTMLTableData),
    TableHeader(HTMLTableHeader),
    Time(HTMLTime),
    Custom(Box<dyn HTMLRenderingClonable>),
}

impl HTMLRendering for HTMLElement {
    fn render(&self) -> String {
        match self {
            HTMLElement::Body(b) => b.render(),
            HTMLElement::Simple(s) => s.render(),
            HTMLElement::Table(table) => table.render(),
            HTMLElement::Row(row) => row.render(),
            HTMLElement::TableData(d) => d.render(),
            HTMLElement::Paragraph(p) => p.render(),
            HTMLElement::Heading(h) => h.render(),
            HTMLElement::TableHeader(t) => t.render(),
            HTMLElement::Time(t) => t.render(),
            HTMLElement::Custom(c) => c.render(), // A pretty bad work around, but it works enough?
        }
    }
}

impl ToHTML for String {
    fn to_html(self) -> HTMLElement {
        HTMLElement::Simple(self)
    }
}
impl ToHTML for &String {
    fn to_html(self) -> HTMLElement {
        HTMLElement::Simple(self.to_string())
    }
}
impl ToHTML for &str {
    fn to_html(self) -> HTMLElement {
        HTMLElement::Simple(self.into())
    }
}

impl<T: ToString> From<T> for HTMLElement {
    fn from(s: T) -> HTMLElement {
        HTMLElement::Simple(s.to_string())
    }
}
