use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
struct Node{
    weight: usize,
    children: Vec<String>,
}
fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let node_list = input.trim().to_string();
    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut node_names: HashSet<String> = HashSet::new();
    for entry in node_list.split("\n"){
        let parts: Vec<&str> = entry.split_whitespace().collect();
        let children: Vec<String>;
        if parts.len() > 2 {
            children = parts[3..].iter().map(|x| x.replace(",","")).collect();
            for child in &children{
                node_names.insert(child.to_string());
            }
            let _ = children.iter().map(|x| node_names.insert(x.clone()));
        } else {
            children = vec![];
        }
        let name = parts[0];
        let weight = parts[1].replace("(","").replace(")","");
        nodes.insert(name, Node { weight: weight.parse().unwrap(), children: children.clone() });
    }
    // Which node is at the bottom?
    let mut root = "";
    for key in nodes.keys(){
        if !node_names.contains(&key.to_string()){
            root = key;
        }
    }
    println!("{} is the root node", root);
    // Now find which is unbalanced
    let mut unbalanced_node = get_unbalanced_node(&nodes, root);
    let mut parent = get_nodes_parent(&nodes, unbalanced_node);
    let mut total_weights: Vec<usize> = nodes[parent].children.iter().map(|x| get_total_weight(&nodes, x)).collect();
    total_weights.sort();
    total_weights.dedup();
    let weight_change_required = total_weights[1]-total_weights[0];
    println!("Unbalanced node's weight should be {}", nodes[unbalanced_node].weight - weight_change_required);
}

fn get_unbalanced_node<'a>(nodes: &'a HashMap<&str, Node>, node: &'a str) -> &'a str {
    let mut total_weights: Vec<usize> = nodes[node].children.iter().map(|x| get_total_weight(&nodes, x)).collect();
    let mut mismatch = 0;
    for w in &total_weights {
        if *w != total_weights[0]{
            mismatch = *w;
            break;
        }
    }

    if (mismatch == 0){
        //Then it's none of our children - it's us that's problem!
        return node;
    }

    // But which is which?
    if total_weights.iter().filter(|&x| *x==mismatch).count() != 1 {
        mismatch = total_weights[0];
    }

    // And which node is it?
    let mut pos = total_weights.iter().position(|&x| x == mismatch).unwrap();
    // println!("{:?},{:?},{:?}", total_weights ,mismatch, nodes[node].children[pos] );
    return get_unbalanced_node(&nodes, &nodes[node].children[pos]);
    // return node;
}

fn get_total_weight(nodes: &HashMap<&str, Node>, node: &str) -> usize {
    let mut weight = nodes[node].weight;
    if nodes[node].children.len() > 0 {
        for child in &nodes[node].children {
            weight += get_total_weight(&nodes, &child);
        }
    }
    weight
}

fn get_nodes_parent<'a>(nodes: &'a HashMap<&str, Node>, node: &'a str) -> &'a str {
    let mut parent = "";
    for key in nodes.keys(){
        if (nodes[key].children.iter().position(|x| x == node) != None){
            parent = key;
            break;
        }
    }
    parent
}
