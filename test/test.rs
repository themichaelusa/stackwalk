use std::fs;
use std::fs::File;
use std::io::Write;

use stackwalk::config::Config;
use stackwalk::indexer::index_directory;

fn main() {
    let toml_str = fs::read_to_string("config.toml").expect("Unable to read file");
    let config = Config::from_toml(&toml_str).unwrap();

    let dir_path = "../../test-repos/dj-stripe-master";
    let (blocks, call_stack, call_graph) = index_directory(&config, dir_path);
    let project_name = "djstripe_callgraph";

    // Save the call graph to a Graphviz file
    let graphviz = call_graph.to_graphviz();
    let graphviz_file_name = format!("{}_call_graph.dot", project_name);
    let mut graphviz_file =
        File::create(&graphviz_file_name).expect("Failed to create Graphviz file");
    write!(graphviz_file, "{}", graphviz).expect("Failed to write to Graphviz file");

    // Save the call graph to a Mermaid file
    let mermaid = call_graph.to_mermaid();
    let mermaid_file_name = format!("{}_call_graph.mermaid", project_name);
    let mut mermaid_file = File::create(&mermaid_file_name).expect("Failed to create Mermaid file");
    write!(mermaid_file, "{}", mermaid).expect("Failed to write to Mermaid file");

    // Save the call graph to a JSON flow chart file
    let json_flow_chart = call_graph.to_json_flowchart();
    let flow_chart_file_name = format!("{}_call_graph.json", project_name);
    let mut flow_chart_file =
        File::create(&flow_chart_file_name).expect("Failed to create Flow Chart file");
    write!(flow_chart_file, "{}", json_flow_chart).expect("Failed to write to Flow Chart file");

    // enumerate potential entry points
    let entry_points = call_graph.get_entry_points();
    if !entry_points.is_empty() {
        for entry_point in entry_points {
            println!("Potential Entry Point: {}", entry_point);
        }
    } else {
        println!("No entry points detected.");
    }
}