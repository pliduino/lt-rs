# Lt-rs

Besides session being renamed to LtSession there's not much change on how to use it. Refer to https://www.libtorrent.org/tutorial-ref.html for tutorials.

This crate requires boost and b2 (boost-build) to compile libtorrent.

## Examples

Basic example of adding a torrent:

```rust
use lt_rs::session::LtSession;
use lt_rs::add_torrent_params::AddTorrentParams;
use lt_rs::alert::{Alert, TorrentFinishedAlert};

fn main() {
    let args = std::env::args().collect();
    if args.len() <= 1 {
        return; // No arguments provided
    }
    let mut session = LtSession::new();
    let atp = AddTorrentParams::parse_magnet_uri(args[1]);
    atp.set_path("./downloads");
    session.add_torrent(atp);
    
    loop {
        session.pop_alerts();
        for alert in &session.alerts() {
            match alert {
                Alert::TorrentAlert(alert) => match alert {
                    TorrentAlert::TorrentFinished(alert) => {
                        return;
                    },
                    _ => (),
                }
                _ => (),
            }
        }
    }
}
```
