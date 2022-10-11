use anyhow::Result;
use clap::Parser;
use log::debug;
use x11rb::{
    connection::Connection,
    protocol::xproto::{get_property, AtomEnum, ConfigureWindowAux, ConnectionExt, StackMode},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    wname: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    debug!("on screen #{}", screen_num);
    raise_zoom_windows(&conn, screen.root, &args.wname)
}

fn raise_zoom_windows<Conn>(conn: &Conn, root_win_id: u32, window_name: &str) -> Result<()>
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

        if name == window_name {
            debug!("raising window with name {}, id {}", name, window_id);
            let values = ConfigureWindowAux::default().stack_mode(StackMode::ABOVE);
            conn.configure_window(window_id, &values)?;
            break;
        }
    }

    Ok(())
}
