use regex::Regex;

struct VariableDefinition {
    name: String,
    set_type: String
}

fn get_variable_definitions_within_file(file_path: &str) -> Vec<VariableDefinition> {
    let file_content: String = std::fs::read_to_string(file_path).expect("");
    let mut variable_definitions: Vec<VariableDefinition> = vec![];

    for line in file_content.split("\n") {
        for v in Regex::new(r"([a-zA-Z0-9<>_]*?):( *)([a-zA-Z0-9<>_{}]*)").unwrap().find_iter(line).map(|m| {
            let text_match = m.as_str();

            return VariableDefinition {
                name: String::from(text_match.split(":").next().unwrap_or("")),
                set_type: String::from(text_match.rsplit(":").next().unwrap_or("").replace(" ", ""))
            }
        }) {
            variable_definitions.push(v);
        }
    }

    return variable_definitions;
}

fn main() {
    let variable_definitions: Vec<VariableDefinition> = get_variable_definitions_within_file("test/index.ts");

    for variable_definition in variable_definitions {
        println!("{}: {}", variable_definition.name, variable_definition.set_type);
    }
}