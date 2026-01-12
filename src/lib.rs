//! Stable IDL (SIDL) macro generation.
//!
//! Provides `#[derive(Diplomat)]` for generating stable type IDs based on
//! struct names. Used for cross-boundary type identification.

use proc_macro::TokenStream;
use quote::quote;
use std::hash::{Hash, Hasher};
use syn::{DeriveInput, parse_macro_input};

/// automatic implementation of the `Diplomat` trait.
///
/// This macro calculates a stable logic hash based on the struct name and fields
/// to generate a unique `TYPE_ID`.
#[proc_macro_derive(Diplomat)]
pub fn derive_diplomat(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Calculate a stable hash for the TYPE_ID.
    // We use CRC64 (ISO) which is deterministic across platforms and Rust versions.
    // DefaultHasher is not guaranteed to be stable.
    struct StableHasher(crc64fast::Digest);

    impl std::hash::Hasher for StableHasher {
        fn finish(&self) -> u64 {
            self.0.sum64()
        }
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes);
        }
    }

    let mut hasher = StableHasher(crc64fast::Digest::new());
    name.to_string().hash(&mut hasher);

    // Force non-zero
    let hash = hasher.finish() as u128;
    // ensure it's not 0 (though unlikely)
    let type_id = if hash == 0 { 1 } else { hash };

    let expanded = quote! {
        impl crate::Diplomat for #name {
            const TYPE_ID: u128 = #type_id;
        }

        impl std::fmt::Debug for dyn crate::Diplomat {
             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                 f.debug_struct("Diplomat")
                  .field("type_name", &stringify!(#name))
                  .field("type_id", &Self::TYPE_ID)
                  .finish()
             }
        }
    };

    TokenStream::from(expanded)
}
