use std::{convert::Infallible, path::PathBuf};

pub struct Args {
    pub program: PathBuf,
    pub colors: Colors,
}

pub struct Colors {
    pub foreground: u32,
    pub background: u32,
}

pub fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = Args {
        program: pargs.free_from_fn::<PathBuf, Infallible>(|x| Ok(x.into()))?,
        colors: Colors {
            foreground: pargs
                .opt_value_from_fn("--foreground", parse_color)?
                .unwrap_or(0xFF_FF_FF),
            background: pargs
                .opt_value_from_fn("--background", parse_color)?
                .unwrap_or(0x00_00_00),
        },
    };

    Ok(args)
}

fn parse_color(s: &str) -> Result<u32, &'static str> {
    let s = s.trim_start_matches("0x");
    u32::from_str_radix(s, 16).map_err(|_| "failed to parse color")
}
