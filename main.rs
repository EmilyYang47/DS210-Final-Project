// Resources: https://users.rust-lang.org/t/how-to-concatenate-two-hashmaps/28970, https://levelup.gitconnected.com/rust-string-concatenation-of-string-and-str-types-59720c2f7182, https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector, https://stackoverflow.com/questions/28280035/accessing-the-last-element-of-a-vec-or-a-slice, https://users.rust-lang.org/t/how-to-concatenate-two-hashmaps/28970, https://www.geeksforgeeks.org/hashmap-containskey-method-in-java/
// Lecture 24, Lecture 15 
// Collaborator: None. 

mod read;
mod graph; 
use crate::graph::Graph;
mod compare; 
mod histogram; 
use crate::histogram::Histogram; 

fn main() {

    let filenames:Vec<(&str,&str, &str)> = vec![("0.feat", "0.featnames", "0"), ("107.feat", "107.featnames", "107"), ("348.feat", "348.featnames", "348"), ("414.feat", "414.featnames", "414"), ("686.feat", "686.featnames", "686"), ("698.feat", "698.featnames", "698"), ("1684.feat", "1684.featnames", "1684"), ("1912.feat", "1912.featnames", "1912"), ("3437.feat", "3437.featnames", "3437"), ("3980.feat", "3980.featnames", "3980")]; 
    let graph = Graph::create_graph("facebook_combined.txt", &filenames); 

    let similarities = compare::all_similarity(&graph);  

    let histogram = Histogram::create_histo(similarities); 
    println!("{:?}", histogram.range);
    println!("{:?}", histogram.frequency); 

    for i in 0..10 {
        println!("{}: {}", histogram.range[i], histogram.frequency.get(&(i as u8)).unwrap()); 
    }

}
