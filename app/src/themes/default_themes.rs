use asset_macro::bundled_or_fetched_asset;
use pathfinder_color::ColorU;
use warp_core::ui::{
    color::{blend::Blend, coloru_with_opacity, OPAQUE},
    theme::{
        color::CustomDetails, AnsiColor, AnsiColors, Details, Fill, HorizontalGradient, Image,
        TerminalColors, VerticalGradient, WarpTheme,
    },
};

const DARK_MODE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x616161FF),
    AnsiColor::from_u32(0xFF8272FF),
    AnsiColor::from_u32(0xB4FA72FF),
    AnsiColor::from_u32(0xFEFDC2FF),
    AnsiColor::from_u32(0xA5D5FEFF),
    AnsiColor::from_u32(0xFF8FFDFF),
    AnsiColor::from_u32(0xD0D1FEFF),
    AnsiColor::from_u32(0xF1F1F1FF),
);
const DARK_MODE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x8E8E8EFF),
    AnsiColor::from_u32(0xFFC4BDFF),
    AnsiColor::from_u32(0xD6FCB9FF),
    AnsiColor::from_u32(0xFEFDD5FF),
    AnsiColor::from_u32(0xC1E3FEFF),
    AnsiColor::from_u32(0xFFB1FEFF),
    AnsiColor::from_u32(0xE5E6FEFF),
    AnsiColor::from_u32(0xFEFFFFFF),
);

const LIGHT_MODE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x212121FF),
    AnsiColor::from_u32(0xC30771FF),
    AnsiColor::from_u32(0x10A778FF),
    AnsiColor::from_u32(0xA89C14FF),
    AnsiColor::from_u32(0x008EC4FF),
    AnsiColor::from_u32(0x523C79FF),
    AnsiColor::from_u32(0x20A5BAFF),
    AnsiColor::from_u32(0xE0E0E0FF),
);
const LIGHT_MODE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x212121FF),
    AnsiColor::from_u32(0xFB007AFF),
    AnsiColor::from_u32(0x5FD7AFFF),
    AnsiColor::from_u32(0xF3E430FF),
    AnsiColor::from_u32(0x20BBFCFF),
    AnsiColor::from_u32(0x6855DEFF),
    AnsiColor::from_u32(0x4FB8CCFF),
    AnsiColor::from_u32(0xF1F1F1FF),
);

const SOLARIZED_DARK_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x073642FF),
    AnsiColor::from_u32(0xDC322FFF),
    AnsiColor::from_u32(0x859900FF),
    AnsiColor::from_u32(0xB58900FF),
    AnsiColor::from_u32(0x268BD2FF),
    AnsiColor::from_u32(0xD33682FF),
    AnsiColor::from_u32(0x2AA198FF),
    AnsiColor::from_u32(0xEEE8D5FF),
);
const SOLARIZED_DARK_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x002B36FF),
    AnsiColor::from_u32(0xCB4B16FF),
    AnsiColor::from_u32(0x586E75FF),
    AnsiColor::from_u32(0x657B83FF),
    AnsiColor::from_u32(0x839496FF),
    AnsiColor::from_u32(0x6C71C4FF),
    AnsiColor::from_u32(0x93A1A1FF),
    AnsiColor::from_u32(0xFDF6E3FF),
);

const SOLARIZED_LIGHT_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x073642FF),
    AnsiColor::from_u32(0xDC322FFF),
    AnsiColor::from_u32(0x859900FF),
    AnsiColor::from_u32(0xB58900FF),
    AnsiColor::from_u32(0x268BD2FF),
    AnsiColor::from_u32(0xD33682FF),
    AnsiColor::from_u32(0x2AA198FF),
    AnsiColor::from_u32(0xEEE8D5FF),
);
const SOLARIZED_LIGHT_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x002B36FF),
    AnsiColor::from_u32(0xCB4B16FF),
    AnsiColor::from_u32(0x586E75FF),
    AnsiColor::from_u32(0x657B83FF),
    AnsiColor::from_u32(0x839496FF),
    AnsiColor::from_u32(0x6C71C4FF),
    AnsiColor::from_u32(0x93A1A1FF),
    AnsiColor::from_u32(0xFDF6E3FF),
);

