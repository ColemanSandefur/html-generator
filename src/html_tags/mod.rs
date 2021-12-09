mod body;
mod heading;
mod paragraph;
mod row;
mod table;
mod table_data;

pub use body::*;
pub use heading::*;
pub use paragraph::*;
pub use row::*;
pub use table::*;
pub use table_data::*;

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
            HTMLElement::Custom(c) => c.render(), // A pretty bad work around, but it works enough?
        }
    }
}

impl<T: ToString> From<T> for HTMLElement {
    fn from(s: T) -> HTMLElement {
        HTMLElement::Simple(s.to_string())
    }
}
