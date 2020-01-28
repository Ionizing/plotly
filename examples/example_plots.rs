use plotly::charts::Axis;
use plotly::charts::Layout;
use plotly::charts::{
    Bar, BarMode, Candlestick, ErrorData, ErrorType, MarkerSymbol, Mode, Ohlc, Scatter,
};
use plotly::Plot;
use rand::Rng;

fn geometric_brownian_motion(s_0: f64, dt: f64, n: usize, drift: f64, diffusion: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Normal::new(0.0, 1.0);
    let mut v = Vec::<f64>::with_capacity(n);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..n {
        let rv = drift_factor + diffusion_factor * rng.sample(dist);
        let prod: f64 = rv * v[idx - 1];
        v.push(prod);
    }
    v
}

fn line_and_scatter_plot() {
    let mut trace1 = Scatter::new("trace1", vec![1, 2, 3, 4], vec![10, 15, 13, 17]);
    trace1.mode = Mode::Markers;
    let mut trace2 = Scatter::new("trace2", vec![2, 3, 4, 5], vec![16, 5, 11, 9]);
    trace2.mode = Mode::Lines;
    let trace3 = Scatter::new("trace3", vec![1, 2, 3, 4], vec![12, 9, 15, 12]);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    plot.show();
}

fn data_labels_hover() {
    let mut trace1 = Scatter::new("trace1", vec![1, 2, 3, 4, 5], vec![1, 6, 3, 6, 1]);
    trace1.mode = Mode::Markers;
    trace1.name = String::from("Team A");
    trace1.marker.size = 12;
    let mut trace2 = Scatter::new("trace2", vec![1.5, 2.5, 3.5, 4.5, 5.5], vec![4, 1, 7, 1, 4]);
    trace2.mode = Mode::Markers;
    trace2.name = String::from("Team B");
    trace2.marker.size = 12;

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let mut layout = Layout::new("Data Labels Hover");
    let mut xaxis = Axis::new("");
    xaxis.range = Some(vec![0.75, 5.25]);
    let mut yaxis = Axis::new("");
    yaxis.range = Some(vec![0., 8.]);
    layout.xaxis = Some(xaxis);
    layout.yaxis = Some(yaxis);
    plot.add_layout(layout);

    plot.show();
}

fn data_labels_on_the_plot() {
    let mut trace1 = Scatter::new("trace1", vec![1, 2, 3, 4, 5], vec![1, 6, 3, 6, 1]);
    trace1.mode = Mode::Markers;
    trace1.name = String::from("Team A");
    trace1.marker.size = 12;
    trace1.text = Some(vec![
        "A-1".to_owned(),
        "A-2".to_owned(),
        "A-3".to_owned(),
        "A-4".to_owned(),
        "A-5".to_owned(),
    ]);
    let mut trace2 = Scatter::new("trace2", vec![1.5, 2.5, 3.5, 4.5, 5.5], vec![4, 1, 7, 1, 4]);
    trace2.mode = Mode::Markers;
    trace2.name = String::from("Team B");
    trace2.text = Some(vec![
        "B-a".to_owned(),
        "B-b".to_owned(),
        "B-c".to_owned(),
        "B-d".to_owned(),
        "B-e".to_owned(),
    ]);
    trace2.marker.size = 12;

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let mut layout = Layout::new("Data Labels on the Plot");
    let mut xaxis = Axis::new("");
    xaxis.range = Some(vec![0.75, 5.25]);
    let mut yaxis = Axis::new("");
    yaxis.range = Some(vec![0., 8.]);
    layout.xaxis = Some(xaxis);
    layout.yaxis = Some(yaxis);
    plot.add_layout(layout);

    plot.show();
}

fn gbm_scatter_plot() {
    let n = 3_000;
    let x = (0..n).collect();
    let y = geometric_brownian_motion(100.0, 1.0 / 365.0, n, 0.15, 0.5);
    let mut t = Scatter::new("path_0", x, y);
    let mut plot = Plot::new();
    plot.add_trace(t);
    plot.show();
}

fn example_scatter_plots() {
    line_and_scatter_plot();
    data_labels_hover();
    data_labels_on_the_plot();
    gbm_scatter_plot();
}

fn basic_bar_chart() {
    let animals = vec![
        "giraffes",
        "orangutans",
        "monkeys",
    ];
    let t = Bar::new(
        "",
        animals,
        vec![20, 14, 23],
    );
    let mut plot = Plot::new();
    plot.add_trace(t);
    plot.show();
}