const DRACULA_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x000000FF),
    AnsiColor::from_u32(0xFF5555FF),
    AnsiColor::from_u32(0x50FA7BFF),
    AnsiColor::from_u32(0xF1FA8CFF),
    AnsiColor::from_u32(0xBD93F9FF),
    AnsiColor::from_u32(0xFF79C6FF),
    AnsiColor::from_u32(0x8BE9FDFF),
    AnsiColor::from_u32(0xBBBBBBFF),
);
const DRACULA_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x555555FF),
    AnsiColor::from_u32(0xFF5555FF),
    AnsiColor::from_u32(0x50FA7BFF),
    AnsiColor::from_u32(0xF1FA8CFF),
    AnsiColor::from_u32(0xCAA9FAFF),
    AnsiColor::from_u32(0xFF79C6FF),
    AnsiColor::from_u32(0x8BE9FDFF),
    AnsiColor::from_u32(0xFFFFFFFF),
);

const PHENOMENON_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x121212FF),
    AnsiColor::from_u32(0xD22D1EFF),
    AnsiColor::from_u32(0x1CA05AFF),
    AnsiColor::from_u32(0xE5A01AFF),
    AnsiColor::from_u32(0x3780E9FF),
    AnsiColor::from_u32(0xBF409DFF),
    AnsiColor::from_u32(0x799C92FF),
    AnsiColor::from_u32(0xFAF9F6FF),
);
const PHENOMENON_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x292929FF),
    AnsiColor::from_u32(0xAE756FFF),
    AnsiColor::from_u32(0x789B88FF),
    AnsiColor::from_u32(0xBD9F65FF),
    AnsiColor::from_u32(0x6F839FFF),
    AnsiColor::from_u32(0xA57899FF),
    AnsiColor::from_u32(0xBFC5C3FF),
    AnsiColor::from_u32(0xFFFFFFFF),
);

const GRUVBOX_DARK_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x282828FF),
    AnsiColor::from_u32(0xCC241DFF),
    AnsiColor::from_u32(0x98971AFF),
    AnsiColor::from_u32(0xD79921FF),
    AnsiColor::from_u32(0x458588FF),
    AnsiColor::from_u32(0xB16286FF),
    AnsiColor::from_u32(0x689D6AFF),
    AnsiColor::from_u32(0xA89984FF),
);
const GRUVBOX_DARK_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x928374FF),
    AnsiColor::from_u32(0xFB4934FF),
    AnsiColor::from_u32(0xB8BB26FF),
    AnsiColor::from_u32(0xFABD2FFF),
    AnsiColor::from_u32(0x83A598FF),
    AnsiColor::from_u32(0xD3869BFF),
    AnsiColor::from_u32(0x8EC07CFF),
    AnsiColor::from_u32(0xEBDBB2FF),
);

const GRUVBOX_LIGHT_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0xFBF1C7FF),
    AnsiColor::from_u32(0xCC241DFF),
    AnsiColor::from_u32(0x98971AFF),
    AnsiColor::from_u32(0xD79921FF),
    AnsiColor::from_u32(0x458588FF),
    AnsiColor::from_u32(0xB16286FF),
    AnsiColor::from_u32(0x689D6AFF),
    AnsiColor::from_u32(0x7C6F64FF),
);
const GRUVBOX_LIGHT_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x928374FF),
    AnsiColor::from_u32(0x9D0006FF),
    AnsiColor::from_u32(0x79740EFF),
    AnsiColor::from_u32(0xB57614FF),
    AnsiColor::from_u32(0x076678FF),
    AnsiColor::from_u32(0x8F3F71FF),
    AnsiColor::from_u32(0x427B58FF),
    AnsiColor::from_u32(0x3C3836FF),
);

const SOLARFLARE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x2E333DFF),
    AnsiColor::from_u32(0xD66060FF),
    AnsiColor::from_u32(0x64AF86FF),
    AnsiColor::from_u32(0xCAA358FF),
    AnsiColor::from_u32(0x5C80B2FF),
    AnsiColor::from_u32(0xB766A1FF),
    AnsiColor::from_u32(0x8069A1FF),
    AnsiColor::from_u32(0xF0F4F7FF),
);
const SOLARFLARE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x37404AFF),
    AnsiColor::from_u32(0xEB8282FF),
    AnsiColor::from_u32(0x64AF86FF),
    AnsiColor::from_u32(0xCAA358FF),
    AnsiColor::from_u32(0x5C80B2FF),
    AnsiColor::from_u32(0xB766A1FF),
    AnsiColor::from_u32(0x8069A1FF),
    AnsiColor::from_u32(0xFFFFFFFF),
);

