use super::scenegraph::Scenegraph;

pub struct SceneManager<'a> {
    pub scenes: Vec<Scenegraph<'a>>,
    current_scene_index: usize,
}

impl<'a> SceneManager<'a> {
    pub fn new() -> Self {
        Self { scenes: Vec::new(), current_scene_index: 0 }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.scenes[self.current_scene_index].update(delta_time);
    }
    pub fn add_scene(&mut self, scene: Scenegraph<'a>) {
        self.scenes.push(scene);
    }

    pub fn get_current_scene_safe(&self) -> Option<&Scenegraph<'a>> {
        self.scenes.get(self.current_scene_index)
    }

    pub fn get_current_scene_mut_safe(&mut self) -> Option<&mut Scenegraph<'a>> {
        self.scenes.get_mut(self.current_scene_index)
    }

    pub fn get_current_scene(&self) -> &Scenegraph<'a> {
        &self.scenes[self.current_scene_index]
    }

    pub fn get_current_scene_mut(&mut self) -> &mut Scenegraph<'a> {
        &mut self.scenes[self.current_scene_index]
    }

    pub fn next_scene(&mut self) {
        self.current_scene_index += 1;
        if self.current_scene_index >= self.scenes.len() {
            self.current_scene_index = 0;
        }
    }

    pub fn previous_scene(&mut self) {
        if self.current_scene_index == 0 {
            self.current_scene_index = self.scenes.len() - 1;
        } else {
            self.current_scene_index -= 1;
        }
    }
}
