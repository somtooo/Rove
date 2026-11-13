use crate::app::test_support::{FONT_CONTEXT, click, element, preview, scroll_down};

#[test]
fn short_sidebar_scrolls_devices_but_keeps_brand_pinned() {
    let _guard = FONT_CONTEXT.lock().expect("font test lock");
    let mut rt = preview();
    rt.set_viewport((900.0, 480.0), 2.0);
    rt.frame(1.0 / 60.0);
    let brand = element(&rt, "sidebar.brand").rect;
    let content = element(&rt, "content").rect;
    scroll_down(&mut rt, "sidebar.content");
    assert_eq!(element(&rt, "sidebar.brand").rect, brand);
    assert_eq!(element(&rt, "content").rect, content, "sidebar scrolling is independent");
    let last = element(&rt, "sidebar.device.caption#1");
    assert!((last.visible[3] - last.rect[3]).abs() < 0.01, "last device is reachable");
    assert!(last.scrolled[1] > 0.0);
}

#[test]
fn sidebar_can_be_hidden_and_restored_without_backend() {
    let _guard = FONT_CONTEXT.lock().expect("font test lock");
    let mut rt = preview();
    click(&mut rt, "sidebar.toggle");
    assert!(!rt.state.sidebar_visible);
    click(&mut rt, "sidebar.show");
    assert!(rt.state.sidebar_visible);
}