const ADEBERRY_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x121212FF),
    AnsiColor::from_u32(0xC76156FF),
    AnsiColor::from_u32(0x57C78AFF),
    AnsiColor::from_u32(0xC8A35AFF),
    AnsiColor::from_u32(0x5785C7FF),
    AnsiColor::from_u32(0xC756A9FF),
    AnsiColor::from_u32(0x57C7C3FF),
    AnsiColor::from_u32(0xEEEDEBFF),
);
const ADEBERRY_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x292929FF),
    AnsiColor::from_u32(0xD22D1EFF),
    AnsiColor::from_u32(0x1CA05AFF),
    AnsiColor::from_u32(0xE5A01AFF),
    AnsiColor::from_u32(0x1458B8FF),
    AnsiColor::from_u32(0xA43787FF),
    AnsiColor::from_u32(0x4D9989FF),
    AnsiColor::from_u32(0xFFFFFFFF),
);

const VINTAGE_DUNE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x1B1A24FF),
    AnsiColor::from_u32(0xD9907AFF),
    AnsiColor::from_u32(0xAFCB8AFF),
    AnsiColor::from_u32(0xDAA464FF),
    AnsiColor::from_u32(0x9EA8D4FF),
    AnsiColor::from_u32(0xD7A2C8FF),
    AnsiColor::from_u32(0x8CC7C9FF),
    AnsiColor::from_u32(0xE8DDB4FF),
);
const VINTAGE_DUNE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x4A465BFF),
    AnsiColor::from_u32(0xF0A58DFF),
    AnsiColor::from_u32(0xC4DFA3FF),
    AnsiColor::from_u32(0xDEC384FF),
    AnsiColor::from_u32(0xB8C1EAFF),
    AnsiColor::from_u32(0xE9B8DCFF),
    AnsiColor::from_u32(0xA6DDE0FF),
    AnsiColor::from_u32(0xFFF8DCFF),
);

const SAGE_MEADOW_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x16251FFF),
    AnsiColor::from_u32(0xE09A8BFF),
    AnsiColor::from_u32(0x88BDA4FF),
    AnsiColor::from_u32(0xD8C48CFF),
    AnsiColor::from_u32(0x9BC7D3FF),
    AnsiColor::from_u32(0xC49AC9FF),
    AnsiColor::from_u32(0xB1D3B9FF),
    AnsiColor::from_u32(0xE6F2DDFF),
);
const SAGE_MEADOW_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x415A51FF),
    AnsiColor::from_u32(0xF2B1A4FF),
    AnsiColor::from_u32(0xA4D4BCFF),
    AnsiColor::from_u32(0xE6D5A7FF),
    AnsiColor::from_u32(0xB5DEEAFF),
    AnsiColor::from_u32(0xDEB8E3FF),
    AnsiColor::from_u32(0xC9E7CFFF),
    AnsiColor::from_u32(0xF8FFF2FF),
);

const MINT_HALO_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x171A35FF),
    AnsiColor::from_u32(0xF0A1B7FF),
    AnsiColor::from_u32(0xA3E7C1FF),
    AnsiColor::from_u32(0xE7D98FFF),
    AnsiColor::from_u32(0xAEE2FFFF),
    AnsiColor::from_u32(0xB5BAFFFF),
    AnsiColor::from_u32(0x9BE6EAFF),
    AnsiColor::from_u32(0xD9F9DFFF),
);
const MINT_HALO_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x454B75FF),
    AnsiColor::from_u32(0xFFC0D0FF),
    AnsiColor::from_u32(0xC2F6D5FF),
    AnsiColor::from_u32(0xF4E9ADFF),
    AnsiColor::from_u32(0xC9EEFFFF),
    AnsiColor::from_u32(0xD1D4FFFF),
    AnsiColor::from_u32(0xB8F7FAFF),
    AnsiColor::from_u32(0xF0FFF2FF),
);

