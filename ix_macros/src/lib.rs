use proc_macro::TokenStream;
use quote::{format_ident, quote};
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
    let payload_name = format_ident!("{}Payload", name);

    let variants = match input.data {
        Data::Enum(ref data_enum) => &data_enum.variants,
        _ => panic!("Accounts derive can only be used on enums"),
    };

    let count = variants.len();

    let variant_details: Vec<_> = variants
        .iter()
        .map(|v| {
            let var_ident = &v.ident;
            let field_ident = format_ident!("{}", to_snake_case(&var_ident.to_string()));
            
            let is_signer = v.attrs.iter().any(|attr| attr.path().is_ident("signer"));
            let is_writable = v.attrs.iter().any(|attr| attr.path().is_ident("writable"));

            (field_ident, is_signer, is_writable)
        })
        .collect();

    // Prepare code fragments
    let struct_fields = variant_details.iter().map(|(field, _, _)| {
        quote! { pub #field: RawPubkey }
    });

    let new_params = variant_details.iter().map(|(field, _, _)| {
        quote! { #field: RawPubkey }
    });

    let new_init = variant_details.iter().map(|(field, _, _)| {
        quote! { #field }
    });

    let meta_entries = variant_details.iter().map(|(field, signer, writable)| {
        quote! {
            AccountMeta {
                pubkey: self.#field,
                is_signer: #signer,
                writeble: #writable,
            }
        }
    });

    let expanded = quote! {
        impl Accounts for #name {
            const ACCOUNT_LENGTH: usize = #count;
            
            #[inline(always)]
            fn index(self) -> usize {
                self as usize
            }
        }

        pub struct #payload_name {
            #(#struct_fields,)*
        }
        
        impl #payload_name {
            
            #[inline(always)]
            pub fn new(
                #(#new_params,)*
            ) -> Self {
                Self {
                    #(#new_init,)*
                }
            }
        }

        impl Into<[AccountMeta; #name::ACCOUNT_LENGTH]> for #payload_name {
            fn into(self) -> [AccountMeta; #name::ACCOUNT_LENGTH] {
                [
                    #(#meta_entries,)*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}

fn to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}