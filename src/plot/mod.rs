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
            .add_cmd::<ColoredxyExampleCmd>()
            .add_cmd::<CubicFunctionCmd>();
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
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let (cmd_name, args) = args.unwrap().subcommand();
        self.m_disp.run(cmd_name, args)
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
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
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
        Ok(())
    }
}

struct CubicFunctionCmd;

impl CubicFunctionCmd {
    fn calc(x: f64) -> f64 {
        x * x * x
    }
}

impl common::Command for CubicFunctionCmd {
    fn create() -> Box<CubicFunctionCmd> { Box::<_>::new(CubicFunctionCmd {}) }
    fn name() -> &'static str { "cubic_function" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b>
    {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(clap::Arg::with_name("x_min")
                    .long("x_min")
                    .default_value("-10")
                    .takes_value(true)
                    .allow_hyphen_values(true))
                .arg(clap::Arg::with_name("x_max")
                    .long("x_max")
                    .default_value("10")
                    .takes_value(true)
                    .allow_hyphen_values(true))
                .arg(clap::Arg::with_name("step")
                    .long("step")
                    .default_value("0.1")
                    .takes_value(true));
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let args = args.unwrap();
        let x_min = args.value_of("x_min").unwrap().parse::<f64>().unwrap();
        let x_max = args.value_of("x_max").unwrap().parse::<f64>().unwrap();
        let step = args.value_of("step").unwrap().parse::<f64>().unwrap();

        let mut x = Vec::<f64>::new();
        let mut y = Vec::<f64>::new();

        let mut y_min = CubicFunctionCmd::calc(x_min);
        let mut y_max = y_min;

        let mut i = x_min + step;
        while i - step < x_max {
            let y_i = CubicFunctionCmd::calc(i);

            if y_min > y_i { y_min = y_i; }
            if y_max < y_i { y_max = y_i; }

            x.push(i);
            y.push(y_i);

            i += step;
        };

        let xy = zip2(&x, &y);

        // Creates a new plot builder
        let mut pb = PlotBuilder2D::new();
        pb.min_x = Some(x_min);
        pb.max_x = Some(x_max);
        pb.min_y = Some(y_min);
        pb.max_y = Some(y_max);

        // Adds the sin plot and the linear plot with custom colors
        pb.add_color_xy(xy, [1.0, 0.0, 0.0, 1.0]);

        let mut plt = Plotter::new();
        plt.plot2d(pb);
        plt.join();
        Ok(())
    }
}