const SPRING_SORBET_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x172B32FF),
    AnsiColor::from_u32(0xF9B2D7FF),
    AnsiColor::from_u32(0xB8F2C1FF),
    AnsiColor::from_u32(0xEAF0A6FF),
    AnsiColor::from_u32(0xCFECF3FF),
    AnsiColor::from_u32(0xE8A6D0FF),
    AnsiColor::from_u32(0xA8DDE6FF),
    AnsiColor::from_u32(0xF6FFDCFF),
);
const SPRING_SORBET_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x46606AFF),
    AnsiColor::from_u32(0xFFC7E3FF),
    AnsiColor::from_u32(0xD6F9DEFF),
    AnsiColor::from_u32(0xF6FFDCFF),
    AnsiColor::from_u32(0xE3F6FAFF),
    AnsiColor::from_u32(0xFFC3E4FF),
    AnsiColor::from_u32(0xD6F7FFFF),
    AnsiColor::from_u32(0xFEFFF4FF),
);

const LAVENDER_HUSH_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x211D32FF),
    AnsiColor::from_u32(0xE8A0A8FF),
    AnsiColor::from_u32(0xB7D4B1FF),
    AnsiColor::from_u32(0xE4D49AFF),
    AnsiColor::from_u32(0xB4D3D9FF),
    AnsiColor::from_u32(0xBDA6CEFF),
    AnsiColor::from_u32(0xA7DAD8FF),
    AnsiColor::from_u32(0xF2EAE0FF),
);
const LAVENDER_HUSH_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x514A69FF),
    AnsiColor::from_u32(0xF2B8BFFF),
    AnsiColor::from_u32(0xCAE6C5FF),
    AnsiColor::from_u32(0xF0E2B1FF),
    AnsiColor::from_u32(0xC9E7ECFF),
    AnsiColor::from_u32(0xD6C0E6FF),
    AnsiColor::from_u32(0xBEF0EEFF),
    AnsiColor::from_u32(0xFFF8EEFF),
);

const PEACH_NIGHT_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x2C2024FF),
    AnsiColor::from_u32(0xFF9A86FF),
    AnsiColor::from_u32(0xB9D8A2FF),
    AnsiColor::from_u32(0xFFD6A6FF),
    AnsiColor::from_u32(0xA8CFE8FF),
    AnsiColor::from_u32(0xFFB399FF),
    AnsiColor::from_u32(0xB7E1D6FF),
    AnsiColor::from_u32(0xFFF0BEFF),
);
const PEACH_NIGHT_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x5E4B50FF),
    AnsiColor::from_u32(0xFFB2A4FF),
    AnsiColor::from_u32(0xD0EDBCFF),
    AnsiColor::from_u32(0xFFE3C2FF),
    AnsiColor::from_u32(0xC4E5FAFF),
    AnsiColor::from_u32(0xFFC8B6FF),
    AnsiColor::from_u32(0xD0F0E8FF),
    AnsiColor::from_u32(0xFFF8D9FF),
);

const BLUEBELL_MOON_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x182231FF),
    AnsiColor::from_u32(0xE99DA2FF),
    AnsiColor::from_u32(0xB7D7B3FF),
    AnsiColor::from_u32(0xE8D5A5FF),
    AnsiColor::from_u32(0x81A6C6FF),
    AnsiColor::from_u32(0xC0A9CEFF),
    AnsiColor::from_u32(0xAACDDCFF),
    AnsiColor::from_u32(0xF3E3D0FF),
);
const BLUEBELL_MOON_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x465466FF),
    AnsiColor::from_u32(0xF2B8BCFF),
    AnsiColor::from_u32(0xD0EBD0FF),
    AnsiColor::from_u32(0xF2E1BEFF),
    AnsiColor::from_u32(0x9FC2E1FF),
    AnsiColor::from_u32(0xD9C1E6FF),
    AnsiColor::from_u32(0xC4E4F0FF),
    AnsiColor::from_u32(0xFFF2E1FF),
);

const ROSE_MILK_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x2B2029FF),
    AnsiColor::from_u32(0xF5AFAFFF),
    AnsiColor::from_u32(0xBFD7B4FF),
    AnsiColor::from_u32(0xEAD8A6FF),
    AnsiColor::from_u32(0xB7D4E8FF),
    AnsiColor::from_u32(0xF9DFDFFF),
    AnsiColor::from_u32(0xD2E6D6FF),
    AnsiColor::from_u32(0xFBEFEFFF),
);
const ROSE_MILK_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x5B4A58FF),
    AnsiColor::from_u32(0xFFC7C7FF),
    AnsiColor::from_u32(0xD9EFCFFF),
    AnsiColor::from_u32(0xF6E6BEFF),
    AnsiColor::from_u32(0xCEE8FAFF),
    AnsiColor::from_u32(0xFFE9E9FF),
    AnsiColor::from_u32(0xE6F4E8FF),
    AnsiColor::from_u32(0xFCF8F8FF),
);

