use rune::tok::TextStyle;

use super::test_support::FONT_CONTEXT;

/// Inter renders with tabular figures, and JetBrains Mono is loaded.
#[test]
fn fonts_load_with_tabular_figures_and_mono() {
    let _guard = FONT_CONTEXT.lock().expect("font test lock");
    let _text = rune::text::TextSystem::new(super::FONTS, ("Inter", "JetBrains Mono"));
    let adv = |s: &str, style: TextStyle| rune::text::advance(s, style);
    let tnum = crate::theme::type_::FOOT_NUM;
    let prop = TextStyle::new(12.5, 400.0);
    assert!((adv("1111", tnum) - adv("0000", tnum)).abs() < 0.01, "tabular: every figure one width");
    assert!(adv("1111", prop) < adv("0000", prop) - 0.5, "proportional figures differ, so tnum did something");
    let mono = crate::theme::type_::MONO;
    assert!((adv("iiii", mono) - adv("MMMM", mono)).abs() < 0.01, "mono slot is monospaced");
    // JetBrains Mono's advance is 600/1000 em.
    assert!((adv("M", mono) - 12.5 * 0.6).abs() < 0.05, "mono slot is JetBrains Mono: {}", adv("M", mono));
}
