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
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let (cmd_name, args) = args.unwrap().subcommand();
        self.m_disp.run(cmd_name, args)
    }
}

struct ListAllConnectionCmd;

impl common::Command for ListAllConnectionCmd {
    fn create() -> Box<Self> { Box::<_>::new(Self {}) }
    fn name() -> &'static str { "list_all_con" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(clap::Arg::with_name("ipv4").long("ipv4"))
                .arg(clap::Arg::with_name("ipv6").long("ipv6"))
                .arg(clap::Arg::with_name("tcp").long("tcp"))
                .arg(clap::Arg::with_name("upd").long("udp"))
                .arg(clap::Arg::with_name("port").long("port").takes_value(true));
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let args = args.unwrap();

        let af_flags = ListAllConnectionCmd::get_ip_ver(args);
        let proto_flags = ListAllConnectionCmd::get_ip_prot(args);
        let port =
            if args.is_present("port") {
                Some(args.value_of("port").unwrap().parse::<u16>().unwrap())
            } else {
                None
            };


        let sockets_info = get_sockets_info(af_flags, proto_flags)?;
        for si in sockets_info {
            if !ListAllConnectionCmd::is_port_match(&si, &port) { continue; }

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
        Ok(())
    }
}

impl ListAllConnectionCmd {
    fn get_ip_ver(args: &clap::ArgMatches) -> AddressFamilyFlags {
        if !args.is_present("ipv4") && !args.is_present("ipv6") {
            return AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        }
        if args.is_present("ipv4") && args.is_present("ipv6") {
            return AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        }

        if args.is_present("ipv4") { return AddressFamilyFlags::IPV4; }
        return AddressFamilyFlags::IPV6;
    }
    fn get_ip_prot(args: &clap::ArgMatches) -> ProtocolFlags {
        if !args.is_present("tcp") && !args.is_present("udp") {
            return ProtocolFlags::TCP | ProtocolFlags::UDP;
        }
        if args.is_present("tcp") && args.is_present("udp") {
            return ProtocolFlags::TCP | ProtocolFlags::UDP;
        }

        if args.is_present("tcp") { return ProtocolFlags::TCP; }
        return ProtocolFlags::UDP;
    }
    fn is_port_match(si: &SocketInfo, port: &Option<u16>) -> bool {
        if port.is_none() { return true; }
        let port = port.unwrap();
        match &si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                return tcp_si.local_port == port || tcp_si.remote_port == port;
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                return udp_si.local_port == port;
            }
        }
    }
}
