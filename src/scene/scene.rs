use crate::scene::SceneItem;

/// Contains all objects to be rendered
pub struct Scene
{
    m_objects: Vec<SceneItem>,
}

impl Scene {
    pub fn items(&self) -> impl Iterator<Item = &SceneItem> {
        self.m_objects.iter()
    }
}