use class_file::jaustp_summary_print;
use class_file::raw_string;
use jaust::class_file;

use clap::Parser;
use jaust::class_file::JavapOptions;

#[derive(Parser)]
struct Opts {
    #[clap(short, long, help = "Prints all classes and members")]
    pub private: bool,

    #[clap(short, long, help = "Prints full class_file structure")]
    pub raw: bool,

    #[clap(short, long, help = "Prints method bytecodes")]
    pub code: bool,

    /// input file
    pub file: String,
}

impl Into<JavapOptions> for Opts {
    fn into(self) -> JavapOptions {
        JavapOptions {
            private: self.private,
            code: self.code,
        }
    }
}

fn main() {
    let ops = Opts::parse();

    let cf = class_file::read_class_file(&ops.file).unwrap();
    if ops.raw {
        println!("{}", raw_string(&cf));
        return;
    }

    jaustp_summary_print(&cf, &ops.into());
}