pub(super) fn light_mode_colors() -> TerminalColors {
    TerminalColors::new(LIGHT_MODE_NORMAL_COLORS, LIGHT_MODE_BRIGHT_COLORS)
}

pub(super) fn dark_mode_colors() -> TerminalColors {
    TerminalColors::new(DARK_MODE_NORMAL_COLORS, DARK_MODE_BRIGHT_COLORS)
}

pub(super) fn solarized_light_colors() -> TerminalColors {
    TerminalColors::new(SOLARIZED_LIGHT_NORMAL_COLORS, SOLARIZED_LIGHT_BRIGHT_COLORS)
}

pub(super) fn solarized_dark_colors() -> TerminalColors {
    TerminalColors::new(SOLARIZED_DARK_NORMAL_COLORS, SOLARIZED_DARK_BRIGHT_COLORS)
}

pub(super) fn dracula_colors() -> TerminalColors {
    TerminalColors::new(DRACULA_NORMAL_COLORS, DRACULA_BRIGHT_COLORS)
}

pub(super) fn phenomenon_colors() -> TerminalColors {
    TerminalColors::new(PHENOMENON_NORMAL_COLORS, PHENOMENON_BRIGHT_COLORS)
}

pub(super) fn gruvbox_dark_colors() -> TerminalColors {
    TerminalColors::new(GRUVBOX_DARK_NORMAL_COLORS, GRUVBOX_DARK_BRIGHT_COLORS)
}

pub(super) fn gruvbox_light_colors() -> TerminalColors {
    TerminalColors::new(GRUVBOX_LIGHT_NORMAL_COLORS, GRUVBOX_LIGHT_BRIGHT_COLORS)
}

pub(super) fn solarflare_colors() -> TerminalColors {
    TerminalColors::new(SOLARFLARE_NORMAL_COLORS, SOLARFLARE_BRIGHT_COLORS)
}

pub(super) fn adeberry_colors() -> TerminalColors {
    TerminalColors::new(ADEBERRY_NORMAL_COLORS, ADEBERRY_BRIGHT_COLORS)
}

pub(super) fn vintage_dune_colors() -> TerminalColors {
    TerminalColors::new(VINTAGE_DUNE_NORMAL_COLORS, VINTAGE_DUNE_BRIGHT_COLORS)
}

pub(super) fn sage_meadow_colors() -> TerminalColors {
    TerminalColors::new(SAGE_MEADOW_NORMAL_COLORS, SAGE_MEADOW_BRIGHT_COLORS)
}

pub(super) fn mint_halo_colors() -> TerminalColors {
    TerminalColors::new(MINT_HALO_NORMAL_COLORS, MINT_HALO_BRIGHT_COLORS)
}

pub(super) fn spring_sorbet_colors() -> TerminalColors {
    TerminalColors::new(SPRING_SORBET_NORMAL_COLORS, SPRING_SORBET_BRIGHT_COLORS)
}

pub(super) fn lavender_hush_colors() -> TerminalColors {
    TerminalColors::new(LAVENDER_HUSH_NORMAL_COLORS, LAVENDER_HUSH_BRIGHT_COLORS)
}

pub(super) fn peach_night_colors() -> TerminalColors {
    TerminalColors::new(PEACH_NIGHT_NORMAL_COLORS, PEACH_NIGHT_BRIGHT_COLORS)
}

pub(super) fn bluebell_moon_colors() -> TerminalColors {
    TerminalColors::new(BLUEBELL_MOON_NORMAL_COLORS, BLUEBELL_MOON_BRIGHT_COLORS)
}

pub(super) fn rose_milk_colors() -> TerminalColors {
    TerminalColors::new(ROSE_MILK_NORMAL_COLORS, ROSE_MILK_BRIGHT_COLORS)
}

