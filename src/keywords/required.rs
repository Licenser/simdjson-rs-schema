use simd_json::{BorrowedValue as Value, Value as ValueTrait};

use super::schema;
use super::validators;

pub struct Required;
impl<'key, V> super::Keyword<'key, V> for Required
where
    V: ValueTrait,
    <V as ValueTrait>::Key: std::borrow::Borrow<String> + std::hash::Hash + Eq,
{
    fn compile(&self, def: &Value, ctx: &schema::WalkContext<'_>) -> super::KeywordResult<V> {
        let required = keyword_key_exists!(def, "required");

        if required.is_array() {
            let required = required.as_array().unwrap();

            let mut items = vec![];
            for item in required.iter() {
                if item.is_str() {
                    items.push(item.to_string());
                } else {
                    return Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "The values of `required` must be string".to_string(),
                    });
                }
            }

            Ok(Some(Box::new(validators::Required { items})))
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of this keyword must be an array.".to_string(),
            })
        }
    }
}
