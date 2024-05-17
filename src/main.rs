use anyhow::Result;
use clap::Parser;
use log::{debug, info};
use x11rb::{
    connection::Connection,
    protocol::xproto::{get_property, AtomEnum, ConfigureWindowAux, ConnectionExt, StackMode},
};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of X11 window to raise
    #[arg(short, long)]
    wname_regex: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    debug!("on screen #{}", screen_num);
    let regex = Regex::new(&args.wname_regex).unwrap();

    raise_window_by_regex(&conn, screen.root, &regex)
}

fn raise_window_by_regex<Conn>(conn: &Conn, root_win_id: u32, window_name_regex: &Regex) -> Result<()>
where
    Conn: Connection,
{
    let tree = conn.query_tree(root_win_id)?.reply()?.children;
    for window_id in tree {
        let name_raw = get_property(
            conn,
            false,
            window_id,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            0,
            2048,
        )?
        .reply()?
        .value;
        let name = std::str::from_utf8(&name_raw).unwrap_or_default();
        debug!("found window with name {}, id {}", name, window_id);

        if window_name_regex.is_match(name) {
            info!("raising window with name {}, id {}", name, window_id);
            let values = ConfigureWindowAux::default().stack_mode(StackMode::ABOVE);
            conn.configure_window(window_id, &values)?;
        }
    }

    conn.flush()?;

    Ok(())
}
