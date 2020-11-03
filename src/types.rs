use common::*;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Project {
	pub scenes: Vec<SceneData>,
	pub entities: Vec<EntityData>,
	pub meshes: Vec<MeshData>,
}

#[derive(Debug, Clone)]
pub struct SceneData {
	pub name: String,
	pub entities: Vec<u32>
}

#[derive(Debug, Clone)]
pub struct EntityData {
	pub name: String,
	pub mesh_id: u16,

	pub position: Vec3,
	pub rotation: Quat,
	pub scale: Vec3,
}

#[derive(Debug, Clone)]
pub struct MeshData {
	pub positions: Vec<Vec3>,
	pub indices: Vec<u16>,
	pub color_data: Vec<MeshColorData>,
	pub weight_data: Option<MeshWeightData>,
}

#[derive(Debug, Clone)]
pub struct MeshColorData {
	pub name: String,
	pub data: Vec<Vec4>,
}

#[derive(Debug, Clone)]
pub struct MeshWeightData {
	pub bones: Vec<MeshBone>,
	pub weights: Vec<MeshWeightVertex>,
}

#[derive(Debug, Clone)]
pub struct MeshBone {
	pub name: String,
	pub head: Vec3,
	pub tail: Vec3,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MeshWeightVertex {
	pub indices: [u8; 3],
	pub weights: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct SceneRef<'toy> {
	file: &'toy Project,
	scene: &'toy SceneData,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityRef<'toy> {
	file: &'toy Project,
	entity: &'toy EntityData,
}

impl Project {
	pub fn find_scene(&self, name: &str) -> Option<SceneRef<'_>> {
		self.scenes.iter()
			.find(|e| e.name == name)
			.map(|scene| SceneRef::from(self, scene))
	}

	pub fn find_entity(&self, name: &str) -> Option<EntityRef<'_>> {
		self.entities.iter()
			.find(|e| e.name == name)
			.map(|entity| EntityRef::from(self, entity))
	}

	pub fn entities(&self) -> impl Iterator<Item=EntityRef<'_>> {
		self.entities.iter()
			.map(move |entity| EntityRef::from(self, entity))
	}
}

impl MeshData {
	pub fn color_data(&self, name: &str) -> Option<&MeshColorData> {
		self.color_data.iter()
			.find(|l| l.name == name)
	}
}

impl<'t> SceneRef<'t> {
	pub fn from(file: &'t Project, scene: &'t SceneData) -> SceneRef<'t> {
		SceneRef { file, scene }
	}

	pub fn entities(&self) -> impl Iterator<Item=EntityRef<'t>> {
		let file = self.file;

		self.scene.entities.iter()
			.map(move |&id| &file.entities[id as usize - 1])
			.map(move |entity| EntityRef::from(file, entity))
	}

	pub fn find_entity(&self, name: &str) -> Option<EntityRef<'t>> {
		self.entities().find(|ent| ent.entity.name == name)
	}
}

impl Deref for SceneRef<'_> {
	type Target = SceneData;
	fn deref(&self) -> &Self::Target { self.scene }
}

impl<'t> EntityRef<'t> {
	pub fn from(file: &'t Project, entity: &'t EntityData) -> EntityRef<'t> {
		EntityRef { file, entity }
	}

	pub fn mesh_data(&self) -> Option<&'t MeshData> {
		let mesh_id = self.entity.mesh_id;

		if mesh_id == 0 {
			return None
		}

		self.file.meshes.get(mesh_id as usize - 1)
	}
}

impl Deref for EntityRef<'_> {
	type Target = EntityData;
	fn deref(&self) -> &Self::Target { self.entity }
}

// TODO: entity queries
// TODO: mesh building