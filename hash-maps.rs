use std::collections::HashMap;

fn main(){
    let mut graph: HashMap<String, Vec<&str>> = HashMap::new();
    graph.insert(String::from("start"), vec!["a", "b"]);
    graph.insert(String::from("a"), vec!["fin"]);
    graph.insert(String::from("b"), vec!["a", "fin"]);
    println!("{:?}", graph);
}
