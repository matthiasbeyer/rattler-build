pub use self::{error::ParsingError, jinja::Jinja, stage1::RawRecipe, stage2::Recipe};

pub mod stage1;
pub mod stage2;

pub mod custom_yaml;
pub mod error;
pub mod jinja;

#[cfg(test)]
#[cfg_attr(test, macro_export)]
macro_rules! assert_miette_snapshot {
    ($value:expr, @$snapshot:literal) => {{
        let mut value = String::new();
        ::miette::GraphicalReportHandler::new_themed(::miette::GraphicalTheme::unicode_nocolor())
            .with_width(80)
            .render_report(&mut value, &$value)
            .unwrap();
        ::insta::assert_snapshot!(value, stringify!($value), @$snapshot);
    }};
    ($name:expr, $value:expr) => {{
        let mut value = String::new();
        ::miette::GraphicalReportHandler::new_themed(::miette::GraphicalTheme::unicode_nocolor())
            .with_width(80)
            .render_report(&mut value, &$value)
            .unwrap();
        ::insta::assert_snapshot!(Some($name), value, stringify!($value));
    }};
    ($value:expr) => {{
        let mut value = String::new();
        ::miette::GraphicalReportHandler::new_themed(::miette::GraphicalTheme::unicode_nocolor())
            .with_width(80)
            .render_report(&mut value, &$value)
            .unwrap();
        ::insta::assert_snapshot!(::insta::_macro_support::AutoName, value, stringify!($value));
    }};
}