/// Default bundled themes
pub fn dark_theme() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x000000FF)),
        ColorU::from_u32(0xffffffff),
        Fill::Solid(ColorU::from_u32(0x19AAD8FF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        None,
        Some("Dark".to_string()),
    )
}

pub fn light_theme() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::white()),
        ColorU::new(17, 17, 17, OPAQUE),
        Fill::Solid(ColorU::from_u32(0x00c2ffff)),
        None,
        Some(Details::Lighter),
        light_mode_colors(),
        None,
        Some("Light".to_string()),
    )
}

pub(super) fn dracula() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x282A36FF)),
        ColorU::from_u32(0xF8F8F2FF),
        Fill::Solid(ColorU::from_u32(0xFF79C6FF)),
        None,
        Some(Details::Darker),
        dracula_colors(),
        None,
        Some("Dracula".to_string()),
    )
}

pub(super) fn solarized_light() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0xFDF6E3FF)),
        ColorU::from_u32(0x586E75FF),
        Fill::Solid(ColorU::from_u32(0x66B5A9FF)),
        None,
        Some(Details::Lighter),
        solarized_light_colors(),
        None,
        Some("Solarized Light".to_string()),
    )
}

pub(super) fn solarized_dark() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x002B36FF)),
        ColorU::from_u32(0xF8F8F2FF),
        Fill::Solid(ColorU::from_u32(0xCB4B16FF)),
        None,
        Some(Details::Darker),
        solarized_dark_colors(),
        None,
        Some("Solarized Dark".to_string()),
    )
}

pub(super) fn gruvbox_dark() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x282828FF)),
        ColorU::from_u32(0xEBDBB2FF),
        Fill::Solid(ColorU::from_u32(0xFC802DFF)),
        None,
        Some(Details::Darker),
        gruvbox_dark_colors(),
        None,
        Some("Gruvbox Dark".to_string()),
    )
}

pub(super) fn gruvbox_light() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0xFBF1C7FF)),
        ColorU::from_u32(0x3C3836FF),
        Fill::Solid(ColorU::from_u32(0xAD3B14FF)),
        None,
        Some(Details::Lighter),
        gruvbox_light_colors(),
        None,
        Some("Gruvbox Light".to_string()),
    )
}

/// Bundled gradient themes
pub(super) fn cyber_wave() -> WarpTheme {
    WarpTheme::new(
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::black().blend(&coloru_with_opacity(ColorU::from_u32(0x00C2FFFF), 20)),
            ColorU::black(),
        )),
        ColorU::white(),
        Fill::HorizontalGradient(HorizontalGradient::new(
            ColorU::from_u32(0x007972FF),
            ColorU::from_u32(0x7B008FFF),
        )),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        None,
        Some("Cyber Wave".to_string()),
    )
}

pub(super) fn willow_dream() -> WarpTheme {
    WarpTheme::new(
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::from_u32(0x206169FF),
            ColorU::from_u32(0x022F27FF),
        )),
        ColorU::white(),
        Fill::HorizontalGradient(HorizontalGradient::new(
            ColorU::from_u32(0xF9AEA8FF),
            ColorU::from_u32(0xDD6258FF),
        )),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        None,
        Some("Willow Dream".to_string()),
    )
}

pub(super) fn fancy_dracula() -> WarpTheme {
    WarpTheme::new(
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::from_u32(0x252630FF),
            ColorU::from_u32(0x3D3F4FFF),
        )),
        ColorU::white(),
        Fill::HorizontalGradient(HorizontalGradient::new(
            ColorU::from_u32(0xBCA1F6FF),
            ColorU::from_u32(0xA3E7FCFF),
        )),
        None,
        Some(Details::Darker),
        dracula_colors(),
        None,
        Some("Fancy Dracula".to_string()),
    )
}

pub(super) fn phenomenon() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x121212FF)),
        ColorU::from_u32(0xFAF9F6FF),
        Fill::Solid(ColorU::from_u32(0x2E5D9EFF)),
        None,
        Some(Details::Darker),
        phenomenon_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/phenomenon_bg.jpg"),
            opacity: 100,
        }),
        Some("Phenomenon".to_string()),
    )
}

/// Bundled themes with background images
pub(super) fn jellyfish() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x1B1718FF)),
        ColorU::white(),
        Fill::Solid(ColorU::from_u32(0x538682FF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/jellyfish_bg.jpg"),
            opacity: 30,
        }),
        Some("Jellyfish".to_string()),
    )
}

