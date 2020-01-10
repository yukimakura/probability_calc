extern crate probability_calc;
extern crate csv_import;
use csv_import::csv_parse;

fn main() {
    let mut datas: Vec<csv_parse::SCANDATA> = Vec::new();
    datas = csv_parse::read_sensor_data("sensor_data_600.csv".to_string()).unwrap();

    // let v_ir = &probability_calc::probability_calc::calc_probabries_ir_with_time(datas).unwrap();
    let v_lidar = &probability_calc::probability_calc::calc_probabries_lidar_with_time(datas).unwrap();
    
    println!{"{:?}",v_lidar.len()}
    probability_calc::probability_calc::plot(&v_lidar[14]);
    for (num,item) in v_lidar.iter().enumerate() {
        println!("{:?}",num);
        println!("{:?}",item);
    }

    
}
