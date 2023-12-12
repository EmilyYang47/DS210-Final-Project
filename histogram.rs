use std::collections::HashMap;

pub struct Histogram<'a>{ 
    pub range: Vec<&'a str>, 
    pub frequency: HashMap::<u8, usize>,
} 

impl Histogram<'_> { 
    fn count_frequency(&mut self, percentage: Vec<f64>) -> HashMap::<u8, usize> {          
        let mut result = HashMap::<u8, usize>::new();  

        for p in percentage.iter() { 
            if p >= &0.0 && p <&0.1 {
                let count = result.entry(0).or_insert(0);
                *count += 1;
            } else if p >= &0.1 && p < &0.2{
                let count = result.entry(1).or_insert(0);
                *count += 1;
            } else if p >= &0.2 && p < &0.3{
                let count = result.entry(2).or_insert(0);
                *count += 1;
            } else if p >= &0.3 && p < &0.4{
                let count = result.entry(3).or_insert(0);
                *count += 1;
            } else if p >= &0.4 && p < &0.5{
                let count = result.entry(4).or_insert(0);
                *count += 1;
            } else if p >= &0.5 && p < &0.6{
                let count = result.entry(5).or_insert(0);
                *count += 1;
            } else if p >= &0.6 && p < &0.7{
                let count = result.entry(6).or_insert(0);
                *count += 1;
            } else if p >= &0.7 && p < &0.8{
                let count = result.entry(7).or_insert(0);
                *count += 1;
            }
            else if p >= &0.8 && p < &0.9{
                let count = result.entry(8).or_insert(0);
                *count += 1; 
            } else { 
                let count = result.entry(9).or_insert(0);
                *count += 1;
            } 
        } 

        for n in vec![0,1,2,3,4,5,6,7,8,9] {
            result.entry(n).or_insert(0); 
        }; 

        return result; 
    }    
   
    pub fn create_histo(percentage: Vec<f64>) -> Histogram<'static> { 
        let range = vec!["0-10%", "10%-20%", "20-30", "30-40", "40-50", "50-60", "60-70", "70-80", "80-90", "90-100"]; 
        let mut h = Histogram{range, frequency: HashMap::<u8, usize>::new()};
        let frequency = h.count_frequency(percentage); 
        h.frequency = frequency; 
        h
    }
    
} 

#[test]
fn test_frequency() {
    let p : Vec<f64> = vec![0.1, 0.3, 0.7, 0.8, 0.4, 0.2, 0.3, 0.2]; 
    let range = vec!["0-10%", "10%-20%", "20-30", "30-40", "40-50", "50-60", "60-70", "70-80", "80-90", "90-100"]; 
    let mut h = Histogram{range, frequency: HashMap::<u8, usize>::new()};
    let f = h.count_frequency(p.clone());

    let mut expect = HashMap::<u8, usize>::new(); 
    let freq = vec![0, 1, 2, 2, 1, 0, 0, 1, 1, 0]; 
    let mut c = 0; 

    for fr in freq { 
        expect.insert(c, fr); 
        c += 1; 
    }
    
    assert_eq!(f, expect, "Frequencies do not count correctly");
}

#[test]
fn test_frequency_add_up_to_total_amount() {
    let p : Vec<f64> = vec![0.1, 0.3, 0.7, 0.8, 0.4, 0.2, 0.3, 0.2]; 
    let range = vec!["0-10%", "10%-20%", "20-30", "30-40", "40-50", "50-60", "60-70", "70-80", "80-90", "90-100"]; 
    let mut h = Histogram{range, frequency: HashMap::<u8, usize>::new()};
    let f = h.count_frequency(p.clone());

    let mut amount = 0;     
    for n in f.iter() {
        amount += n.1; 
    }
    assert_eq!(amount, p.len(), "Frequencies do not sum up to the total amount");
}

