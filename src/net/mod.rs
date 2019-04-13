extern crate netstat;

use netstat::*;

use crate::common;

pub struct NetDispatcher {
    m_disp: common::Dispatcher,
}

impl NetDispatcher {
    fn new() -> Self {
        let mut disp = Self {
            m_disp: common::Dispatcher::new()
        };
        disp.m_disp
            .add_cmd::<ListAllConnectionCmd>();
        disp
    }
}

impl common::Command for NetDispatcher {
    fn create() -> Box<Self> {
        Box::<>::new(Self::new())
    }
    fn name() -> &'static str { "net" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let fs_sub_cmd = clap::App::new(Self::name());
        let fs_sub_cmd = self.m_disp.fill_subcommands(fs_sub_cmd);
        app.subcommand(fs_sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) {
        let (cmd_name, args) = args.unwrap().subcommand();
        self.m_disp.run(cmd_name, args);
    }
}

struct ListAllConnectionCmd;

impl common::Command for ListAllConnectionCmd {
    fn create() -> Box<Self> { Box::<_>::new(Self {}) }
    fn name() -> &'static str { "list_all_con" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let sub_cmd = clap::App::new(Self::name());
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
        for si in sockets_info {
            match si.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp_si) => println!(
                    "TCP {}:{} -> {}:{} {:?} - {}",
                    tcp_si.local_addr,
                    tcp_si.local_port,
                    tcp_si.remote_addr,
                    tcp_si.remote_port,
                    si.associated_pids,
                    tcp_si.state
                ),
                ProtocolSocketInfo::Udp(udp_si) => println!(
                    "UDP {}:{} -> *:* {:?}",
                    udp_si.local_addr, udp_si.local_port, si.associated_pids
                ),
            }
        }
    }
}
