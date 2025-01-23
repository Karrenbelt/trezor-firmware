// mod font_roboto_bold_20;
// mod font_roboto_regular_20;
mod font_robotomono_medium_20;
mod font_tthoves_bold_17;
mod font_tthoves_demibold_21;
mod font_tthoves_regular_21;

// use font_roboto_bold_20::Font_Roboto_Bold_20_info;
// use font_roboto_regular_20::Font_Roboto_Regular_20_info;
use font_robotomono_medium_20::Font_RobotoMono_Medium_20_info;
use font_tthoves_bold_17::Font_TTHoves_Bold_17_upper_info;
use font_tthoves_demibold_21::Font_TTHoves_DemiBold_21_info;
use font_tthoves_regular_21::Font_TTHoves_Regular_21_info;

pub const FONT_NORMAL: crate::ui::display::Font = &Font_TTHoves_Regular_21_info;
// TODO: remove BOLD (points to the same font as BOLD_UPPER)
pub const FONT_BOLD: crate::ui::display::Font = &Font_TTHoves_Bold_17_upper_info;
pub const FONT_BOLD_UPPER: crate::ui::display::Font = &Font_TTHoves_Bold_17_upper_info;
pub const FONT_DEMIBOLD: crate::ui::display::Font = &Font_TTHoves_DemiBold_21_info;
pub const FONT_MONO: crate::ui::display::Font = &Font_RobotoMono_Medium_20_info;
// TODO: remove SUB (points to the same font as DEMIBOLD)
pub const FONT_SUB: crate::ui::display::Font = &Font_TTHoves_DemiBold_21_info;
