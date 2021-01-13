use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::*;

pub fn read_timesheet(limit: Option<usize>) -> TimeBlockSeries {
    let mut series = TimeBlockSeries::new();

    let file_name = "E:\\Projects\\Rust\\console\\cycle\\Time 1.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let line_split = raw_line.split("\t").collect::<Vec<_>>();
        //bg!(&line_split);
        let date_string = line_split[1];
        if date_string.contains("/") && line_split[2].len() > 0 {
            let date_split = date_string.split("/").collect::<Vec<_>>();
            //bg!(&date_split);
            let month = date_split[0].parse::<u8>().unwrap();
            let day = date_split[1].parse::<u8>().unwrap();
            let year = date_split[2].parse::<i32>().unwrap();
            let date = time::Date::try_from_ymd(year, month, day).unwrap();
            let start_hour = line_split[2].parse::<u8>().unwrap();
            let start_minute = line_split[3].parse::<u8>().unwrap();
            let end_hour = line_split[4].parse::<u8>().unwrap();
            let end_minute = line_split[5].parse::<u8>().unwrap();
            let start_time_of_day = time::Time::try_from_hms(start_hour, start_minute, 0).unwrap();
            let end_time_of_day = time::Time::try_from_hms(end_hour, end_minute, 0).unwrap();
            let start_time = time::PrimitiveDateTime::new(date, start_time_of_day);
            let end_time = time::PrimitiveDateTime::new(date, end_time_of_day);
            assert!(start_time < end_time);
            let label = line_split[13];
            let block = time_series::TimeBlock::new(&label, start_time, end_time);
            series.blocks.push(block);
            if let Some(limit) = limit {
                if series.blocks.len() >= limit {
                    break;
                }
            }
        }
    }

    series
}