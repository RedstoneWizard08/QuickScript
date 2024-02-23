use clap::builder::{
    styling::{AnsiColor, Color, Style},
    Styles,
};

pub fn get_styles() -> Styles {
    Styles::styled()
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .error(
            Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Red)))
                .bold(),
        )
        .usage(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightCyan))),
        )
        .literal(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue))),
        )
        .placeholder(Style::new())
        .valid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
}
