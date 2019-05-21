#[cfg(not(feature = "format-debug-output"))]
// Based on https://github.com/rust-lang/rustfmt/blob/master/src/bin/main.rs
pub fn code_gen_debug(input: String) {
    println!("{}", input);
}

#[cfg(feature = "format-debug-output")]
pub use self::print_with_rustfmt_nightly::code_gen_debug;

#[cfg(feature = "format-debug-output")]
mod print_with_rustfmt_nightly {
    use rustfmt_nightly::{Config, EmitMode, FileLines, FileName, Input, Session, Verbosity};
    use std::io::{stdout, Write};

    // Based on https://github.com/rust-lang/rustfmt/blob/master/src/bin/main.rs
    pub fn code_gen_debug(input: String) {
        let mut config = Config::default();

        config.set().emit_mode(EmitMode::Stdout);
        config.set().verbose(Verbosity::Quiet);

        config.set().file_lines(FileLines::default());

        for f in config.file_lines().files() {
            match *f {
                FileName::Stdin => {}
                _ => eprintln!("Warning: Extra file listed in file_lines option '{}'", f),
            }
        }

        let out = &mut stdout();
        let mut session = Session::new(config, Some(out));
        format_and_emit_report(&mut session, Input::Text(input));

        if session.has_operational_errors() || session.has_parsing_errors() {
            panic!("Error formatting generated code");
        }
    }

    fn format_and_emit_report<T: Write>(session: &mut Session<T>, input: Input) {
        match session.format(input) {
            Ok(report) => {
                if report.has_warnings() {
                    eprintln!("{}", report)
                }
            }
            Err(msg) => {
                eprintln!("Error writing files: {}", msg);
                session.add_operational_error();
            }
        }
    }
}
