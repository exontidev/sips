use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(ix_data))]
struct InstructionData {
    discriminator: Vec<u8>,
    accounts: Option<syn::Type>,
}

fn derive_instruction2(
    input: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(input)?;

    let ix_data: InstructionData = deluxe::extract_attributes(&mut ast)?;

    let accounts = ix_data.accounts;
    let discriminator = ix_data.discriminator;

    let name = ast.ident;

    let bytes = discriminator.iter();

    let disc = quote! {
        impl Instruction for #name {
            const DISCRIMINATOR: &'static [u8] = &[#(#bytes),*];
        }
    };

    let accs = match accounts {
        Some(accounts) => {
            quote! {
                impl InstructionAccounts for #name {
                    type Account = #accounts
                }
            }
        }

        _ => quote! {},
    };

    Ok(quote! {
        #disc,
        #accs
    })
}

#[proc_macro_derive(Instruction, attributes(ix_data))]
pub fn derive_instruction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_instruction2(input.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_derive(Instructions)]
pub fn derive_instruction_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(ref data_enum) => &data_enum.variants,
        _ => panic!("InstructionSet can only be derived for enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let inner_type = match &variant.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => &f.unnamed[0].ty,
            _ => panic!("Variants must be tuple variants with exactly one inner type, e.g. Create(PumpCreateInstruction)"),
        };

        quote! {
            if data.starts_with(<#inner_type as Instruction>::DISCRIMINATOR) {
                return <#inner_type as Instruction>::from_bytes(data)
                    .map(#name::#variant_ident);
            }
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn try_from_bytes(data: &[u8]) -> Result<Self, Error> {
                #(#match_arms)*

                Err(Error::InvalidDiscriminator)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Accounts, attributes(signer, writable))]
pub fn derive_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Accounts derive can only be used on structs with named fields"),
        },
        _ => panic!("Accounts derive can only be used on structs"),
    };

    let count = fields.len();

    let meta_entries = fields.iter().map(|f| {
        let field_ident = &f.ident;

        let is_signer = f.attrs.iter().any(|attr| attr.path().is_ident("signer"));
        let is_writable = f.attrs.iter().any(|attr| attr.path().is_ident("writable"));

        quote! {
            AccountMeta {
                pubkey: &self.#field_ident,
                is_signer: #is_signer,
                writable: #is_writable,
            }
        }
    });

    let expanded = quote! {
        impl<'a> IntoAccountMetaArray<'a, #count> for #name {
            fn accounts_meta(&'a self) -> [AccountMeta<'a>; #count] {
                [
                    #(#meta_entries),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
