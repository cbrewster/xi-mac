extern crate xi_core_lib;
extern crate xi_rpc;

use std::io;
use xi_core_lib::MainState;
use xi_rpc::RpcLoop;

fn main() {
    let mut state = MainState::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut rpc_looper = RpcLoop::new(stdout);

    rpc_looper.mainloop(|| stdin.lock(), &mut state);
}
