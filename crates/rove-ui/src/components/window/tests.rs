use crate::app::test_support::{FONT_CONTEXT, click, element, preview, settle};

#[test]
fn hidden_sidebar_keeps_equal_panel_insets() {
    let _guard = FONT_CONTEXT.lock().expect("font test lock");
    let mut rt = preview();
    click(&mut rt, "sidebar.toggle");
    settle(&mut rt);
    let [x, y, w, h] = element(&rt, "panel").rect;
    assert_eq!([x, y, 1360.0 - x - w, 840.0 - y - h], [8.0; 4]);
}
