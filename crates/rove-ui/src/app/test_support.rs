//! Shared runtime harness. Component behaviour tests live beside their components.

use rune::{Env, Runtime};

use super::State;
use crate::components::window;

// Rune publishes measurement-font configuration process-wide. Don't let Runtime::new's
// default fonts race the explicit Inter/JetBrains font test.
pub(crate) static FONT_CONTEXT: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) fn preview() -> Runtime<State> {
    let mut state = super::init();
    state.frame = None;
    let mut rt = Runtime::new(state, window::view, window::WINDOW_SIZE,
        Env { system_dark: true, scale: 2.0 });
    rt.frame(0.0);
    rt
}

pub(crate) fn click(rt: &mut Runtime<State>, key: &str) {
    let (x, y) = rt.find(key).expect("control exists");
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(1.0 / 60.0);
}

pub(crate) fn element(rt: &Runtime<State>, key: &str) -> rune::ElementInfo {
    rt.elements().into_iter().find(|e| e.key.as_deref() == Some(key)).expect("element exists")
}

pub(crate) fn settle(rt: &mut Runtime<State>) {
    for _ in 0..240 {
        rt.frame(1.0 / 120.0);
    }
}

pub(crate) fn scroll_down(rt: &mut Runtime<State>, key: &str) {
    let [x, y, w, h] = element(rt, key).rect;
    rt.wheel(x + w / 2.0, y + h / 2.0, -4000.0);
    settle(rt);
}