fn grouped_bar_chart() {
    let animals1 = vec![
        "giraffes",
        "orangutans",
        "monkeys",
    ];
    let trace1 = Bar::new(
        "SF Zoo",
        animals1,
        vec![20, 14, 23],
    );

    let animals2 = vec![
        "giraffes",
        "orangutans",
        "monkeys",
    ];
    let trace2 = Bar::new(
        "LA Zoo",
        animals2,
        vec![12, 18, 29],
    );

    let mut layout = Layout::new("");
    layout.barmode = Some(BarMode::Group);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_layout(layout);
    plot.show();
}

fn stacked_bar_chart() {
    let animals1 = vec![
        "giraffes",
        "orangutans",
        "monkeys",
    ];
    let trace1 = Bar::new(
        "SF Zoo",
        animals1,
        vec![20, 14, 23],
    );

    let animals2 = vec![
        "giraffes",
        "orangutans",
        "monkeys",
    ];
    let trace2 = Bar::new(
        "LA Zoo",
        animals2,
        vec![12, 18, 29],
    );

    let mut layout = Layout::new("");
    layout.barmode = Some(BarMode::Stack);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_layout(layout);
    plot.show();
}

fn example_bar_charts() {
    basic_bar_chart();
    grouped_bar_chart();
    stacked_bar_chart();
}

fn basic_symmetric_error_bars() {
    let mut trace1 = Scatter::new("trace1", vec![0, 1, 2], vec![6, 10, 2]);
    let mut trace1_error_y = ErrorData::new(ErrorType::Data);
    trace1_error_y.array = Some(vec![1.0, 2.0, 3.0]);
    trace1.error_y = Some(trace1_error_y);

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    plot.show();
}

fn bar_chart_with_error_bars() {
    let mut trace1 = Bar::new(
        "Control",
        vec![
            "Trial 1".to_owned(),
            "Trial 2".to_owned(),
            "Trial 3".to_owned(),
        ],
        vec![3, 6, 4],
    );
    let mut trace1_error_y = ErrorData::new(ErrorType::Data);
    trace1_error_y.array = Some(vec![1.0, 0.5, 1.5]);
    trace1.error_y = Some(trace1_error_y);

    let mut trace2 = Bar::new(
        "LA Zoo",
        vec![
            "Trial 1".to_owned(),
            "Trial 2".to_owned(),
            "Trial 3".to_owned(),
        ],
        vec![4, 7, 3],
    );
    let mut trace2_error_y = ErrorData::new(ErrorType::Data);
    trace2_error_y.array = Some(vec![0.5, 1.0, 2.0]);
    trace2.error_y = Some(trace2_error_y);

    let layout = Layout::new("");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_layout(layout);
    plot.show();
}

fn example_error_bar_plots() {
    basic_symmetric_error_bars();
    bar_chart_with_error_bars();
}

fn simple_candlestick_chart() {
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849998, 115.919998, 116.779999, 117.949997, 118.769997, 118.739998, 118.900002,
        119.110001, 118.339996, 120.0, 119.400002, 120.449997, 120.0, 119.550003, 120.419998,
        121.669998, 122.139999, 120.93, 121.150002, 127.029999, 127.980003, 128.309998, 129.130005,
        130.539993, 131.350006, 131.649994, 132.460007, 133.080002, 133.470001, 135.520004,
    ];
    let high = vec![
        116.510002, 116.860001, 118.160004, 119.43, 119.379997, 119.93, 119.300003, 119.620003,
        120.239998, 120.5, 120.089996, 120.449997, 120.809998, 120.099998, 122.099998, 122.440002,
        122.349998, 121.629997, 121.389999, 130.490005, 129.389999, 129.190002, 130.5, 132.089996,
        132.220001, 132.449997, 132.940002, 133.820007, 135.089996, 136.270004,
    ];
    let low = vec![
        115.75, 115.809998, 116.470001, 117.940002, 118.300003, 118.599998, 118.209999, 118.809998,
        118.220001, 119.709999, 119.370003, 119.730003, 119.769997, 119.5, 120.279999, 121.599998,
        121.599998, 120.660004, 120.620003, 127.010002, 127.779999, 128.160004, 128.899994,
        130.449997, 131.220001, 131.119995, 132.050003, 132.75, 133.25, 134.619995,
    ];
    let close = vec![
        116.019997, 116.610001, 117.910004, 118.989998, 119.110001, 119.75, 119.25, 119.040001,
        120.0, 119.989998, 119.779999, 120.0, 120.080002, 119.970001, 121.879997, 121.940002,
        121.949997, 121.629997, 121.349998, 128.75, 128.529999, 129.080002, 130.289993, 131.529999,
        132.039993, 132.419998, 132.119995, 133.289993, 135.020004, 135.509995,
    ];

    let trace1 = Candlestick::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.show();
}

