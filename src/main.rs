extern crate probability_calc;
extern crate csv_import;
extern crate gnuplot;

use csv_import::csv_parse;
use gnuplot::*;
fn main() {
    let mut datas: Vec<csv_parse::SCANDATA> = Vec::new();
    datas = csv_parse::read_sensor_data("sensor_data_600.csv".to_string()).unwrap();

    // let v_ir = &probability_calc::probability_calc::calc_probabries_ir_with_time(datas).unwrap();
    let v_lidar = &probability_calc::probability_calc::calc_probabries_lidar_with_time(datas).unwrap();
    
    println!{"{:?}",v_lidar.len()}
    let mut plot_data = Figure::new();
    let range = &probability_calc::probability_calc::AXIS_RANGE { x_min : 605.0 ,x_max : 645.0 ,y_min : 0.0 ,y_max : 0.15};
    probability_calc::probability_calc::plot(&v_lidar[14],&mut plot_data,"blue".to_string(),"t=14".to_string(),range);
    probability_calc::probability_calc::plot(&v_lidar[6],&mut plot_data,"red".to_string(),"t=6".to_string(),range);
    for (num,item) in v_lidar.iter().enumerate() {
        println!("{:?}",num);
        println!("{:?}",item);
    }
    plot_data.set_title("t=14 and t=6 line graph");
    plot_data.show();
    
    
}
