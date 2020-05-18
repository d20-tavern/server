extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[proc_macro_attribute]
pub fn db_test(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let asyncness = &input.sig.asyncness;
    let vis = input.vis;

    let result = quote! {
        #[tokio::test]
        #[cfg(feature = "db-test")]
        #(#attrs)*
        #asyncness #vis fn #name() #ret {
            let conn = tavern_server::db::get_connection().await
                .expect("database reset failed");
            let mut tx = conn.begin().await
                .map_err(|err| Error::Transaction(err))
                .expect("database reset failed");
            sqlx::query(r"
                SELECT 'TRUNCATE ' || input_table_name || ' CASCADE;' AS truncate_query
                FROM(SELECT table_schema || '.' || table_name AS input_table_name
                FROM information_schema.tables WHERE table_schema
                NOT IN ('pg_catalog', 'information_schema') AND table_schema NOT LIKE 'pg_toast%')
                AS information; 
                ")
                .execute(&mut tx)
                .await
                .map_err(|err| Error::RunQuery(err))
                .expect("database reset failed");
            tx.commit().await
                .map_err(|err| Error::Transaction(err))
                .expect("database reset failed");

            tavern_server::db::init()
                .await
                .expect("database initialization failed");

            #body
        }
    };

    result.into()
}
