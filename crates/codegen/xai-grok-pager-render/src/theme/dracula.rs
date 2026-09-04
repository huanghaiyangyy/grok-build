//! Dracula theme — classic palette from https://draculatheme.com
//!
//! Use with Ghostty (or any transparent terminal) via:
//!   [ui]
//!   theme = "dracula"
//!   screen_mode = "minimal"
//!
//! `minimal` locks chrome to the terminal-native Reset palette so Ghostty's
//! transparent background shows through; Dracula accents still apply when not in
//! minimal mode (fullscreen).

use ratatui::style::{Color, Modifier};

use super::tokyonight::Theme;

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

#[allow(dead_code)]
mod palette {
    use super::*;

    // Classic Dracula
    pub const BG: Color = rgb(40, 42, 54); // #282A36
    pub const BG_LIGHT: Color = rgb(68, 71, 90); // #44475A Current Line / Selection
    pub const BG_DARK: Color = rgb(33, 34, 44); // slightly darker than BG
    pub const FG: Color = rgb(248, 248, 242); // #F8F8F2
    pub const COMMENT: Color = rgb(98, 114, 164); // #6272A4
    pub const CYAN: Color = rgb(139, 233, 253); // #8BE9FD
    pub const GREEN: Color = rgb(80, 250, 123); // #50FA7B
    pub const ORANGE: Color = rgb(255, 184, 108); // #FFB86C
    pub const PINK: Color = rgb(255, 121, 198); // #FF79C6
    pub const PURPLE: Color = rgb(189, 147, 249); // #BD93F9
    pub const RED: Color = rgb(255, 85, 85); // #FF5555
    pub const YELLOW: Color = rgb(241, 250, 140); // #F1FA8C
}
use palette::*;

impl Theme {
    /// Classic Dracula (truecolor). Opaque backgrounds for fullscreen mode.
    pub const fn dracula() -> Self {
        Self {
            bg_base: BG,
            bg_light: BG_LIGHT,
            bg_dark: BG_DARK,
            bg_highlight: BG_LIGHT,
            bg_hover: rgb(58, 60, 78),
            bg_terminal: BG_DARK,

            accent_user: CYAN,
            accent_assistant: PURPLE,
            accent_thinking: COMMENT,
            accent_tool: COMMENT,
            accent_system: CYAN,
            accent_error: RED,
            accent_success: GREEN,
            accent_running: PINK,
            accent_skill: PURPLE,

            text_primary: FG,
            text_secondary: rgb(200, 200, 210),

            gray_dim: rgb(70, 80, 110),
            gray: COMMENT,
            gray_bright: rgb(160, 170, 200),

            command: YELLOW,
            path: ORANGE,
            running: CYAN,
            warning: ORANGE,

            fuzzy_accent: PINK,

            accent_plan: ORANGE,
            accent_verify: PURPLE,
            accent_remember: GREEN,

            selection_border: BG_LIGHT,
            hover_border: rgb(58, 60, 78),
            prompt_border: rgb(58, 60, 78),
            prompt_border_active: PURPLE,

            accent_model: CYAN,

            scrollbar_bg: BG_DARK,
            scrollbar_fg: BG_LIGHT,

            diff_delete_bg: rgb(70, 25, 30),
            diff_delete_fg: RED,
            diff_insert_bg: rgb(20, 55, 35),
            diff_insert_fg: GREEN,
            diff_equal_fg: COMMENT,
            diff_gutter_fg: COMMENT,

            bg_visual: BG_LIGHT,

            paste_bg: BG_DARK,
            paste_fg: FG,
            paste_dim: COMMENT,

            md_heading_h1: PINK,
            md_heading_h1_mod: Modifier::BOLD,
            md_heading_h2: PURPLE,
            md_heading_h2_mod: Modifier::BOLD,
            md_heading_h3: CYAN,
            md_heading_h3_mod: Modifier::BOLD,
            md_heading_h4: GREEN,
            md_heading_h4_mod: Modifier::BOLD,
            md_heading_h5: ORANGE,
            md_heading_h5_mod: Modifier::BOLD,
            md_heading_h6: YELLOW,
            md_heading_h6_mod: Modifier::BOLD,
            md_code: GREEN,
            md_task_checked: GREEN,
            md_task_unchecked: COMMENT,
            md_muted: COMMENT,
            md_code_bg: BG_DARK,
            md_text: FG,
            link_fg: CYAN,
        }
    }

    /// Dracula accents on a transparent / terminal-native canvas.
    ///
    /// All background fields are [`Color::Reset`] so Ghostty (and similar)
    /// compositor transparency shows through even in fullscreen, without
    /// requiring `screen_mode = "minimal"`.
    pub const fn dracula_transparent() -> Self {
        let mut t = Self::dracula();
        // Clear only the full-canvas backgrounds so Ghostty opacity shows through.
        // Keep scrollbar / selection / code-block chrome in Dracula RGB for contrast.
        t.bg_base = Color::Reset;
        t.bg_dark = Color::Reset;
        t.bg_terminal = Color::Reset;
        t
    }
}
