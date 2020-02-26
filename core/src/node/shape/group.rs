use crate::node::{Fill, Stroke, Transform, TransformMatrix};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Group {
    pub id: Option<String>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Fill>,
    pub transform: Transform,
}

impl Group {
    pub const NAME: &'static str = "group";

    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }

    pub fn recalculate_transform(&mut self, parent_global: TransformMatrix) -> TransformMatrix {
        self.transform.calculate_global(parent_global)
    }

    pub fn empty_overrides(&self) -> bool {
        self.stroke.is_none() && self.fill.is_none() && self.transform.is_not_exist()
    }
}