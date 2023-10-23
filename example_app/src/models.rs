use crate::schema::*;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = variants)]
pub struct Variant {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = variants)]
pub struct NewVariant {
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = products_variants)]
pub struct NewProductVariant {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>,
}

#[derive(Clone)]
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>,
}

pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>,
}
