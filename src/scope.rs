use super::keywords;
use super::schema;
use hashbrown::HashMap;
use simd_json::value::{BorrowedValue as Value, Value as ValueTrait};

use super::helpers;

#[derive(Debug)]
pub struct Scope<'a, V>
where
    V: ValueTrait,
{
    keywords: keywords::KeywordMap<V>,
    schemes: HashMap<String, schema::Schema<'a, V>>,
}

impl<'a, V> Scope<'a, V>
where
    V: ValueTrait,
{
    pub fn new() -> Scope<'a, V> {
        let mut scope = Scope {
            keywords: keywords::default(),
            schemes: HashMap::new(),
        };

        scope
    }

    pub fn resolve(&'a self, id: &url::Url) -> Option<schema::ScopedSchema<'a, V>> {
        let (schema_path, fragment) = helpers::serialize_schema_path(id);

        let schema = self.schemes.get(&schema_path).or_else(|| {
            for (_, schema) in self.schemes.iter() {
                let internal_schema = schema.resolve(schema_path.as_ref());
                if internal_schema.is_some() {
                    return internal_schema;
                }
            }

            None
        });

        schema.and_then(|schema| match fragment {
            Some(ref fragment) => schema
                .resolve_fragment(fragment)
                .map(|schema| schema::ScopedSchema::new(self, schema)),
            None => Some(schema::ScopedSchema::new(self, schema)),
        })
    }

    pub fn compile_and_return(
        &'_ mut self,
        def: Value<'a>,
        ban_unknown: bool,
    ) -> Result<schema::ScopedSchema<'_, V>, schema::SchemaError> {
        let schema = schema::compile(
            def,
            None,
            schema::CompilationSettings::new(&self.keywords, ban_unknown),
        )?;
        self.add_and_return(schema.id.clone().as_ref().unwrap(), schema)
    }

    #[allow(clippy::map_entry)] // allowing for the return values
    fn add_and_return(
        &mut self,
        id: &url::Url,
        schema: schema::Schema<'a, V>,
    ) -> Result<schema::ScopedSchema<V>, schema::SchemaError> {
        let (id_str, fragment) = helpers::serialize_schema_path(id);

        if fragment.is_some() {
            return Err(schema::SchemaError::WrongId);
        }

        if !self.schemes.contains_key(&id_str) {
            self.schemes.insert(id_str.clone(), schema);
            Ok(schema::ScopedSchema::new(self, &self.schemes[&id_str]))
        } else {
            Err(schema::SchemaError::IdConflicts)
        }
    }
}
