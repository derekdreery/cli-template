use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// How verbose to be
    #[structopt(parse(from_occurrences))]
    pub verbosity: u32,
    // Add your arguments here...
}

/// Pass your run function here to wrap it in a main.
///
/// It should have the signature `fn run(opts: config::Opt) -> Result<(), E>` for any type `E` that impls
/// `std::error::Error`.
#[macro_export]
macro_rules! main {
    ($main_name:tt) => {
        /// Wrap the run method so we can pass it command line args, setup logging, and handle errors
        /// gracefully.
        fn main() {
            let opts = Opt::from_args();
            crate::config::setup_logger(opts.verbosity);
            if let Err(err) = $main_name(opts) {
                log::error!("{}", err);
                for e in err.chain().skip(1) {
                    log::error!("caused by {}", e);
                }
            }
        }
    };
}

/// Make the logger match our verbosity. This is custom because we don't want to see all messages
/// from other packages, only `jack-volume`.
pub fn setup_logger(verbosity: u32) {
    use log::LevelFilter;
    let level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    // this will no longer work if you have a binary with a name different to your package name. In
    // that case delete the next statement and add in your binary name directly in place of the
    // `package_name`.
    let package_name = env!("CARGO_PKG_NAME").replace('-', "_");
    env_logger::builder()
        .filter(None, LevelFilter::Debug)
        .filter(Some(&package_name), level)
        .init()
}
