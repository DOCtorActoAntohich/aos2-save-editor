mod unit_enum_members;

use self::unit_enum_members::UnitEnumMembers;

#[proc_macro_derive(EnumMembersArray)]
pub fn enum_members_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match enum_members_derive_impl(tokens.into()) {
        Ok(stream) => stream.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn enum_members_derive_impl(
    tokens: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    UnitEnumMembers::try_from(tokens).map(Into::into)
}
