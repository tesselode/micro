use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Component)]
pub fn derive_component(item: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(item as syn::DeriveInput);
	let ident = input.ident;
	let mut generics = input.generics;

	if generics.type_params().count() > 0 {
		/*
		if the component has generic type params, skip making it inspectable, but still
		implement `Component`
		*/
		for param in generics.type_params_mut() {
			param.bounds.push(syn::parse_quote!(::core::marker::Send));
			param.bounds.push(syn::parse_quote!(::core::marker::Sync));
			param.bounds.push(syn::parse_quote!('static));
		}
		let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
		quote! { impl #impl_generics ::micro_ecs::Component for #ident #ty_generics #where_clause {} }
			.into()
	} else {
		quote! {
			impl ::micro_ecs::Component for #ident {}

			::micro_ecs::inventory::submit! {
				::micro_ecs::InspectableComponent {
					name: stringify!(#ident),
					count: |world| world.query_mut::<()>().with::<&#ident>().into_iter().count(),
					exists: |entity_ref| entity_ref.get::<&#ident>().is_some(),
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
}