pub(super) fn koi() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x211719FF)),
        ColorU::white(),
        Fill::Solid(ColorU::from_u32(0xFF3131FF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/koi_bg.jpg"),
            opacity: 30,
        }),
        Some("Koi".to_string()),
    )
}

pub(super) fn leafy() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::black()),
        ColorU::white(),
        Fill::Solid(ColorU::from_u32(0x55972DFF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/leafy_bg.jpg"),
            opacity: 30,
        }),
        Some("Leafy".to_string()),
    )
}

pub(super) fn marble() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0xE3E3E3FF)),
        ColorU::black(),
        Fill::Solid(ColorU::from_u32(0x585858FF)),
        None,
        Some(Details::Lighter),
        light_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/marble_bg.jpg"),
            opacity: 50,
        }),
        Some("Marble".to_string()),
    )
}

pub(super) fn pink_city() -> WarpTheme {
    let details = CustomDetails {
        ..CustomDetails::lighter_details()
    };
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0xFBEFF6FF)),
        ColorU::black(),
        Fill::Solid(ColorU::from_u32(0xE10087FF)),
        None,
        Some(Details::Custom(details)),
        light_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/pink_city_bg.jpg"),
            opacity: 40,
        }),
        Some("Pink City".to_string()),
    )
}

pub(super) fn snowy() -> WarpTheme {
    WarpTheme::new(
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::from_u32(0xFFFFFFFF),
            ColorU::from_u32(0xDEE6EBFF),
        )),
        ColorU::black(),
        Fill::Solid(ColorU::from_u32(0x647E90FF)),
        None,
        Some(Details::Lighter),
        light_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/snowy_bg.jpg"),
            opacity: 20,
        }),
        Some("Snowy".to_string()),
    )
}

pub(super) fn red_rock() -> WarpTheme {
    WarpTheme::new(
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::from_u32(0x211719FF)
                .blend(&coloru_with_opacity(ColorU::from_u32(0x4C3435FF), 45)),
            ColorU::from_u32(0x211719FF)
                .blend(&coloru_with_opacity(ColorU::from_u32(0xD3032FF), 45)),
        )),
        ColorU::white(),
        Fill::Solid(ColorU::from_u32(0x9F4147FF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/red_rock_bg.jpg"),
            opacity: 30,
        }),
        Some("Red Rock".to_string()),
    )
}

pub(super) fn dark_city() -> WarpTheme {
    WarpTheme::new(
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::from_u32(0x01181FFF)
                .blend(&coloru_with_opacity(ColorU::from_u32(0x1A363FFF), 45)),
            ColorU::from_u32(0x01181FFF)
                .blend(&coloru_with_opacity(ColorU::from_u32(0x1A4551FF), 45)),
        )),
        ColorU::white(),
        Fill::Solid(ColorU::from_u32(0xE9072DFF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/dark_city_bg.jpg"),
            opacity: 20,
        }),
        Some("Dark City".to_string()),
    )
}

pub(super) fn sent_referral_reward() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x334567FF)),
        ColorU::white(),
        Fill::Solid(ColorU::from_u32(0xCD51FFFF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/sent_referral_reward_bg.jpg"),
            opacity: 100,
        }),
        Some("Warp Referral".to_string()),
    )
}

pub(super) fn solar_flare() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x1B1C18FF)),
        ColorU::from_u32(0xDDE6EEFF),
        Fill::Solid(ColorU::from_u32(0x34895CFF)),
        None,
        Some(Details::Darker),
        solarflare_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/solarflare_bg.jpg"),
            opacity: 20,
        }),
        Some("Solar Flare".to_string()),
    )
}

pub(super) fn adeberry() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x1D2022FF)),
        ColorU::from_u32(0xE4EEF5FF),
        Fill::Solid(ColorU::from_u32(0x6C96B4FF)),
        None,
        Some(Details::Darker),
        adeberry_colors(),
        None,
        Some("Adeberry".to_string()),
    )
}

pub(super) fn vintage_dune() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x1B1A24FF)),
        ColorU::from_u32(0xF3EAC8FF),
        Fill::Solid(ColorU::from_u32(0x767F9EFF)),
        None,
        Some(Details::Darker),
        vintage_dune_colors(),
        None,
        Some("Vintage Dune".to_string()),
    )
}

