//! Fixed preview and reference data: `~/Documents`, 6 folders, 10 files,
//! `shore-at-dusk.jpg` selected, `forest-walk.jpg` hovered. Exactly what the reference HTML shows.

use crate::icons;
use rune::text::Icon;

pub mod scenes;

pub struct Place {
    pub icon: &'static Icon,
    pub name: &'static str,
}

pub const FAVOURITES: [Place; 8] = [
    Place { icon: &icons::HOME, name: "Home" },
    Place { icon: &icons::DOWNLOAD, name: "Downloads" },
    Place { icon: &icons::DOC, name: "Documents" },
    Place { icon: &icons::IMAGE, name: "Pictures" },
    Place { icon: &icons::MUSIC, name: "Music" },
    Place { icon: &icons::VIDEO, name: "Videos" },
    Place { icon: &icons::CODE, name: "Projects" },
    Place { icon: &icons::TRASH, name: "Trash" },
];
/// The place the active tab shows.
pub const CURRENT_PLACE: usize = 2;

pub enum RemoteState {
    Connected,
    Offline(&'static str),
}

pub struct Remote {
    pub icon: &'static Icon,
    pub name: &'static str,
    pub state: RemoteState,
}

pub const NETWORK: [Remote; 2] = [
    Remote { icon: &icons::CLOUD, name: "gdrive", state: RemoteState::Connected },
    Remote { icon: &icons::SERVER, name: "nas", state: RemoteState::Offline("SMB · offline") },
];

pub struct Device {
    pub icon: &'static Icon,
    pub name: &'static str,
    /// Fraction used, 0..1.
    pub used: f32,
    pub caption: &'static str,
    pub removable: bool,
}

pub const DEVICES: [Device; 2] = [
    Device { icon: &icons::DRIVE, name: "som2omarchy", used: 0.63, caption: "191.2 GB free of 512 GB", removable: false },
    Device { icon: &icons::USB, name: "SanDisk Ultra", used: 0.89, caption: "7.1 GB free of 64 GB", removable: true },
];

pub struct Tab {
    pub icon: &'static Icon,
    pub name: &'static str,
}

pub const TABS: [Tab; 3] = [
    Tab { icon: &icons::DOC, name: "Documents" },
    Tab { icon: &icons::DOWNLOAD, name: "Downloads" },
    Tab { icon: &icons::CODE, name: "rove" },
];
pub const ACTIVE_TAB: usize = 0;

/// What a tile shows in its art box.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Art {
    /// An icon-theme icon, by its WhiteSur reference file name (`whitesur-<name>.png`).
    Icon(&'static str),
    /// A thumbnail from `Thumb`.
    Thumb(Thumb),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Thumb {
    Shore,
    Alps,
    Forest,
}

impl Thumb {
    pub const ALL: [Thumb; 3] = [Thumb::Shore, Thumb::Alps, Thumb::Forest];

    /// Video duration, for the chip.
    pub fn duration(self) -> Option<&'static str> {
        (self == Thumb::Alps).then_some("0:42")
    }

    /// Stand-in thumbnail art, the reference's inline SVG, sized 92×62 as the tile shows it
    /// (the 96×64 viewBox fits inside, as the browser's `meet` fits it).
    pub fn svg(self) -> &'static str {
        match self {
            Thumb::Shore => concat!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" width="92" height="62" viewBox="0 0 96 64">"##,
                r##"<defs><linearGradient id="gS" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#2B2A63"/><stop offset=".62" stop-color="#D9716A"/><stop offset="1" stop-color="#F7B57A"/></linearGradient></defs>"##,
                r##"<rect width="96" height="64" fill="url(#gS)"/><circle cx="64" cy="38" r="8" fill="#FFE1B0"/><rect y="41" width="96" height="23" fill="#232D57"/>"##,
                r##"<path d="M0 45h96 M12 50h30 M52 54h34 M20 59h40" stroke="#F7B57A" stroke-opacity=".45" stroke-width="1.2"/></svg>"##
            ),
            Thumb::Alps => concat!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" width="92" height="62" viewBox="0 0 96 64">"##,
                r##"<defs><linearGradient id="gP" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#8EC1EE"/><stop offset="1" stop-color="#E6F0FA"/></linearGradient></defs>"##,
                r##"<rect width="96" height="64" fill="url(#gP)"/><path d="M0 52 L22 26 L34 38 L52 14 L74 40 L86 30 L96 40 V64 H0z" fill="#4C6E9D"/>"##,
                r##"<path d="M52 14 L58 21 L54 22 L49 18z M22 26 L26 31 L19 30z" fill="#F4F8FC"/><path d="M0 58 L30 44 L56 56 L80 46 L96 54 V64 H0z" fill="#2E4670"/></svg>"##
            ),
            Thumb::Forest => concat!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" width="92" height="62" viewBox="0 0 96 64">"##,
                r##"<rect width="96" height="64" fill="#1E3A34"/><rect width="96" height="30" fill="#7FB7A4"/>"##,
                r##"<path d="M8 50 L16 22 L24 50z M26 54 L36 16 L46 54z M50 52 L58 26 L66 52z M68 56 L80 18 L92 56z" fill="#173029"/>"##,
                r##"<rect y="52" width="96" height="12" fill="#10231F"/></svg>"##
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileState {
    Rest,
    Hover,
    Selected,
}

pub struct Item {
    pub name: &'static str,
    pub meta: &'static str,
    pub art: Art,
    pub state: TileState,
}

const fn item(name: &'static str, meta: &'static str, art: Art) -> Item {
    Item { name, meta, art, state: TileState::Rest }
}

pub const FOLDERS: [Item; 6] = [
    item("Invoices", "48 items", Art::Icon("folder")),
    item("Taxes 2026", "12 items", Art::Icon("folder")),
    item("Scans", "131 items", Art::Icon("folder-images")),
    item("Code", "9 items", Art::Icon("folder-code")),
    item("Private", "Locked", Art::Icon("folder-locked")),
    item("Drive sync", "Synced", Art::Icon("folder-cloud")),
];

pub const FILES: [Item; 10] = [
    item("report-q3.odt", "2.4 MB", Art::Icon("x-office-document")),
    item("budget-2026.ods", "88 KB", Art::Icon("x-office-spreadsheet")),
    Item { state: TileState::Selected, ..item("shore-at-dusk.jpg", "4.1 MB", Art::Thumb(Thumb::Shore)) },
    item("alps-timelapse.mkv", "128 MB", Art::Thumb(Thumb::Alps)),
    item("meeting-notes.md", "6 KB", Art::Icon("text-markdown")),
    item("backup-nov.tar.gz", "1.2 GB", Art::Icon("application-x-compressed-tar")),
    item("install.sh", "3 KB", Art::Icon("application-x-shellscript")),
    item("archlinux-2026.11.iso", "1.4 GB", Art::Icon("application-x-cd-image")),
    Item { state: TileState::Hover, ..item("forest-walk.jpg", "3.8 MB", Art::Thumb(Thumb::Forest)) },
    item("todo.txt", "1 KB", Art::Icon("text-x-generic")),
];

/// Every icon-theme icon the mock shows.
pub fn theme_icon_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = FOLDERS.iter().chain(FILES.iter())
        .filter_map(|i| if let Art::Icon(n) = i.art { Some(n) } else { None })
        .collect();
    names.sort_unstable();
    names.dedup();
    names
}

pub const FOLDER_TITLE: &str = "Documents";
pub const ITEM_COUNT: &str = "16 items";
pub const SORT_KEY: &str = "Name";
pub const SELECTION_SUMMARY: &str = "1 of 16 selected · 4.1 MB";
pub const FREE_SPACE: &str = "191.2 GB free";
/// The footer's icon-size slider, 0..1 (the reference is at 45%, tile 124).
pub const ICON_SIZE: f32 = 0.45;

pub struct Crumb {
    pub icon: &'static Icon,
    pub name: &'static str,
}

pub const CRUMBS: [Crumb; 3] = [
    Crumb { icon: &icons::DRIVE, name: "som2omarchy" },
    Crumb { icon: &icons::HOME, name: "Home" },
    Crumb { icon: &icons::DOC, name: "Documents" },
];
