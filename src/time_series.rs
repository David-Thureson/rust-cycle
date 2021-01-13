use crate::*;

pub fn main() {
    //bg!(data_source::read_timesheet(Some(5)));
    let series = data_source::read_timesheet(Some(5));
    series.print_indent(0);
}

pub enum TimeCycleSize {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug)]
pub struct TimeBlockSeries {
    pub blocks: Vec<TimeBlock>,
}

#[derive(Debug)]
pub struct TimeBlock {
    pub label: String,
    pub start_time: time::PrimitiveDateTime,
    pub end_time: time::PrimitiveDateTime,
}

impl TimeBlockSeries {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
        }
    }

    pub fn print_indent(&self, depth: usize) {
        let i0 = indent(depth);
        println!("\n{}TimeBlockSeries {{", i0);
        for block in self.blocks.iter() {
            block.print_indent(depth + 1);
        }
        println!("{}}}", i0);
    }
}

impl TimeBlock {

    pub fn new(label: &str, start_time: time::PrimitiveDateTime, end_time: time::PrimitiveDateTime) -> Self {
        Self {
            label: label.to_string(),
            start_time,
            end_time,
        }
    }

    pub fn midpoint_time(&self) -> time::PrimitiveDateTime {
        self.start_time + ((self.end_time - self.start_time) / 2)
    }

    pub fn start_time_cycle_fraction(&self, cycle_size: &TimeCycleSize) -> f64 {
        time_to_cycle_fraction(&self.start_time, cycle_size)
    }

    pub fn end_time_cycle_fraction(&self, cycle_size: &TimeCycleSize) -> f64 {
        time_to_cycle_fraction(&self.end_time, cycle_size)
    }

    pub fn midpoint_time_cycle_fraction(&self, cycle_size: &TimeCycleSize) -> f64 {
        time_to_cycle_fraction(&self.midpoint_time(), cycle_size)
    }

    pub fn duration(&self) -> time::Duration {
        self.end_time - self.start_time
    }

    pub fn duration_cycle_fraction(&self, cycle_size: &TimeCycleSize) -> f64 {
        time_to_cycle_fraction(&self.midpoint_time(), cycle_size)
    }

    pub fn print_indent(&self, depth: usize) {
        let i0 = indent(depth);
        let i1 = indent(depth + 1);
        println!("\n{}TimeBlock {{", i0);
        println!("{}label:            {:?}", i1, self.label);
        println!("{}start_time:       {}", i1, format_time(&self.start_time));
        println!("{}midpoint_time:    {}", i1, format_time(&self.midpoint_time()));
        println!("{}end_time:         {}", i1, format_time(&self.end_time));
        println!("{}minutes:          {}", i1, self.duration().whole_minutes());
        println!("{}start_in_hour:    {:?}", i1, self.start_time_cycle_fraction(&TimeCycleSize::Hour));
        println!("{}midpoint_in_hour: {:?}", i1, self.midpoint_time_cycle_fraction(&TimeCycleSize::Hour));
        println!("{}end_in_hour:      {:?}", i1, self.end_time_cycle_fraction(&TimeCycleSize::Hour));
        println!("{}start_in_day:     {:?}", i1, self.start_time_cycle_fraction(&TimeCycleSize::Day));
        println!("{}midpoint_in_day:  {:?}", i1, self.midpoint_time_cycle_fraction(&TimeCycleSize::Day));
        println!("{}end_in_day:       {:?}", i1, self.end_time_cycle_fraction(&TimeCycleSize::Day));
        println!("{}start_in_week:    {:?}", i1, self.start_time_cycle_fraction(&TimeCycleSize::Week));
        println!("{}midpoint_in_week: {:?}", i1, self.midpoint_time_cycle_fraction(&TimeCycleSize::Week));
        println!("{}end_in_week:      {:?}", i1, self.end_time_cycle_fraction(&TimeCycleSize::Week));
        println!("{}}}", i0);
    }

}

fn format_time(time: &time::PrimitiveDateTime) -> String {
    time.format("%F %r")
}

fn time_to_cycle_fraction(time: &time::PrimitiveDateTime, cycle_size: &TimeCycleSize) -> f64 {
    let duration_from_cycle_start = *time - time_to_start_of_cycle(time, cycle_size);
    let fraction = duration_to_cycle_fraction(&duration_from_cycle_start, cycle_size);
    assert!(fraction.is_finite());
    assert!(fraction >= 0.0);
    assert!(fraction <= 1.0);
    fraction
}

fn duration_to_cycle_fraction(duration: &time::Duration, cycle_size: &TimeCycleSize) -> f64 {
    let msec_in_cycle = match cycle_size {
        TimeCycleSize::Hour => 60 * 60 * 1_000,
        TimeCycleSize::Day => 24 * 60 * 60 * 1_000,
        TimeCycleSize::Week => 7 * 24 * 60 * 60 * 1_000,
        _ => unimplemented!()
    };
    let msec_from_start = duration.whole_milliseconds();
    let fraction = msec_from_start as f64 / msec_in_cycle as f64;
    assert!(fraction.is_finite());
    assert!(fraction >= 0.0);
    assert!(fraction <= 1.0);
    fraction
}

fn time_to_start_of_cycle(time: &time::PrimitiveDateTime, cycle_size: &TimeCycleSize) -> time::PrimitiveDateTime {
    match cycle_size {
        TimeCycleSize::Hour => time::PrimitiveDateTime::new(time.date(), time::Time::try_from_hms(time.hour(), 0, 0).unwrap()),
        TimeCycleSize::Day => time::PrimitiveDateTime::new(time.date(), time::time!(0: 00)),
        TimeCycleSize::Week => {
            let days_from_sunday = time.weekday().number_days_from_sunday();
            let start_of_week_same_time = * time - time::Duration::days(days_from_sunday as i64);
            let start_of_week = time_to_start_of_cycle(&start_of_week_same_time, &TimeCycleSize::Day);
            //rintln! ("time = {}, start_of_week = {}", format_time(time), format_time( & start_of_week));
            start_of_week
        },
        _ => unimplemented!()
    }
}

