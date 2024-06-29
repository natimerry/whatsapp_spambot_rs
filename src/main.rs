use clap::builder::Str;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use clap::Parser;
use log::error;
use tracing::{debug, info, Level, warn};
use tracing_subscriber::EnvFilter;

mod csv_parser;
mod whatsapp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the chromedriver
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    // Port of chromedriver
    #[arg(short,long,default_value = "9515")]
    port:String,
    //path
    #[arg(short,long,default_value = "form.csv",alias = "path")]
    file_path:String,

    #[arg(short = 'D',long,default_value = "dump.txt",alias = "dump")]
    dump_path:String,

    #[arg(long)]
    profile:String,

}



#[tokio::main]
async fn main(){
    let debug_file =
        tracing_appender::rolling::hourly("./logs/", "debug")
            .with_max_level(tracing::Level::INFO);

    let warn_file = tracing_appender::rolling::hourly("./logs/", "warnings")
        .with_max_level(tracing::Level::WARN);
    let all_files = debug_file.and(warn_file);

    tracing_subscriber::registry()
        .with(EnvFilter::from_env("SPAMMER_LOG_LEVEL"))
        // .with(console_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(all_files)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_ansi(true)
                .with_writer(std::io::stdout.with_max_level(Level::DEBUG))
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let args = Args::parse();
    debug!("{:?}",args);

    let url = format!("http://{}:{}",args.address,args.port);
    let all_members = whatsapp::Sender::new(args.file_path,args.dump_path,url)
        .set_profile(args.profile);
    match all_members.send_msgs().await{
        Ok(_) => {
            info!("Finished sending messages!!");
        }
        Err(e) => {
            error!("{:?}",e);
            error!("Unable to send messages");
        }
    }

}
