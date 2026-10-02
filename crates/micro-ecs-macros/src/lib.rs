use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Component)]
pub fn derive_component(item: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(item as syn::DeriveInput);
	let ident = input.ident;
	quote! {
		impl micro_ecs::Component for #ident {}

		micro_ecs::inventory::submit! {
			micro_ecs::InspectableComponent {
				name: stringify!(#ident),
				inspect: |ui, entity_ref| {
					if let Some(component) = entity_ref.get::<&#ident>() {
						ui.collapsing(stringify!(#ident), |ui| {
							ui.monospace(format!("{:#?}", component));
						});
					}
				},
			}
		}
	}
	.into()
}
