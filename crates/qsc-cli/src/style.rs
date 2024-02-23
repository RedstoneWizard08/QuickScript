use clap::builder::{
    styling::{AnsiColor, Color, Style},
    Styles,
};

pub fn get_styles() -> Styles {
    Styles::styled()
        .header(Style::new().bold().underline())
        .error(
            Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Red)))
                .bold(),
        )
        .usage(Style::new().bold().underline())
        .literal(Style::new().bold())
        .placeholder(Style::new())
        .valid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
}
