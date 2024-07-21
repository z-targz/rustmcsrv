use std::path::Path;

use convert_case::{Case, Casing};


use proc_macro::TokenStream;



/*
lazy_static!{
    static ref TRAIT_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
*/



#[proc_macro]
pub fn create_entity_macros(_: TokenStream) -> TokenStream {
    
    let mut out = String::new();
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let files = std::fs::read_dir(
        Path::new(cargo_manifest_dir.as_str()).parent().unwrap()
        .join("src")
        .join("entity")
        .join("tags")).unwrap();
    

    files.into_iter()
        .map(|file| {
            let tag_name_snake = 
                file.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".rs").to_string();
            let tag_name = tag_name_snake.to_case(Case::Pascal);
            (tag_name, tag_name_snake)
            })
        .for_each(|(tag_name, tag_name_snake)|
        {
            out +=
                format!("
                    #[proc_macro_derive({tag_name})]
                    pub fn derive_{tag_name_snake}(input: TokenStream) -> TokenStream {{
                        let ast = syn::parse(input).unwrap();
                        entity::register_entity_tag(ast, \"{tag_name}\");
                        quote!{{}}.into()
                    }}").as_str();
        });
    let out: proc_macro2::TokenStream = out.parse().unwrap();
    out.into()
}

/*#[proc_macro]
pub fn create_entity_attribute_macros(_: TokenStream) -> TokenStream {

}*/