use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(ix_data))]
struct InstructionData {
    discriminator : Vec<u8>,
    accounts : Option<syn::Type>
}

fn derive_instruction2(input: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast : DeriveInput = syn::parse2(input)?;

    let ix_data : InstructionData = deluxe::extract_attributes(&mut ast)?;
    
    let accounts = ix_data.accounts;
    let discriminator = ix_data.discriminator;

    let name = ast.ident;

    let bytes = discriminator.iter();

    let disc = quote! { 
        impl RawSerializable for #name {
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
        },

        _ => quote! {}
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
            if data.starts_with(<#inner_type as RawSerializable>::DISCRIMINATOR) {
                return <#inner_type as RawSerializable>::from_bytes(data)
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

#[proc_macro_derive(Accounts)]
pub fn derive_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl AccountIndex for #name {
            fn index(self) -> usize {
                self as usize
            }
        }
    }.into()
}