use minijinja::Environment;
use serde_yaml::Value;
use std::fs::File;
use std::io::Read;

fn render_template(path: &str, data: Value) -> Result<String, Box<dyn std::error::Error>> {
    // Load the template file into a string
    let mut template_file = File::open(path)?;
    let mut template_string = String::new();
    template_file.read_to_string(&mut template_string)?;

    // Create a Jinja environment and parse the template string
    let mut env = Environment::new();
    let template_key = "template";
    env.add_template(template_key, &template_string).unwrap();
    let template = env.get_template(template_key)?;
    // Go from yaml_serde::Value to minijinja::value::Value using Serializable trait
    let ctx = minijinja::value::Value::from_serializable(&data);

    // Render the template with the input data
    let rendered = template.render(&ctx)?;
    // Return the rendered string
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use crate::log_utils;
    use crate::template::render_template;
    use log::{trace, LevelFilter};
    use relative_path::RelativePath;
    use serde_yaml::Value;
    use std::env::current_dir;

    #[test]
    fn test_render_basic_template() -> Result<(), Box<dyn std::error::Error>> {
        log_utils::setup_logging(LevelFilter::Trace, true);
        println!();
        trace!("Running test_render_basic_template.");
        let current_dir = current_dir()?;
        let template = RelativePath::new("resources/test/templates/world.jinja2")
            .to_logical_path(&current_dir);
        let yaml = "
        val: world
        ";
        let value_file: Value = serde_yaml::from_str(yaml)?;
        let output_string = render_template(template.to_str().unwrap(), value_file)?;
        let expected_string = "Hello, world!".to_string();
        assert_eq!(expected_string, output_string);
        Ok(())
    }

    #[test]
    fn test_render_nested_template_with_default() -> Result<(), Box<dyn std::error::Error>> {
        log_utils::setup_logging(LevelFilter::Trace, true);
        println!();
        trace!("Running test_render_nested_template_with_default.");
        let current_dir = current_dir()?;
        let template = RelativePath::new("resources/test/templates/nested-default.jinja2")
            .to_logical_path(&current_dir);
        let yaml = "
        val: world
        nested:
            second_level:
                bool_val: true
        ";
        let value_file: Value = serde_yaml::from_str(yaml)?;
        let output_string = render_template(template.to_str().unwrap(), value_file)?;
        let expected_string = "test default_str true".to_string();
        assert_eq!(expected_string, output_string);
        Ok(())
    }

    #[test]
    fn test_render_invalid_template() -> Result<(), Box<dyn std::error::Error>> {
        log_utils::setup_logging(LevelFilter::Trace, true);
        println!();
        trace!("Running test_render_invalid_template.");
        let current_dir = current_dir()?;
        let template = RelativePath::new("resources/test/templates/nested-default.jinja2")
            .to_logical_path(&current_dir);
        let yaml = "
        val: world
        nested:
            some: 'other_value'
        ";
        let value_file: Value = serde_yaml::from_str(yaml)?;
        assert_matches!(
            render_template(template.to_str().unwrap(), value_file),
            Err(_)
        );
        Ok(())
    }
}
