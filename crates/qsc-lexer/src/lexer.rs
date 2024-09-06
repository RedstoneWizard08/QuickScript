use std::{fs, path::PathBuf};

use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::{error::Rich, input::Input, Parser};
use ron::ser::PrettyConfig;

use crate::{func::funcs, token::tokens};

pub fn parse_tokens<'s>(file: String, src: String, output: PathBuf) {
    debug!("Parsing tokens...");

    let (tokens, errs) = tokens().parse(src.as_str()).into_output_errors();

    debug!("Displaying errors...");

    errs.into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .for_each(|e| {
            Report::build(ReportKind::Error, file.clone(), e.span().start)
                .with_message(e.to_string())
                .with_label(
                    Label::new((file.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((file.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(file.clone(), src.clone())]))
                .unwrap()
        });

    debug!("Emitting tokens...");

    let tokens = tokens
        .unwrap_or_default()
        .iter()
        .map(|v| v.0)
        .collect::<Vec<_>>();

    let data = ron::ser::to_string_pretty(&tokens, PrettyConfig::new()).unwrap();

    if !output.parent().unwrap().exists() {
        fs::create_dir_all(output.parent().unwrap()).unwrap();
    }

    fs::write(output, data).unwrap();
}

pub fn parse<'s>(file: String, src: String) {
    debug!("Parsing tokens...");

    let (tokens, mut errs) = tokens().parse(src.as_str()).into_output_errors();

    debug!("Parsing AST...");

    let parse_errs = if let Some(tokens) = &tokens {
        let (ast, parse_errs) = funcs()
            .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
            .into_output_errors();

        debug!("Validating AST...");

        if let Some((funcs, file_span)) = ast.filter(|_| errs.len() + parse_errs.len() == 0) {
            if let Some(main) = funcs.get("main") {
                if !main.0.args.is_empty() {
                    errs.push(Rich::custom(
                        main.1,
                        "The main function cannot have arguments".to_string(),
                    ))
                } else {
                    // TODO: Compile
                }
            } else {
                errs.push(Rich::custom(
                    file_span,
                    "Programs need a main function but none was found".to_string(),
                ));
            }
        }

        parse_errs
    } else {
        Vec::new()
    };

    debug!("Displaying errors...");

    errs.into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .chain(
            parse_errs
                .into_iter()
                .map(|e| e.map_token(|tok| tok.to_string())),
        )
        .for_each(|e| {
            Report::build(ReportKind::Error, file.clone(), e.span().start)
                .with_message(e.to_string())
                .with_label(
                    Label::new((file.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((file.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(file.clone(), src.clone())]))
                .unwrap()
        });
}
