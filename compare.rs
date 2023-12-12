use std::collections::HashMap; 
use crate::graph::Graph; 

fn greater_featuresize(f1: &Vec<usize>, f2: &Vec<usize>) -> f64 { 
    // determine the larger size of the two feature name vectors  
    if f1.len() > f2.len() {
        return f1.len() as f64; 
    }
    else {
        return f2.len() as f64; 
    } 
}

fn similarity(pair: &(usize, usize), graph: &Graph) -> f64 { 
    // Calculate the similairty between one sigle pair of friend users 
    let user1 = &pair.0;
    let user2 = &pair.1; 
    let user1_features = graph.node_features.get(user1).expect("none"); 
    let user2_features = graph.node_features.get(user2).expect("none"); 
    
    let mut count = 0.0; 
    for f1 in user1_features.iter(){ 
        for f2 in user2_features.iter() {
            if f2 == f1 {count += 1.0};
        }
    }

    let n = greater_featuresize(user1_features, user2_features);    
    let similarity =  count / n; 
    
    if similarity > 1.0 { panic!("error: similarity larger than one "); } 
    if similarity < 0.0 { panic!("error: similarity smaller than zero "); }

    return similarity; 

}    

pub fn all_similarity(graph: &Graph) -> Vec<f64> { 
    // calculate the similarity for all pairs of friend users 
    let mut similarities: Vec<f64> = Vec::new();

    for pair in &graph.edges { 
        let s = similarity(pair, graph); 
        similarities.push(s); 
    }

    return similarities; 
} 


#[test]
fn test_greater_featuresize() {
    let f1 = vec![1,2,3,4,5]; 
    let f2 = vec![1,2,3,4,5,6]; 
    let f3 = vec![1,2,3,4,5]; 
    
    let greater_size1 = greater_featuresize (&f1, &f2);  
    let expect1 = 6.0; 
    let greater_size2 = greater_featuresize (&f1, &f3);  
    let expect2 = 5.0;

    assert_eq!(greater_size1, expect1, "Wrong greater size");
    assert_eq!(greater_size2, expect2, "Wrong greater size");

}

#[test]
fn test_similarity() { 
    let edges = vec![(0,1)]; 
    let mut node_features = HashMap::<usize, Vec<usize>>::new(); 
    node_features.insert(0, vec![2, 3, 1]); 
    node_features.insert(1, vec![3]); 
    let graph = Graph{edges, node_features}; 
    let pair = (0,1); 

    let s = similarity(&pair, &graph); 
    let expect = 1.0/3.0; 

    assert_eq!(s, expect, "Similarity calculated wrong ");
    assert_eq!(s>=0.0, true, "Similarity smaller than zero ");
    assert_eq!(s<=1.0, true, "Similarity larger than one ");

}

