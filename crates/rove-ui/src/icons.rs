//! Rove's interface icons (`icons/svg`, MIT): 24-unit grid, stroke 1.75, round caps and joins.
//!
//! Each SVG file is the icon body as is. Rune's icon renderer reads the root `<svg>` as a group,
//! so its `fill="none" stroke="currentColor" stroke-width="1.75"` attributes reach the paths, and
//! the stroke draws in the element's `.color(..)`. Like the reference's `<svg viewBox="0 0 24 24">`,
//! the stroke scales with the drawn size.

use rune::text::Icon;

macro_rules! icons {
    ($($id:ident = $file:literal),* $(,)?) => {
        $(
            #[allow(dead_code)]
            pub static $id: Icon = Icon { name: $file, view: 24.0, body: include_str!(concat!("../icons/", $file, ".svg")) };
        )*
    };
}

icons! {
    ARCHIVE = "archive", BOLT = "bolt", CHECK = "check", CHEV_D = "chevD", CHEV_L = "chevL",
    CHEV_R = "chevR", CHEV_U = "chevU", CLOCK = "clock", CLOUD = "cloud", CODE = "code",
    COLUMNS = "columns", COPY = "copy", CUT = "cut", DISK = "disk", DOC = "doc",
    DOWNLOAD = "download", DRIVE = "drive", DUAL = "dual", EJECT = "eject", EYE = "eye",
    GIT = "git", GLOBE = "globe", GRID = "grid", HOME = "home", IMAGE = "image", INFO = "info",
    KEYBOARD = "keyboard", LINK = "link", LIST = "list", LOCK = "lock", MENU = "menu",
    MINUS = "minus", MONITOR = "monitor", MORE = "more", MUSIC = "music", OPEN = "open",
    PANEL = "panel", PENCIL = "pencil", PHONE = "phone", PIN = "pin", PLUG = "plug", PLUS = "plus",
    REFRESH = "refresh", SEARCH = "search", SERVER = "server", SHARE = "share", SHELF = "shelf",
    SIDEBAR = "sidebar", SLIDERS = "sliders", SORT = "sort", STAR = "star", TAG = "tag",
    TERMINAL = "terminal", TRASH = "trash", TYPE = "type", UNDO = "undo", UP = "up", USB = "usb",
    VIDEO = "video", X = "x", ZIP = "zip",
}

/// The selection check badge's tick: the `check` path at stroke 3.2 (`.chk .i`).
pub static CHECK_BOLD: Icon = Icon {
    name: "check-bold",
    view: 24.0,
    body: r#"<g fill="none" stroke="currentColor" stroke-width="3.2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></g>"#,
};

/// The video thumbnail's play triangle, filled.
pub static PLAY: Icon = Icon { name: "play", view: 24.0, body: r#"<path d="M8 5v14l11-7z"/>"# };
