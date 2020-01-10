extern crate probability_calc;
extern crate csv_import;
use csv_import::csv_parse;

fn main() {
    let mut datas: Vec<csv_parse::SCANDATA> = Vec::new();
    datas = csv_parse::read_sensor_data("sensor_data_600.csv".to_string()).unwrap();
    
    // for item in calc_probabries(datas).unwrap(){
    //     println!("{:?},{:?}",item.element,item.probability);
    // }
    // probability_calc::probability_calc::plot(probability_calc::probability_calc::calc_probabries_lidar(datas,datas.len()).unwrap());
    println!("{:?}",probability_calc::probability_calc::calc_probabries_lidar_with_time(datas).unwrap()[0]);
}
