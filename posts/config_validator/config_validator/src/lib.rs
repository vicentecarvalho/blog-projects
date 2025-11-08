use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    self, Attribute, Data, DeriveInput, Fields, Ident, Type, TypePath,
};

/// Verify if field is mandatory
///
/// ### Param
///
/// * `field`: The field holding the attribute
///
/// ### Returns
/// true if field is mandatory
fn is_field_mandatory(attrs: &Vec<Attribute>) -> bool {
    verify_attribute_token(attrs, "validate", "mandatory")
}

/// Verify if attribute key value is present in list
/// of attributes
///
/// ### Param
///
/// * `attrs`: Attribute list
///
/// ### Returns
/// true if present
fn verify_attribute_token(
    attrs: &Vec<Attribute>,
    key: &str,
    value: &str,
) -> bool {
    for attr in attrs {
        if !attr.path().is_ident(key) {
            continue;
        }
        if attr
            .parse_nested_meta(|meta| match meta.path.is_ident(value) {
                true => Ok(()),
                false => Err(meta.error("unrecognized repr")),
            })
            .is_ok()
        {
            return true;
        }
    }
    false
}

/// Proc macro implementation to validate mandatory
/// fields in a struct.
///
/// ### Requirements
///
/// - The original struct must implement traits Clone and Debug.
///
///
/// ### Example
///
/// ```rust
/// use config_validator::Validate;
/// use serde::Serialize;
///
/// #[derive(Debug, Clone, Default, Validate, Serialize)]
/// pub struct Settings {
///     /// config file path
///     #[validate(mandatory)]
///     pub config_file: Option<String>,
///     /// name
///     pub name: Option<String>,
/// }
/// let s: Settings = Settings { config_file: Some(String::from("path/to/file")), ..Default::default()};
/// let s: ValidatedSettings = s.validate_configuration(); // will panic if the mandatory fields are not initialized
/// let file: String = s.get_config_file();
/// ```
#[proc_macro_derive(Validate, attributes(validate))]
pub fn validator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let ts = impl_validator(&ast);
    ts.into()
}

/// Helper function to generate a getter method for non-mandatory field
fn generate_getter(
    field_name: &syn::Ident,
    field_type: &syn::Type,
    method_impl: &mut Vec<TokenStream>,
) {
    let method_name = quote::format_ident!("get_{}", field_name);
    method_impl.push(quote! {
        pub fn #method_name(&self) -> #field_type {
            self.0.#field_name.clone()
        }
    });
}

/// Proc macro implementation
fn impl_validator(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields_data = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                panic!("Validate can only be derived on struct named fields.")
            }
        },
        _ => panic!("Validate can only be derived on Structs."),
    };

    // define derive traits for the Validate struct
    let traits = vec![
        syn::Path::from(syn::Ident::new("Clone", struct_name.span())),
        syn::Path::from(syn::Ident::new("Debug", struct_name.span())),
        syn::Path::from(syn::Ident::new("Serialize", struct_name.span())),
    ];

    let struct_validate_name =
        Ident::new(&format!("Validated{struct_name}"), struct_name.span());
    let mut validate_field_block: Vec<TokenStream> = vec![];
    let mut method_impl: Vec<TokenStream> = vec![];

    for field in fields_data {
        let field_name = &field.ident.clone().unwrap();
        let field_type = &field.ty;

        let path = match field_type {
            Type::Path(TypePath { path, .. }) => path,
            _ => {
                generate_getter(field_name, field_type, &mut method_impl);
                continue;
            }
        };

        if path.segments[0].ident != "Option"
            || !is_field_mandatory(&field.attrs)
        {
            generate_getter(field_name, field_type, &mut method_impl);
            continue;
        }

        // extract the type inside Option<T>
        let args = match &path.segments[0].arguments {
            syn::PathArguments::AngleBracketed(args) => args,
            _ => {
                generate_getter(field_name, field_type, &mut method_impl);
                continue;
            }
        };

        let inner_type = match &args.args[0] {
            syn::GenericArgument::Type(inner_type) => inner_type,
            _ => {
                generate_getter(field_name, field_type, &mut method_impl);
                continue;
            }
        };

        // generate validate block for mandatory Option fields
        validate_field_block.push(quote! {
            if self.#field_name.is_none() {
                panic!("[config-validator] mandatory field \"{}\" not provided for struct \"{}\"",
                       stringify!(#field_name), stringify!(#struct_name));
            }
        });

        // generate getter method for mandatory Option fields
        let method_name = quote::format_ident!("get_{}", field_name);
        method_impl.push(quote! {
            pub fn #method_name(&self) -> #inner_type {
                self.0.#field_name.clone().unwrap()
            }
        });
    }

    quote! {
        impl #struct_name {
            pub fn validate_configuration(&self) -> #struct_validate_name {
                #(#validate_field_block)*
                #struct_validate_name(self.clone())

            }
        }

        #[derive(#(#traits),*)]
        pub struct #struct_validate_name(#struct_name);

        impl #struct_validate_name {
            #(#method_impl)*
        }
    }
}
