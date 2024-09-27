extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
/// Implements fluid_component with fluid parameters, generating all necessary boiler plate
pub fn build_fluid(_args: TokenStream, _input: TokenStream) -> TokenStream{
    let mut input = parse_macro_input!(_input as ItemStruct);
    let args = parse_macro_input!(_args with Attribute::parse_outer);

    let struct_name = input.ident.clone();
    
    
    let mut found_property_ident: syn::Result<Ident> = 
        syn::Result::Err(
            syn::Error::new(input.span(), "Could not find attribute 'build_fluid'")
        );
    //Scan through the args to find the one that belongs to this macro
    for arg in args {
        if let Some(ident) = arg.path().get_ident(){
            if ident.clone() == Ident::new("build_fluid", Span::call_site()){
                //found the arg that belongs to this macro
                found_property_ident = arg.parse_args();
            }
        }
    }

    //TODO Check fluid properties to see if its the correct type

    //Should get defined in next if-let block
    let mut output = syn::Result::Err(
        syn::Error::new(input.span(), "Never defined.")
    );

    if let Ok(fluid_properties) = found_property_ident {
        if let syn::Fields::Named(ref mut fields) = input.fields{
            // Everything seems to be in place
            fields.named.push(
                syn::Field::parse_named.parse(
                        quote!{ pub quantity: f32 }.into()
                    ).expect("Unable to parse hardcoded injection, this is a glitch in the library.")
            );

            output = Ok(
                quote!{
                    #input

                    impl FluidComponent for #struct_name{
                        fn get_fluid_properties(&self) -> &'static FluidProperties{
                            return &#fluid_properties;
                        }
                        fn get_quantity(&self) -> &f32{
                            return &self.quantity;
                        }
                    }
                }
            );
            
        }else{
            output = syn::Result::Err(
                syn::Error::new(input.span(), "The build_fluid macro can only be used on structs with named fields.")
            );
        }
    }else if let Err(property_ident_err) = found_property_ident {
        output = Err(property_ident_err);
    }

    

    
    
    

    match output {
        syn::Result::Ok(tt) => tt,
        syn::Result::Err(err) => err.to_compile_error(),
    }
    .into()
}
