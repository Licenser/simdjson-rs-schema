use simd_json::Value as ValueTrait;

use super::schema;
use super::validators;

macro_rules! kw_minmax {
    ($name:ident, $keyword:expr) => {
        #[allow(missing_copy_implementations)]
        pub struct $name;
        impl<V> super::Keyword<V> for $name
        where
            V: ValueTrait + std::clone::Clone + std::convert::From<simd_json::value::owned::Value> + std::fmt::Display + 'static,
            <V as ValueTrait>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str> + std::fmt::Debug + std::string::ToString + std::marker::Sync + std::marker::Send,
        {
            fn compile(&self, def: &V, ctx: &schema::WalkContext<'_>) -> super::KeywordResult<V> {
                let value = keyword_key_exists!(def, $keyword);

                if value.is_f64() {
                    let value = value.as_f64().unwrap();
                    Ok(Some(Box::new(validators::$name {
                        number: value
                    })))
                } else {
                    Err(schema::SchemaError::Malformed {
                        path: ctx.fragment.join("/"),
                        detail: "the `minimum/maximum/exclusiveMinimum/exclusiveMaximum` value must be a number".to_string()
                    })
                }
            }
        }
    }
}

kw_minmax!(Maximum, "maximum");
kw_minmax!(ExclusiveMaximum, "exclusiveMaximum");
kw_minmax!(Minimum, "minimum");
kw_minmax!(ExclusiveMinimum, "exclusiveMinimum");
