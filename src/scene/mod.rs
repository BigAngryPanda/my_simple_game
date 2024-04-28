pub mod scene_item;
pub mod camera;

pub use scene_item::*;
pub use camera::*;

/// Contains all objects to be rendered
struct Scene
{
	m_objects: Vec<SceneItem>,
}