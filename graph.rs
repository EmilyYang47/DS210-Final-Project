use std::collections::HashMap; 

use crate::read::*;

pub struct Graph {
    pub edges: Vec<(usize, usize)>, 
    pub node_features: HashMap::<usize, Vec<usize>>,
} 

impl Graph {

    fn find_all_indices(&mut self, feat: Vec<usize>) -> Vec<usize> {
        // find all indices of '1's in the user's feature vector to see what features that user has 
        let mut indices: Vec<usize> = Vec::new();
        let mut i = 0; 
    
        for f in feat.iter() {
            if f == &1 { 
                indices.push(i); 
            } 
            i += 1;    
        }
        return indices 
    }    
    
    fn convert_feat_to_featname(&mut self, feat: Vec<usize>, names: HashMap::<usize, usize>) -> Vec<usize> {
        //convert each feature index to its corredponding feature name  
        let mut featnames: Vec<usize> = Vec::new(); 
        let indices = self.find_all_indices(feat); 
    
        for i in indices.iter() {
            let corresponding_featname = names.get(i).expect("none"); 
            if !(featnames.contains(&corresponding_featname)) {
                featnames.push(*corresponding_featname); 
            } 
        }
        return featnames; 
    }

    fn pair_up(&mut self, featpair: HashMap::<usize, Vec<usize>>, names: HashMap::<usize, usize>) -> HashMap::<usize, Vec<usize>> { 
        // pair up each user node and the feature names it has  
        let mut result = HashMap::<usize, Vec<usize>>::new();  
    
        for f in featpair {
            let converted = self.convert_feat_to_featname(f.1, names.clone()); 
            result.insert(f.0, converted); 
        } 
    
        return result;   
    }

    pub fn create_graph(edge_file: &str, filenames:&Vec<(&str,&str, &str)>) -> Graph {
        //create a new Graph based on the two input parameters 
        let edges = read_edge(edge_file); 
        let mut g = Graph{edges, node_features:HashMap::<usize, Vec<usize>>::new()};
        
        for f in filenames{ 
            let feat = read_feat(f.0); 
            let name = read_featname(f.1);
            let egofeat = read_egofeat(f.2); 

            let paired = g.pair_up(feat, name.clone()); 
            g.node_features.extend(paired);
            let paired_ego = g.pair_up(egofeat, name.clone()); 
            g.node_features.extend(paired_ego);  
        }
        g               
    }
    
} 