pub(super) fn sage_meadow() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x16251FFF)),
        ColorU::from_u32(0xE6F2DDFF),
        Fill::Solid(ColorU::from_u32(0x88BDA4FF)),
        None,
        Some(Details::Darker),
        sage_meadow_colors(),
        None,
        Some("Sage Meadow".to_string()),
    )
}

pub(super) fn mint_halo() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x171A35FF)),
        ColorU::from_u32(0xD9F9DFFF),
        Fill::Solid(ColorU::from_u32(0x9FA1FFFF)),
        None,
        Some(Details::Darker),
        mint_halo_colors(),
        None,
        Some("Mint Halo".to_string()),
    )
}

pub(super) fn spring_sorbet() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x172B32FF)),
        ColorU::from_u32(0xF6FFDCFF),
        Fill::Solid(ColorU::from_u32(0xF9B2D7FF)),
        None,
        Some(Details::Darker),
        spring_sorbet_colors(),
        None,
        Some("Spring Sorbet".to_string()),
    )
}

pub(super) fn lavender_hush() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x211D32FF)),
        ColorU::from_u32(0xF2EAE0FF),
        Fill::Solid(ColorU::from_u32(0xBDA6CEFF)),
        None,
        Some(Details::Darker),
        lavender_hush_colors(),
        None,
        Some("Lavender Hush".to_string()),
    )
}

pub(super) fn peach_night() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x2C2024FF)),
        ColorU::from_u32(0xFFF0BEFF),
        Fill::Solid(ColorU::from_u32(0xFF9A86FF)),
        None,
        Some(Details::Darker),
        peach_night_colors(),
        None,
        Some("Peach Night".to_string()),
    )
}

pub(super) fn bluebell_moon() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x182231FF)),
        ColorU::from_u32(0xF3E3D0FF),
        Fill::Solid(ColorU::from_u32(0x81A6C6FF)),
        None,
        Some(Details::Darker),
        bluebell_moon_colors(),
        None,
        Some("Bluebell Moon".to_string()),
    )
}

pub(super) fn rose_milk() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x2B2029FF)),
        ColorU::from_u32(0xFBEFEFFF),
        Fill::Solid(ColorU::from_u32(0xF5AFAFFF)),
        None,
        Some(Details::Darker),
        rose_milk_colors(),
        None,
        Some("Rose Milk".to_string()),
    )
}

pub(super) fn received_referral_reward() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0xFFFFFFFF)),
        ColorU::black(),
        Fill::Solid(ColorU::from_u32(0xCD51FFFF)),
        None,
        Some(Details::Lighter),
        light_mode_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/received_referral_reward_bg.jpg"),
            opacity: 100,
        }),
        Some("Received Referral Reward".to_string()),
    )
}

const DELTARUNE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x050520FF),
    AnsiColor::from_u32(0xDC1510FF),
    AnsiColor::from_u32(0x33A56CFF),
    AnsiColor::from_u32(0xFBCE3CFF),
    AnsiColor::from_u32(0x6A7BC4FF),
    AnsiColor::from_u32(0xA017D0FF),
    AnsiColor::from_u32(0x77E0FFFF),
    AnsiColor::from_u32(0xFFFFFFFF),
);
const DELTARUNE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x8888AAFF),
    AnsiColor::from_u32(0xEB0095FF),
    AnsiColor::from_u32(0x40E4D4FF),
    AnsiColor::from_u32(0xF4A731FF),
    AnsiColor::from_u32(0x75FBEDFF),
    AnsiColor::from_u32(0xF983D8FF),
    AnsiColor::from_u32(0xBBFFFCFF),
    AnsiColor::from_u32(0xC7E3F2FF),
);

pub(super) fn deltarune_colors() -> TerminalColors {
    TerminalColors::new(DELTARUNE_NORMAL_COLORS, DELTARUNE_BRIGHT_COLORS)
}

pub(super) fn deltarune() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x0B0B3BFF)),
        ColorU::from_u32(0xFFFFFFFF),
        Fill::Solid(ColorU::from_u32(0xEB0095FF)),
        None,
        Some(Details::Darker),
        deltarune_colors(),
        None,
        Some("Deltarune".to_string()),
    )
}
