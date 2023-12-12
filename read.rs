
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


pub fn read_edge(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    return result;
} 

pub fn read_feat(path: &str) -> HashMap::<usize, Vec<usize>> {
    let mut result = HashMap::<usize, Vec<usize>>::new();  

    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = { let mut vec = Vec::new(); 
                for i in 1..v.len(){
                 vec.push(v[i].parse::<usize>().unwrap());}
                 vec
                };
        result.insert(x, y); 
    }
    return result; 
} 

pub fn read_egofeat(node: &str) -> HashMap::<usize, Vec<usize>> {
    let mut result = HashMap::<usize, Vec<usize>>::new();  

    let file = File::open(format!("{}.egofeat", node)).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = node.parse::<usize>().unwrap();
        let y = { let mut vec = Vec::new(); 
                for i in v{
                 vec.push(i.parse::<usize>().unwrap());}
                 vec
                };
        result.insert(x, y); 
    }
    return result; 
} 

pub fn read_featname(path: &str) -> HashMap::<usize, usize> {
    let mut result = HashMap::<usize, usize>::new();  

    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect(); 

        let x = v[0].parse::<usize>().unwrap();
        let y = v.last().unwrap().parse::<usize>().unwrap();
        result.insert(x,y);
    }
    return result; 

}
