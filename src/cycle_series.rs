use crate::*;

#[derive(Debug)]
pub struct CycleSeries {
    pub entries: Vec<CycleEntry>,
}

#[derive(Debug)]
pub struct CycleEntry {
    pub label: String,
    pub vector: Vector,
}

impl CycleSeries {
    pub fn new() -> Self {
        Self {
            entries: vec![],
        }
    }

    /*
    pub fn vector_sum(&self) -> Vector {
        self.entries.iter().sum()
    }
    */

    pub fn print_indent(&self, depth: usize) {
        let i0 = indent(depth);
        println!("\n{}CycleSeries {{", i0);
        for entry in self.entries.iter() {
            entry.print_indent(depth + 1);
        }
        println!("{}}}", i0);
    }
}

impl CycleEntry {

    pub fn new(label: &str, vector: Vector) -> Self {
        Self {
            label: label.to_string(),
            vector,
        }
    }

    pub fn print_indent(&self, depth: usize) {
        let i0 = indent(depth);
        let i1 = indent(depth + 1);
        println!("\n{}CycleEntry {{", i0);
        println!("{}label: {:?}", i1, self.label);
        self.vector.print_indent(depth + 1);
        println!("{}}}", i0);
    }

}

