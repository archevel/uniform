
use uniform_model::Entity;

pub fn template(_spec: Vec<Entity>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_should_return_template_string() {
        let input = vec![];
        let result = template(input);
        assert_eq!(result, "<h1>hello</h1>");
    }
}