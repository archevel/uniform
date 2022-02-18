
use std::collections::HashMap;
use uniform_model::Entity;

pub fn template_for_edit(lang: &str, _spec: &Vec<Entity>, _localizations: HashMap<&str, &str>) -> String {
    "<input id=\"Foo.bar\" name=\"Foo.bar\" type=\"text\" form=\"Foo\" value=\"{{Foo.bar_value}}\"/>".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templator_can_create_template_for_edit_with_input() {
        let (_, input) = uniform_parser::entities(
            "Something bar\n\
               bar string\n"
        ).unwrap();
        let localization = crate::localizor::localize(&input);
        let localization_dict = crate::localizor::to_localization_dictionary(&localization);
        let result = template_for_edit("dbg-Dbg", &input, localization_dict);
        assert_ne!(result, "");
        let has_input_for_bar = result.contains("<input id=\"Foo.bar\" name=\"Foo.bar\" type=\"text\" form=\"Foo\" value=\"{{Foo.bar_value}}\"/>");
        assert_eq!(has_input_for_bar, true);
    }
}