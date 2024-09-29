// TODO: the doctests in this are ignored, ideally they shouldn't be, to ensure they are kept up to date with the macros. I don't know if there's a way to make SQLx work in doctests.

use darling::FromDeriveInput;
use proc_macro2::{Span, TokenStream};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::punctuated::Pair;
use syn::spanned::Spanned;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, Ident, LitStr, Path, Result, Token};
use syn::{parse_quote, ExprLit, Lit};
use syn::{ExprPath, ItemEnum};

#[derive(FromDeriveInput)]
#[darling(attributes(lookup_table))]
struct LookupTableDeriveOpts {
    ident: Ident,
    table_name: LitStr,
}

struct LookupTableOpts {
    path: Path,
    table_name: LitStr,
}

fn import_my_crate() -> TokenStream {
    let found_crate = crate_name("database-utils").expect("database-utils is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident )
        }
    }
}

impl Parse for LookupTableOpts {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut input = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        let table_name = input.pop().map(Pair::into_value);
        let table_name = if let Some(Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str), ..
        })) = table_name
        {
            lit_str
        } else {
            panic!("bad arg")
        };
        let path = input.pop().map(Pair::into_value);
        let path = if let Some(Expr::Path(ExprPath { path, qself, .. })) = path {
            if qself.is_some() {
                panic!("qualified path shouldn't contain explicit Self type");
            }
            path
        } else {
            panic!("bad arg")
        };
        Ok(Self { path, table_name })
    }
}

/// Implements `LookupTable` for an enum. This enum's variants names (converted to snake case), should match the `value` column in the lookup table.
/// The rows in the lookup table should be inserted in the same order as the enum variants are specified, with sequential IDs, starting on 1.
/// This also implements the supertraits of LookupTable
///
/// ```ignore
/// #[lookup_table(table_name = "lu_some_table")]
/// pub enum SomeType {
///     A,
///     B,
///     C,
///     D,
/// }
/// ```
#[proc_macro_attribute]
pub fn lookup_table(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemEnum);
    let attr = TokenStream::from(attr);
    let ident = &item.ident;
    //   Err(input.span().error("there's a problem here..."))
    let explicit_discriminant_errors = item
        .variants
        .iter()
        .filter_map(|var| var.discriminant.as_ref())
        .map(|disc| {
            disc.1
                .span()
                .error("invalid explicit discriminant")
                .emit_as_item_tokens()
        })
        .fold(None, |acc: Option<TokenStream>, val| {
            let mut acc = match acc {
                Some(acc) => acc,
                None => return Some(val),
            };
            acc.extend(val);
            Some(acc)
        });
    if let Some(explicit_discriminant_errors) = explicit_discriminant_errors {
        return quote! {
            #item
            #explicit_discriminant_errors
        }
        .into();
    }
    // TODO: remove unwrap
    item.variants
        .first_mut()
        .expect("enum must have at least 1 variant")
        .discriminant = Some((Default::default(), parse_quote!(1)));

    let my_crate = import_my_crate();
    let try_from_i32_error_struct = quote!(#my_crate::NoVariantFori32);
    let output = quote! {
        #[derive(::sqlx::Type, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::marker::Copy, ::strum::EnumString, ::strum::EnumIter, ::strum::FromRepr, ::strum::IntoStaticStr, #my_crate::LookupTable)]
        #[lookup_table(#attr)]
        #[strum(serialize_all = "snake_case")]
        #[repr(i32)]
        #item
        impl ::std::convert::TryFrom<i32> for #ident {
            type Error = #try_from_i32_error_struct;

            fn try_from(value: i32) -> Result<Self, #try_from_i32_error_struct> {
                Self::from_repr(value).ok_or(#try_from_i32_error_struct(value))
            }
        }
        impl ::std::convert::From<#ident> for i32 {
            fn from(value: #ident) -> Self {
                value as i32
            }
        }
    };
    output.into()
}

/// Implements `LookupTable` for the given type
///
/// We should prefer the `#[lookup_table(table_name = "lu_some_table")]` attribute macro where possible, instead of this derive.
/// It uses this derive internally.
/// # Examples
///
/// ```ignore
/// #[derive(LookupTable)]
/// #[lookup_table(table_name = "lu_some_table")] // this looks the same as the `lookup_table` attribute macro, but here it's just an argument for the derive macro.
/// enum SomeType {
///     A = 1, // for most databases, the first automatically generated ID is `1`
///     B,
///     C
/// }
/// ```
#[proc_macro_derive(LookupTable, attributes(lookup_table))]
pub fn lookup_table_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let LookupTableDeriveOpts { ident, table_name } = FromDeriveInput::from_derive_input(&input).expect("Wrong options");
    let my_crate = import_my_crate();

    let output = quote! {
        #my_crate::impl_lookup_table!(#ident, #table_name);
    };

    output.into()
}

/// Implements `LookupTable` for the given type
///
/// This should be used for types where the attribute macro can't be used.
///
/// # Examples
///
/// ```ignore
/// impl_lookup_table!(some_crate::SomeType, "lu_some_table");
/// ```
#[proc_macro]
pub fn impl_lookup_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let LookupTableOpts { path, table_name } = parse_macro_input!(input as LookupTableOpts);
    let table_name = table_name.value();

    let fetch_all = format!(r#"SELECT "id", "value" FROM {table_name};"#);
    let fetch_by_id = format!(r#"SELECT "id", "value" FROM {table_name} WHERE "id" = $1;"#);
    let fetch_by_value = format!(r#"SELECT "id", "value" FROM {table_name} WHERE "value" = $1;"#);

    let my_crate = import_my_crate();
    let sqlx_driver = quote!(::sqlx::Postgres);
    let sqlx_driver_as_database = quote!(<#sqlx_driver as ::sqlx::Database>);
    let output = quote! {
        impl<'a> #my_crate::LookupTable<'a, #sqlx_driver> for #path {
            const TABLE_NAME: &'static str = #table_name;
            fn fetch_all_records() -> ::sqlx::query::Map<
                'a,
                #sqlx_driver,
                impl ::core::ops::FnMut(#sqlx_driver_as_database::Row) -> ::std::result::Result<#my_crate::LookupRecord, ::sqlx::Error>,
                #sqlx_driver_as_database::Arguments<'a>,
            > {
                ::sqlx::query_as!(#my_crate::LookupRecord, #fetch_all)
            }

            fn fetch_record_by_id(
                id: ::std::primitive::i32,
            ) -> ::sqlx::query::Map<
                'a,
                #sqlx_driver,
                impl ::core::ops::FnMut(#sqlx_driver_as_database::Row) -> ::std::result::Result<#my_crate::LookupRecord, ::sqlx::Error>,
                #sqlx_driver_as_database::Arguments<'a>,
            > {
                ::sqlx::query_as!(#my_crate::LookupRecord, #fetch_by_id, id)
            }

            fn fetch_record_by_value(
                value: &::std::primitive::str,
            ) -> ::sqlx::query::Map<
                'a,
                #sqlx_driver,
                impl ::core::ops::FnMut(#sqlx_driver_as_database::Row) -> ::std::result::Result<#my_crate::LookupRecord, ::sqlx::Error>,
                #sqlx_driver_as_database::Arguments<'a>,
            > {
                ::sqlx::query_as!(#my_crate::LookupRecord, #fetch_by_value, value)
            }
        }
    };

    output.into()
}
