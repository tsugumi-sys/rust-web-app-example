mod models;
mod schema;

use self::models::{NewCompleteProduct, NewProduct, NewVariant, NewVariantValue, Product, Variant};
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use diesel::{ExpressionMethods, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_product(new_product: NewCompleteProduct, conn: &mut PgConnection) -> Result<i32> {
    use self::schema::products::dsl::products;
    use self::schema::products_variants::dsl::*;
    use self::schema::variants::dsl::*;

    conn.transaction(|conn| {
        let inserted_row = diesel::insert_into(products)
            .values(new_product.product)
            .get_result::<(i32, String, f64, bool)>(conn);

        let last_product_id: i32 = inserted_row.unwrap().0;

        for new_variant in new_product.variants {
            let variants_result = variants
                .filter(name.eq(&new_variant.variant.name))
                .limit(1)
                .load::<Variant>(conn)?;
            let last_variant_id: i32 = match variants_result.first() {
                Some(variant) => variant.id,
                None => {
                    let variant = diesel::insert_into(variants)
                        .values(new_variant.variant)
                        .get_result::<(i32, String)>(conn);
                    variant.unwrap().0
                }
            };

            for new_value in new_variant.values {
                diesel::insert_into(products_variants)
                    .values((
                        product_id.eq(last_product_id),
                        variant_id.eq(last_variant_id),
                        value.eq(new_value),
                    ))
                    .execute(conn)?;
            }
        }
        Ok(last_product_id)
    })
}

pub fn list_products(
    conn: &mut PgConnection,
) -> Result<Vec<(Product, Vec<(Option<String>, String)>)>> {
    use self::schema::products::dsl::products;
    let all_products = products.load::<Product>(conn)?;
    let mut res = Vec::new();
    for product in all_products {
        let product_vars = products_variants.filter(product_id.eq(&product.id));
    }
    Ok(all_products)
}

fn main() {
    println!("Hello, world!");
    establish_connection();
}

// test
#[test]
fn create_product_test() {
    let conn = &mut establish_connection();
    conn.test_transaction::<_, Error, _>(|conn| {
        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "boots".to_string(),
                    cost: 12.22,
                    active: true,
                },
                variants: vec![NewVariantValue {
                    variant: NewVariant {
                        name: "size".to_string(),
                    },
                    values: vec![Some(12.to_string()), Some(18.to_string())],
                }],
            },
            conn,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_string(&list_products(conn).unwrap()).unwrap(),
            serde_json::to_string(&vec![(
                Product {
                    id: 1,
                    name: "boots".to_string(),
                    cost: 12.22,
                    active: true,
                },
                vec![
                    (Some(12.to_string()), "size".to_string()),
                    (Some(18.to_string()), "size".to_string())
                ]
            )])
            .unwrap()
        );

        Ok(())
    })
}
