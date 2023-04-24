use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use valuable::{Valuable, NamedField, Value, NamedValues, Structable, StructDef, Fields};

/// Raw data about mastodon app. Save `Data` using `serde` to prevent needing
/// to authenticate on every run.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct Data {
    /// Base url of instance eg. `https://botsin.space`.
    pub base: Cow<'static, str>,
    /// The client's id given by the instance.
    pub client_id: Cow<'static, str>,
    /// The client's secret given by the instance.
    pub client_secret: Cow<'static, str>,
    /// Url to redirect back to your application from the instance signup.
    pub redirect: Cow<'static, str>,
    /// The client's access token.
    pub token: Cow<'static, str>,
}

/// Fields that will be logged - generally we avoid logging secrets like
/// access token and client secret
static DATA_FIELDS: &[NamedField] = &[
    NamedField::new("base"),
    NamedField::new("client_id"),
    NamedField::new("redirect"),
];

impl Valuable for Data {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, visit: &mut dyn valuable::Visit) {
        let data = &self;
        visit.visit_named_fields(&NamedValues::new(
            DATA_FIELDS,
            &[
                Value::String(&data.base),
                Value::String(&data.client_id),
                Value::String(&data.redirect),
            ],
        ));
    }
}

impl Structable for Data {
    fn definition(&self) -> StructDef<'_> {
        StructDef::new_static("Data", Fields::Named(DATA_FIELDS))
    }
}