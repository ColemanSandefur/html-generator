pub mod html_tags;

pub use html_tags::*;

#[cfg(test)]
mod tests {
    use crate::html_tags::*;
    use crate::prelude::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn paragraph_tester() {
        let mut p = HTMLParagraph::new(vec!["Hello world".into()]);

        p.push_element("!".into());

        assert_eq!("<p>Hello world!</p>", p.render());
        assert_ne!("<p>HELLO WORLD!</p>", p.render());
    }
}

pub mod prelude {
    pub trait HTMLManipulation<T> {
        //Define these 2 functions for auto implementations of the rest
        fn get_elements(&self) -> &Vec<T>;
        fn get_mut_elements(&mut self) -> &mut Vec<T>;

        // Auto implementations
        fn push_element(&mut self, el: T) {
            self.get_mut_elements().push(el);
        }

        fn pop_element(&mut self) -> Option<T> {
            self.get_mut_elements().pop()
        }

        fn remove_element(&mut self, index: usize) -> Option<T> {
            let elements = self.get_mut_elements();

            if elements.len() > index {
                return Some(elements.remove(index));
            }

            None
        }

        fn insert_element(&mut self, index: usize, el: T) {
            self.get_mut_elements().insert(index, el);
        }

        fn set_element(&mut self, index: usize, el: T) -> Option<T> {
            let old_el = self.remove_element(index);
            self.insert_element(index, el);

            old_el
        }

        fn set_elements(&mut self, elements: Vec<T>) {
            *self.get_mut_elements() = elements
        }
    }

    pub trait HTMLRendering {
        fn render(self: &Self) -> String;
        fn render_pretty(self: &Self) -> String {
            format!("{}\n", self.render())
        }
    }

    pub trait HTMLRenderingClonable: HTMLRendering {
        fn clone_html(self: &Self) -> Box<dyn HTMLRenderingClonable>;
    }

    impl Clone for Box<dyn HTMLRenderingClonable> {
        fn clone(&self) -> Self {
            self.clone_html()
        }
    }

    pub trait HTMLCustom: Clone {
        fn render(self: &Self) -> String;
    }

    impl HTMLRendering for String {
        fn render(&self) -> String {
            self.clone()
        }
    }
}
