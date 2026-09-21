use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, GenericArgument, Ident, parse_macro_input};

use syn::{PathArguments, Type, TypePath};

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let segment = path.segments.last();

            matches!(segment, Some(seg) if seg.ident == "Option")
        }
        _ => false,
    }
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;

    if segment.ident != "Option" {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    let GenericArgument::Type(inner_ty) = args.args.first()? else {
        return None;
    };

    Some(inner_ty)
}

fn prefix_ident(ident: Ident) -> Ident {
    Ident::new(&format!("macro_{ident}"), ident.span())
}

fn prefix_ident_custom(ident: Ident, prefix: &str) -> Ident {
    Ident::new(&format!("{prefix}{ident}"), ident.span())
}

#[proc_macro_derive(FromIter)]
pub fn derive_from_iter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data
        && let Fields::Named(ref fields) = data.fields
    {
        let options = fields.named.iter().map(|field| {
            let ty = field.ty.clone();
            let name = field.ident.clone().unwrap();

            let prefixed = prefix_ident(name);
            let value = if is_option(&ty) {
                quote!(Some(None))
            } else {
                quote!(None)
            };

            quote!(let mut #prefixed: Option<#ty> = #value;)
        });

        let fill_options = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();
            let name_string = name.to_string();
            let field_type = &field.ty;

            let prefixed = prefix_ident(name);

            let res_value = quote!(column_value.from_column::<#field_type>(column_type).map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?);

            // let tracing_fmt_string = format!("Set {}={{value:?}}", prefixed);

            quote!(
                String { .. } if matches!(Self::get_latest_column_name(#name_string), Some(col) if col == column_name) => {
                    let value = #res_value;
                    // tracing::trace!(#tracing_fmt_string);
                    #prefixed = Some(#res_value);
                })
        });

        let panic_strings = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();

            let prefixed = prefix_ident(name.clone());
            let prefixed_str = prefixed.to_string();
            let panic_string = format!(
                "expected to be able to unwrap field '{name}' with value {{{prefixed_str}:?}}"
            );

            let panic_var = prefix_ident_custom(name.clone(), "panic_");

            quote!(
                let #panic_var = format!(#panic_string);
            )
        });

        let construct_self = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();

            let prefixed = prefix_ident(name.clone());

            let panic_var = prefix_ident_custom(name.clone(), "panic_");

            quote!(
                #name: #prefixed.unwrap_or_else(|| panic!("{}", #panic_var))
            )
        });

        let name = input.ident;

        return quote!(
            impl dataloom::dataloom_db_core::traits::from_iter::FromIter for #name {
                fn from_iter(iter: impl Iterator<Item = dataloom::dataloom_db_core::traits::from_iter::FromIterValue>) -> Result<Self, dataloom::dataloom_db_core::traits::DatabaseStrategyError>
                where
                    Self: Sized,
                {
                    use dataloom::dataloom_db_core::{column::{FromColumn, ToColumn}, traits::{DatabaseStrategyError, model::Model}};
                    #(#options)*

                    for dataloom::dataloom_db_core::traits::from_iter::FromIterValue {
                        column_name,
                        column_value,
                        column_type,
                    } in iter {
                        match column_name {
                            #(#fill_options)*
                            _ => {}
                        }
                    }

                    #(#panic_strings)*

                    Ok(Self {
                        #(#construct_self),*
                    })
                }

            }
        )
        .into();
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive 'FromIter'",
        )
        .into_compile_error(),
    )
}

#[proc_macro_derive(SaveData)]
pub fn derive_save_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data
        && let Fields::Named(ref fields) = data.fields
    {
        let name = input.ident;

        let error_strings = fields.named.iter().map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_name_str = field_name.to_string();

            let panic_var_name = prefix_ident_custom(field_name.clone(), "panic_");
            let panic_var_value = format!(
                "expected for Self::get_latest_column_name for '{field_name_str}' to be Some."
            );

            quote!(
                let #panic_var_name = #panic_var_value;
            )
        });

        let save_models = fields.named.iter().map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_name_string = field_name.to_string();

            let panic_var_name = prefix_ident_custom(field_name.clone(), "panic_");

            quote!(dataloom::dataloom_db_core::save::SaveModel::new(
                Self::get_latest_column_name(#field_name_string).unwrap_or_else(|| panic!("{}", #panic_var_name)),
                self.#field_name.to_column().unwrap()
            ))
        });

        return quote!(
            impl dataloom::dataloom_db_core::traits::save_data::SaveData for #name {
                fn get_save_data(&self) -> Vec<dataloom::dataloom_db_core::save::SaveModel> {
                    use dataloom::dataloom_db_core::column::ToColumn;
                    use dataloom::dataloom_db_core::traits::model::Model;

                    #(#error_strings)*

                    vec![
                        #(#save_models),*
                    ]
                }

            }
        )
        .into();
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive 'FromIter'",
        )
        .into_compile_error(),
    )
}
