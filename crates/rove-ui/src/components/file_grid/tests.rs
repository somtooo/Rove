use crate::app::test_support::{FONT_CONTEXT, click, element, preview, scroll_down};
use crate::fixtures;

#[test]
fn short_windows_clip_tiles_above_footer_and_scroll_to_last_file() {
    let _guard = FONT_CONTEXT.lock().expect("font test lock");
    for width in [1360.0, 900.0, 720.0] {
        for sidebar_visible in [true, false] {
            let mut rt = preview();
            rt.state.sidebar_visible = sidebar_visible;
            rt.set_viewport((width, 480.0), 2.0);
            rt.frame(1.0 / 60.0);
            let footer = element(&rt, "footer").rect;
            let content = element(&rt, "content").rect;
            assert!((content[1] + content[3] - footer[1]).abs() < 0.01);
            for tile in rt.elements().into_iter().filter(|e| e.role == "button" && e.label.as_deref().is_some_and(|name|
                fixtures::FOLDERS.iter().chain(fixtures::FILES.iter()).any(|item| item.name == name))) {
                assert!(tile.visible[3] == 0.0 || tile.visible[1] + tile.visible[3] <= footer[1] + 0.01);
            }
            scroll_down(&mut rt, "content");
            assert_eq!(element(&rt, "footer").rect, footer, "footer stays pinned");
            let last = rt.elements().into_iter().find(|e| e.label.as_deref() == Some("todo.txt") && e.role == "button").unwrap();
            assert!((last.visible[3] - last.rect[3]).abs() < 0.01, "last file is fully reachable at {width}, sidebar={sidebar_visible}");
            assert!(last.scrolled[1] > 0.0, "the grid actually scrolled");
        }
    }
}

#[test]
fn clicking_a_tile_changes_the_selection() {
    let _guard = FONT_CONTEXT.lock().expect("font test lock");
    let mut rt = preview();
    assert_eq!(rt.state.selected, Some("shore-at-dusk.jpg"));
    click(&mut rt, "tile");
    assert_eq!(rt.state.selected, Some("Invoices"));
    click(&mut rt, "tile.hover");
    assert_eq!(rt.state.selected, Some("forest-walk.jpg"));
}
