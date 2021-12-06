pub mod html_tags;

#[cfg(test)]
mod tests {
    use crate::html_tags::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn paragraph_tester() {
        let p = HTMLParagraph::new("Hello world".into());

        println!("{}", p.render());

        let mut table = HTMLTable::new();
        let mut row = HTMLRow::new();
        row.push(HTMLElement::Paragraph(p.clone()));
        row.push(HTMLElement::Paragraph(p.clone()));
        table.push(row.clone());
        table.push(row.clone());
        table.push(row.clone());

        println!("{}", table.render());
        assert_eq!("<p>Hello world</p>", p.render());
        assert_ne!("<p>HELLO WORLD</p>", p.render());
    }
}
