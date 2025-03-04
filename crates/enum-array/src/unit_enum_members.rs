use syn::spanned::Spanned;

pub struct UnitEnumMembers {
    type_name: syn::Ident,
    variant_names: UnitVariants,
}

struct UnitVariants {
    variant_names: Vec<syn::Ident>,
}

impl TryFrom<proc_macro2::TokenStream> for UnitEnumMembers {
    type Error = syn::Error;

    fn try_from(tokens: proc_macro2::TokenStream) -> Result<Self, Self::Error> {
        let syn::DeriveInput {
            ident: type_name,
            data,
            ..
        } = parse_enum(tokens)?;

        let syn::DataEnum { variants, .. } = extract_data_enum(data)?;

        Ok(Self {
            type_name,
            variant_names: variants.try_into()?,
        })
    }
}

impl From<UnitEnumMembers> for proc_macro2::TokenStream {
    fn from(
        UnitEnumMembers {
            type_name,
            variant_names: UnitVariants { variant_names },
        }: UnitEnumMembers,
    ) -> Self {
        let array_length = variant_names.len();

        quote::quote! {
            impl #type_name {
                pub const MEMBERS_COUNT: usize = #array_length;

                pub fn members() -> [Self; Self::MEMBERS_COUNT] {
                    [
                        #(Self::#variant_names),*
                    ]
                }
            }
        }
    }
}

impl TryFrom<syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>> for UnitVariants {
    type Error = syn::Error;

    fn try_from(
        variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    ) -> Result<Self, Self::Error> {
        let variants: Vec<syn::Ident> = variants
            .into_iter()
            .map(|variant| {
                if variant.fields == syn::Fields::Unit {
                    Ok(variant.ident)
                } else {
                    Err(syn::Error::new(
                        variant.span(),
                        "Only unit variants are supported",
                    ))
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            variant_names: variants,
        })
    }
}

fn parse_enum(tokens: proc_macro2::TokenStream) -> syn::Result<syn::DeriveInput> {
    let ast: syn::DeriveInput = syn::parse2(tokens)?;

    if !ast.generics.params.is_empty() {
        Err(syn::Error::new(
            ast.generics.span(),
            "Generic enums are not supported",
        ))
    } else {
        Ok(ast)
    }
}

fn extract_data_enum(ast_data: syn::Data) -> syn::Result<syn::DataEnum> {
    match ast_data {
        syn::Data::Enum(data_enum) => Ok(data_enum),
        syn::Data::Struct(syn::DataStruct {
            struct_token: syn::token::Struct { span },
            ..
        })
        | syn::Data::Union(syn::DataUnion {
            union_token: syn::token::Union { span },
            ..
        }) => Err(syn::Error::new(span, "Only Enums are supported")),
    }
}
