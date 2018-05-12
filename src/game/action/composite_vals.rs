use super::Action;

#[derive(Clone)]
pub struct CaptureVal {
    pub remove_action: Action,
    pub move_action: Action
}

impl From<[Action; 2]> for CaptureVal {
    fn from(actions: [Action; 2]) -> Self {
        Self {
            remove_action: actions[0].clone(),
            move_action: actions[1].clone()
        }
    }
}
