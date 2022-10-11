// use std::{collections::HashMap, sync::Mutex};

// use crate::{
//     scene::{Scene, SceneRef, SceneId}, 
//     gltf_loader::{GltfLoader, GltfError}, 
//     prelude::GraphicsChip
// };

// pub struct SceneManager {
//     scenes: HashMap<SceneId, Scene>,
// }

// impl SceneManager {
//     pub fn new() -> Self {
//         Self {
//             scenes: HashMap::new(),
//         }
//     }

//     pub fn new_scene(&mut self, path: &String, gpu: &Mutex<GraphicsChip>) -> Result<SceneId, GltfError> {
//         let mut scene = GltfLoader::load(path, gpu)?;
//         let id = uuid::Uuid::new_v4();
//         scene.id = id;
//         self.scenes.insert(id, scene);
//         Ok(id)
//     }

//     pub fn get_scene(&self, index: SceneId) -> Option<&Scene> {
//         self.scenes.get(&index)
//     }
// }

