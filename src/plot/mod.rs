extern crate dataplotlib;

use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;
use dataplotlib::util::{linspace, zip2};

use crate::common;

pub struct PlotDispatcher {
    m_disp: common::Dispatcher,
}

impl PlotDispatcher {
    fn new() -> PlotDispatcher {
        let mut disp = PlotDispatcher {
            m_disp: common::Dispatcher::new()
        };
        disp.m_disp
            .add_cmd::<ColoredxyExampleCmd>();
        disp
    }
}

impl common::Command for PlotDispatcher {
    fn create() -> Box<PlotDispatcher> {
        Box::<>::new(PlotDispatcher::new())
    }
    fn name() -> &'static str { "plot" }
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


struct ColoredxyExampleCmd;

impl common::Command for ColoredxyExampleCmd {
    fn create() -> Box<ColoredxyExampleCmd> { Box::<_>::new(ColoredxyExampleCmd {}) }
    fn name() -> &'static str { "coloredxy_example" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b>
    {
        let sub_cmd = clap::App::new(Self::name());
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>)
    {
        let x = linspace(0, 10, 100);

        let y_sin = x.iter().map(|x| x.sin()).collect();
        let xy_sin = zip2(&x, &y_sin);

        let xy_lin = zip2(&x, &x);

        // Creates a new plot builder
        let mut pb = PlotBuilder2D::new();

        // Adds the sin plot and the linear plot with custom colors
        pb.add_color_xy(xy_sin, [1.0, 0.0, 0.0, 1.0]);
        pb.add_color_xy(xy_lin, [0.0, 0.0, 1.0, 1.0]);

        let mut plt = Plotter::new();
        plt.plot2d(pb);
        plt.join();
    }
}