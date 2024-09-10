use std::{
    fmt::Debug,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;

/*
lazy_static!{
    static ref TRAIT_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
*/

pub(crate) enum EntityMacroError {
    IOError(std::io::Error),
    SynError(syn::Error),
    Other(String),
}

impl std::error::Error for EntityMacroError {}

impl std::fmt::Display for EntityMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(io_error) => std::fmt::Display::fmt(io_error, f),
            Self::SynError(syn_error) => std::fmt::Display::fmt(syn_error, f),
            Self::Other(error_msg) => f.write_str(error_msg.as_str()),
        }
    }
}

impl std::fmt::Debug for EntityMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(io_error) => Debug::fmt(io_error, f),
            Self::SynError(syn_error) => Debug::fmt(syn_error, f),
            Self::Other(error_msg) => f.write_str(error_msg.as_str()),
        }
    }
}

impl From<std::io::Error> for EntityMacroError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<syn::Error> for EntityMacroError {
    fn from(value: syn::Error) -> Self {
        Self::SynError(value)
    }
}

impl From<String> for EntityMacroError {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

#[proc_macro]
pub fn create_entity_macros(_: TokenStream) -> TokenStream {
    let mut out = String::new();
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let files = std::fs::read_dir(
        Path::new(cargo_manifest_dir.as_str())
            .parent()
            .unwrap()
            .join("src")
            .join("entity")
            .join("entities"),
    )
    .unwrap();

    let result = files
        .into_iter()
        .map(|result| {
            match result {
                Ok(dir_entry) => {
                    let module_path = dir_entry
                        .path()
                        .iter()
                        .map(|segment| segment.to_str().unwrap().trim_end_matches(".rs").to_owned())
                        .collect::<Vec<_>>()
                        .join("::");

                    let mut file_contents = String::new();
                    File::open(dir_entry.path())?.read_to_string(&mut file_contents)?;
                    let file_src = syn::parse_file(&file_contents.as_str())?;

                    struct FileVisitor {
                        pub tag_names: Vec<String>,
                    }
                    impl FileVisitor {
                        pub fn new(_file_path: PathBuf) -> Self {
                            Self {
                                tag_names: Vec::new(),
                            }
                        }
                    }
                    impl<'ast> Visit<'ast> for FileVisitor {
                        fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
                            let mut to_add: bool = false;
                            for attr in &item.attrs {
                                match attr.path().get_ident() {
                                    Some(ident) => {
                                        if ident.to_string().as_str() == "entity_tag" {
                                            to_add = true;
                                        }
                                    }
                                    None => (),
                                }
                            }
                            if to_add {
                                self.tag_names.push(item.ident.to_string());
                            }
                        }
                    }
                    let mut file_visitor = FileVisitor::new(dir_entry.path());
                    file_visitor.visit_file(&file_src);

                    //let tag_name = module_name.to_case(Case::Pascal);
                    Ok((file_visitor.tag_names, module_path))
                }
                Err(e) => Err(EntityMacroError::IOError(e)),
            }
        })
        .map(|result| match result {
            Ok((tag_names, module_path)) => {
                tag_names.into_iter().for_each(|tag_name| {
                    let module_name = module_path.split("::").last().unwrap();
                    out += format!(
                        "
                            #[proc_macro_derive({tag_name})]
                            pub fn derive_{module_name}(input: TokenStream) -> TokenStream {{
                                let ast = syn::parse(input).unwrap();
                                entity::register_entity_tag(ast, \"{tag_name}\", \"{module_path}\");
                                quote!{{}}.into()
                            }}"
                    )
                    .as_str();
                });
                Ok(())
            }
            Err(e) => Err(e),
        })
        .collect::<Result<(), EntityMacroError>>();
    match result {
        Ok(_) => {
            let out: proc_macro2::TokenStream = out.parse().unwrap();
            out.into()
        }
        Err(e) => {
            let error_string = e.to_string();
            quote! {compile_error!{#error_string}}.into()
        }
    }
}

/*#[proc_macro]
pub fn create_entity_attribute_macros(_: TokenStream) -> TokenStream {

}*/
