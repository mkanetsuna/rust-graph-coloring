use std::collections::HashMap;

fn is_safe(node: &str, color: &str, graph: &HashMap<&str, Vec<&str>>, colors: &HashMap<&str, &str>) -> bool {
    if let Some(neighbors) = graph.get(node) {
        for &neighbor in neighbors {
            if let Some(&neighbor_color) = colors.get(neighbor) {
                if neighbor_color == color {
                    return false;
                }
            }
        }
    }
    true
}

fn graph_coloring<'a>(
    nodes: &[&'a str], 
    graph: &HashMap<&'a str, Vec<&'a str>>, 
    colors: &mut HashMap<&'a str, &'a str>, 
    available_colors: &[&'a str], 
    node_index: usize
) -> bool {
    if node_index == nodes.len() {
        return true;
    }

    let node = nodes[node_index];

    for &color in available_colors {
        if is_safe(node, color, graph, colors) {
            colors.insert(node, color);
            if graph_coloring(nodes, graph, colors, available_colors, node_index + 1) {
                return true;
            }
            colors.remove(node);
        }
    }
    false
}

fn main() {
    let nodes = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    let edges = vec![
        ("A", "B"),
        ("A", "C"),
        ("B", "D"),
        ("C", "D"),
        ("C", "I"),
        ("C", "F"),
        ("I", "E"),
        ("I", "F"),
        ("I", "H"),
        ("E", "G"),
        ("G", "H")
    ];

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(u, v) in &edges {
        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u);
    }

    let mut colors: HashMap<&str, &str> = HashMap::new();
    let available_colors = vec!["Red", "Green", "Blue", "Yellow"];

    if graph_coloring(&nodes, &graph, &mut colors, &available_colors, 0) {
        for (node, color) in &colors {
            println!("Node {}: {}", node, color);
        }
    } else {
        println!("No solution found");
    }
}