fn gbm_simple_candlestick_chart() {
    let n = 3_000;
    let x = (0..n).collect();
    let mid = geometric_brownian_motion(100.0, 1.0 / 365.0, n, 0.15, 0.5);
    let mut open = Vec::<f64>::with_capacity(n);
    let mut high = Vec::<f64>::with_capacity(n);
    let mut low = Vec::<f64>::with_capacity(n);
    let mut close = Vec::<f64>::with_capacity(n);
    for mid_point in mid.iter() {
        let up: bool = rand::random();
        if up {
            open.push(0.98 * *mid_point);
            close.push(1.02 * *mid_point);
        } else {
            open.push(1.02 * *mid_point);
            close.push(0.98 * *mid_point);
        }

        low.push(0.92 * *mid_point);
        high.push(1.09 * *mid_point);
    }

    let plot = Plot::candlestick(x, open, high, low, close);
    plot.show();
}

fn example_candlestick_plots() {
    simple_candlestick_chart();
    gbm_simple_candlestick_chart();
}

fn simple_ohlc_chart() {
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849998, 115.919998, 116.779999, 117.949997, 118.769997, 118.739998, 118.900002,
        119.110001, 118.339996, 120.0, 119.400002, 120.449997, 120.0, 119.550003, 120.419998,
        121.669998, 122.139999, 120.93, 121.150002, 127.029999, 127.980003, 128.309998, 129.130005,
        130.539993, 131.350006, 131.649994, 132.460007, 133.080002, 133.470001, 135.520004,
    ];
    let high = vec![
        116.510002, 116.860001, 118.160004, 119.43, 119.379997, 119.93, 119.300003, 119.620003,
        120.239998, 120.5, 120.089996, 120.449997, 120.809998, 120.099998, 122.099998, 122.440002,
        122.349998, 121.629997, 121.389999, 130.490005, 129.389999, 129.190002, 130.5, 132.089996,
        132.220001, 132.449997, 132.940002, 133.820007, 135.089996, 136.270004,
    ];
    let low = vec![
        115.75, 115.809998, 116.470001, 117.940002, 118.300003, 118.599998, 118.209999, 118.809998,
        118.220001, 119.709999, 119.370003, 119.730003, 119.769997, 119.5, 120.279999, 121.599998,
        121.599998, 120.660004, 120.620003, 127.010002, 127.779999, 128.160004, 128.899994,
        130.449997, 131.220001, 131.119995, 132.050003, 132.75, 133.25, 134.619995,
    ];
    let close = vec![
        116.019997, 116.610001, 117.910004, 118.989998, 119.110001, 119.75, 119.25, 119.040001,
        120.0, 119.989998, 119.779999, 120.0, 120.080002, 119.970001, 121.879997, 121.940002,
        121.949997, 121.629997, 121.349998, 128.75, 128.529999, 129.080002, 130.289993, 131.529999,
        132.039993, 132.419998, 132.119995, 133.289993, 135.020004, 135.509995,
    ];

    let trace1 = Ohlc::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.show();
}

fn gbm_simple_ohlc_chart() {
    let n = 3_000;
    let x = (0..n).collect();
    let mid = geometric_brownian_motion(100.0, 1.0 / 365.0, n, 0.15, 0.5);
    let mut open = Vec::<f64>::with_capacity(n);
    let mut high = Vec::<f64>::with_capacity(n);
    let mut low = Vec::<f64>::with_capacity(n);
    let mut close = Vec::<f64>::with_capacity(n);
    for mid_point in mid.iter() {
        let up: bool = rand::random();
        if up {
            open.push(0.98 * *mid_point);
            close.push(1.02 * *mid_point);
        } else {
            open.push(1.02 * *mid_point);
            close.push(0.98 * *mid_point);
        }

        low.push(0.92 * *mid_point);
        high.push(1.09 * *mid_point);
    }

    let plot = Plot::ohlc(x, open, high, low, close);
    plot.show();
    plot.show_png(1024, 1024);
    plot.show_jpg(1024, 512);
}

fn example_ohlc_plots() {
//    simple_ohlc_chart();
    gbm_simple_ohlc_chart();
}

fn main() -> std::io::Result<()> {
//    example_scatter_plots();
    example_bar_charts();
//    example_error_bar_plots();
//    example_candlestick_plots();
    example_ohlc_plots();

    Ok(())
}
