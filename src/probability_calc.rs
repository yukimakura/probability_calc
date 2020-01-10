extern crate csv_import;
extern crate gnuplot;

use csv_import::csv_parse;
use std::error::Error;

use gnuplot::{Figure, Color};

#[derive(Debug)]
pub struct PROBABILITY{
    pub element :i32,
    pub probability :f64
}

pub fn calc_probabries_ir_with_time(input_vec : Vec<csv_parse::SCANDATA>) -> Result<Vec<Vec<PROBABILITY>>, Box<dyn Error>>{
    let mut datas :Vec<Vec<csv_parse::SCANDATA>> = Vec::new();
    let mut probs :Vec<Vec<PROBABILITY>> = Vec::new();

    for n in 0..24{
        datas.push(vec![csv_parse::SCANDATA{ date : -255, time : -255,lidar : -255, ir : -255}]);
    }
    
    for item in input_vec {
        let hour = (item.time - (item.time % 10000)) / 10000;
        if(datas[hour as usize][0].ir == -255){
            datas[hour as usize][0] = item;
        }else{
            datas[hour as usize].push(item);
        }
        
    }
    for item in &datas{
        probs.push(calc_probabries_ir(item,item.len()).unwrap());
    }

    Ok(probs)
}

pub fn calc_probabries_ir(input_vec : &Vec<csv_parse::SCANDATA>,max_size : usize) -> Result<Vec<PROBABILITY>, Box<dyn Error>> {
    let mut probs : Vec<PROBABILITY> = Vec::new();
    let mut prob_element_check_flag : bool = false;
    let mut p_buff : PROBABILITY;

    for item in input_vec{
        for mut p_item in &mut probs{
            if p_item.element == item.ir {
                p_item.probability = p_item.probability + 1.0;
                prob_element_check_flag = true;
            }
        }
        if prob_element_check_flag == false {
            let p_buff  = PROBABILITY {element: item.ir , probability: 1.0};
            probs.push(p_buff);
        }
        prob_element_check_flag = false;
    }
    probs.sort_by(|a, b| a.element.cmp(&b.element));
    for item in &mut probs{
        item.probability = item.probability / ( max_size as f64);
    }

    Ok(probs)
}

pub fn calc_probabries_lidar_with_time(input_vec : Vec<csv_parse::SCANDATA>) -> Result<Vec<Vec<PROBABILITY>>, Box<dyn Error>>{
    let mut datas :Vec<Vec<csv_parse::SCANDATA>> = Vec::new();
    let mut probs :Vec<Vec<PROBABILITY>> = Vec::new();

    for n in 0..24{
        datas.push(vec![csv_parse::SCANDATA{ date : -255, time : -255,lidar : -255, ir : -255}]);
    }
    
    for item in input_vec {
        let hour = (item.time - (item.time % 10000)) / 10000;
        if(datas[hour as usize][0].lidar == -255){
            datas[hour as usize][0] = item;
        }else{
            datas[hour as usize].push(item);
        }
        
    }
    for item in &datas{
        probs.push(calc_probabries_lidar(item,item.len()).unwrap());
    }

    Ok(probs)
}

pub fn calc_probabries_lidar(input_vec : &Vec<csv_parse::SCANDATA>,max_size : usize) -> Result<Vec<PROBABILITY>, Box<dyn Error>> {
    let mut probs : Vec<PROBABILITY> = Vec::new();
    let mut prob_element_check_flag : bool = false;
    let mut p_buff : PROBABILITY;

    for item in input_vec{
        for mut p_item in &mut probs{
            if p_item.element == item.lidar {
                p_item.probability = p_item.probability + 1.0;
                prob_element_check_flag = true;
            }
        }
        if prob_element_check_flag == false {
            let p_buff  = PROBABILITY {element: item.lidar , probability: 1.0};
            probs.push(p_buff);
        }
        prob_element_check_flag = false;
    }
    probs.sort_by(|a, b| a.element.cmp(&b.element));
    for item in &mut probs{
        item.probability = item.probability / ( max_size as f64);
    }

    Ok(probs)
}

pub fn plot(datas : &Vec<PROBABILITY>) {
    let mut fg = Figure::new();
    let mut x : Vec<i32> = Vec::new();
    let mut y : Vec<f64> = Vec::new();
    for item in datas {
        x.push(item.element);
        y.push(item.probability);
    }

    fg.axes2d().lines(&x, &y, &[Color("blue")]); //xはポインタで受けてるので&がない.

    // "png" 設定で画像を保存する.
    // fg.set_terminal("png", "NormalDistribution.png");
    fg.show();
}
