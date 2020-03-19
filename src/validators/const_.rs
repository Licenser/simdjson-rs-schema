use simd_json::value::Value as ValueTrait;

use super::error;
use super::scope;

#[allow(missing_copy_implementations)]
pub struct Const<V: ValueTrait> {
    pub item: V,
}

impl<V> super::Validator<V> for Const<V>
where
    V: ValueTrait + std::clone::Clone + std::convert::From<simd_json::value::owned::Value> + std::fmt::Display + std::marker::Sync + std::marker::Send + std::cmp::PartialEq + 'static,
    <V as ValueTrait>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq + std::convert::AsRef<str> + std::fmt::Debug + std::string::ToString + std::marker::Sync + std::marker::Send,
{
    fn validate(&self, val: &V, path: &str, _scope: &scope::Scope<V>) -> super::ValidationState {
        let mut state = super::ValidationState::new();

        if *val != self.item {
            state.errors.push(Box::new(error::Const {
                path: path.to_string(),
            }))
        }

        state
    }
}
