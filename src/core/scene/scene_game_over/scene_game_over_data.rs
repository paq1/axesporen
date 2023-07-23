pub struct SceneGameOverData {
    pub is_init: bool,
    pub nb_monde_decouvert: u32
}

impl SceneGameOverData {
    pub fn new(nb_monde_decouvert: u32) -> Self {
        Self {
            is_init: false,
            nb_monde_decouvert
        }
    }
}