use humantime;
use log::{debug, error, info, log_enabled, warn, Level, LevelFilter};
use moon::{start, Frontend, UpMsgRequest, *};
use shared::{DownMsg, Message, UpMsg};

async fn frontend() -> Frontend {
    Frontend::new().title("MZ Components").default_styles(false)
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "backend=debug");
    start(frontend, up_msg_handler, |_| {}).await
}


async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    debug!("Up Message is {:?}", req);

}
