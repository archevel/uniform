use std::collections::HashMap;
use uniform_model::PropType;
use uniform_model::{Entity, Prop};

fn template_entity_title(name: &str, localizations: &HashMap<&str, &str>) -> String {
    let entity_title_key = name.to_string() + "_title";

    let entity_title = localizations
        .get(entity_title_key.as_str())
        .expect("Missing entity title localization key");
    String::from("<h2>") + entity_title + "</h2>"
}

fn template_entity_form(name: &str, lang: &str) -> String {
    String::from("<form id=\"")
        + name
        + "\" name=\""
        + name
        + "\" action=\"/"
        + lang
        + "/"
        + name
        + "/{{entity_id}}\" method=\"post\" lang=\""
        + lang
        + "\"></form>"
}

fn template_entity_form_field_for_prop_label(
    prop_identifier: &str,
    localizations: &HashMap<&str, &str>,
) -> String {
    let prop_label_key = prop_identifier.to_string() + "_label";

    let prop_label = localizations
        .get(prop_label_key.as_str())
        .expect("Missing prop label localization key");
    "<label id=\"".to_string()
        + prop_identifier
        + "_label\" for=\""
        + prop_identifier
        + "\">"
        + prop_label
        + "<a href=\"#"
        + prop_identifier
        + "_help\" ></a></label>"
}

fn prop_type_to_input_type(prop_type: &PropType) -> String {
    match prop_type {
        PropType::String => "text".to_string(),
        _ => panic!("Input type not implemented for prop type"),
    }
}

fn template_entity_form_field_for_prop_input(
    name: &str,
    prop_identifier: &str,
    prop_type: &PropType,
) -> String {
    let input_type = prop_type_to_input_type(prop_type);
    "<input id=\"".to_string()
        + prop_identifier
        + "\" name=\""
        + prop_identifier
        + "\" type=\""
        + input_type.as_str()
        + "\" form=\""
        + name
        + "\" value=\"{{"
        + prop_identifier
        + "_value}}\"/><span></span>"
}

fn template_entity_form_field_for_prop(
    name: &str,
    prop_name: &str,
    prop_type: &PropType,
    localizations: &HashMap<&str, &str>,
) -> String {
    let prop_identifier = name.to_string() + "." + prop_name;
    let label = template_entity_form_field_for_prop_label(&prop_identifier, localizations);
    let input = template_entity_form_field_for_prop_input(name, &prop_identifier, prop_type);

    label + input.as_str()
}

fn template_entity_form_field(
    name: &str,
    prop: &Prop,
    localizations: &HashMap<&str, &str>,
) -> String {
    match prop {
        Prop::Prop(prop_name, prop_type) => {
            template_entity_form_field_for_prop(name, prop_name, prop_type, localizations)
        }
    }
}

fn template_entity_form_fields(
    name: &str,
    props: &Vec<Prop>,
    localizations: &HashMap<&str, &str>,
) -> String {
    props
        .into_iter()
        .map(|p| template_entity_form_field(name, p, localizations))
        .collect::<Vec<String>>()
        .join("")
}

fn template_entity(
    name: &str,
    props: &Vec<Prop>,
    lang: &str,
    localizations: &HashMap<&str, &str>,
) -> String {
    let title = template_entity_title(name, localizations);
    let form = template_entity_form(name, lang);
    let form_fields = template_entity_form_fields(name, props, localizations);
    title + form.as_str() + form_fields.as_str()
}

pub fn template_for_edit(lang: &str, spec: &Entity, localizations: &HashMap<&str, &str>) -> String {
    match spec {
        Entity::Entity(name, _, props) => template_entity(name, props, lang, localizations),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templator_can_create_template_for_edit_with_labeled_input() {
        let (_, entities) = uniform_parser::entities(
            "Foo bar\n\
               bar string\n",
        )
        .unwrap();
        let entity = &entities[0];
        let localization = crate::localizor::localize(&entities);
        let localization_dict = crate::localizor::to_localization_dictionary(&localization);
        let result = template_for_edit("dbg-Dbg", entity, &localization_dict);
        assert_ne!(result, "");
        let has_input_for_bar = result.contains("<input id=\"Foo.bar\" name=\"Foo.bar\" type=\"text\" form=\"Foo\" value=\"{{Foo.bar_value}}\"/><span></span>");
        assert_eq!(has_input_for_bar, true);
        let has_label_for_bar = result.contains("<label id=\"Foo.bar_label\" for=\"Foo.bar\">Foo.bar_label<a href=\"#Foo.bar_help\" ></a></label>");
        assert_eq!(has_label_for_bar, true);
    }

    #[test]
    fn templator_for_edit_template_is_not_hardcoded() {
        let (_, entities) = uniform_parser::entities(
            "Blarg score\n\
               score string\n",
        )
        .unwrap();
        let entity = &entities[0];
        let localization = crate::localizor::localize(&entities);
        let localization_dict = crate::localizor::to_localization_dictionary(&localization);
        let result = template_for_edit("dbg-Dbg", entity, &localization_dict);
        assert_ne!(result, "");
        let has_input_for_bar = result.contains("<input id=\"Blarg.score\" name=\"Blarg.score\" type=\"text\" form=\"Blarg\" value=\"{{Blarg.score_value}}\"/><span></span>");
        assert_eq!(has_input_for_bar, true);
        let has_label_for_bar = result.contains("<label id=\"Blarg.score_label\" for=\"Blarg.score\">Blarg.score_label<a href=\"#Blarg.score_help\" ></a></label>");
        assert_eq!(has_label_for_bar, true);
    }

    #[test]
    fn templator_can_create_template_for_edit_with_titled_form() {
        let (_, entities) = uniform_parser::entities(
            "Foo bar\n\
               bar string\n",
        )
        .unwrap();
        let entity = &entities[0];
        let localization = crate::localizor::localize(&entities);
        let localization_dict = crate::localizor::to_localization_dictionary(&localization);
        let result = template_for_edit("dbg-Dbg", entity, &localization_dict);
        assert_ne!(result, "");
        let has_form_for_foo = result.contains("<form id=\"Foo\" name=\"Foo\" action=\"/dbg-Dbg/Foo/{{entity_id}}\" method=\"post\" lang=\"dbg-Dbg\"></form>");
        assert_eq!(has_form_for_foo, true);
        let has_title_for_foo = result.contains("<h2>Foo_title</h2>");
        assert_eq!(has_title_for_foo, true);
    }
}
