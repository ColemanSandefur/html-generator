extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Field;
use syn::{self, LitStr};

#[proc_macro_derive(HTMLRendering, attributes(rendered, rendered_iter))]
pub fn html_rendering_derive(item: TokenStream) -> TokenStream {
    let tree = syn::parse(item).unwrap();

    impl_html_rendering(&tree)
}

enum RenderType {
    Normal,
    Iter,
}

struct RenderedField {
    field: Field,
    render_type: RenderType,
    tag: String,
}

fn get_rendered_field(fields: &Punctuated<Field, Comma>) -> RenderedField {
    let mut output = None;
    let mut tag = String::new();
    for field in fields {
        for attribute in &field.attrs {
            let segment = match attribute.path.segments.first() {
                Some(segment) => segment,
                None => continue,
            };
            let identifier = segment.ident.to_string();

            let new_tag = match attribute.parse_args::<LitStr>() {
                Ok(token) => token.value(),
                Err(_) => String::new(),
            };
            let new = match identifier.as_str() {
                "rendered" => {
                    tag = new_tag;
                    Some((field.clone(), RenderType::Normal))
                }
                "rendered_iter" => {
                    tag = new_tag;
                    Some((field.clone(), RenderType::Iter))
                }
                _ => None,
            };

            if new.is_some() {
                match output.is_some() {
                    true => panic!("Only one field can be rendered"),
                    false => output = new,
                }
            }
        }
    }

    match output {
        Some(field) => RenderedField {
            field: field.0,
            render_type: field.1,
            tag,
        },
        None => panic!("One field must be labeled as #[rendered]"),
    }
}

fn impl_html_rendering(tree: &syn::DeriveInput) -> TokenStream {
    let name = &tree.ident;

    if let syn::Data::Struct(tree_struct) = &tree.data {
        if let syn::Fields::Named(fields) = &tree_struct.fields {
            let RenderedField {
                field: printed_field,
                render_type,
                tag,
            } = get_rendered_field(&fields.named);
            let identifier = printed_field.ident.unwrap();

            let str_lit = format!("<{}>{{}}</{}>", &tag, &tag);
            let str_lit = str_lit.as_str();

            let func = match render_type {
                RenderType::Normal => {
                    quote! {
                        format!(#str_lit, self.#identifier.render())
                    }
                }
                RenderType::Iter => {
                    quote! {
                        let mut output = format!("<{}>", #tag);
                        for item in &self.#identifier {
                            output.push_str(&item.render());
                        }
                        output.push_str(&format!("</{}>", #tag));
                        output
                    }
                }
            };

            return quote! {
                impl HTMLRendering for #name {
                    fn render(&self) -> String {
                        #func
                    }
                }
            }
            .into();
        } else {
            panic!("#[derive(HTMLRendering)] is only defined for structs with named attributes")
        }
    } else {
        panic!("#[derive(HTMLRendering)] is only defined for structs");
    }
}
