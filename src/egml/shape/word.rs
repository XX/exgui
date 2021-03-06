use std::convert::AsRef;
use crate::egml::AnyModel;

#[derive(Default)]
pub struct Word {
    pub id: Option<String>,
    pub content: String,
    pub modifier: Option<fn(&mut Word, &dyn AnyModel)>,
}

impl Word {
    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }
}

impl AsRef<str> for Word {
    fn as_ref(&self) -> &str {
        self.content.as_str()
    }
}