
use std::collections::HashMap;
use uniform_model::{Entity, OverviewProp, Prop};

fn localize_property(entity_name: &str, prop: &Prop) -> String {
    match prop {
        Prop::Prop(prop_name, _) => {
            let entity_prop = entity_name.to_string() + "." + &prop_name;
            let prop_label = entity_prop.to_string() + "_label=" + &entity_prop + "_label\n";
            let prop_shorthelp =
                entity_prop.to_string() + "_shorthelp=" + &entity_prop + "_shorthelp\n";
            let prop_placeholder =
                entity_prop.to_string() + "_placeholder?=" + &entity_prop + "_placeholder\n";
            let prop_help = entity_prop.to_string() + "_help?=" + &entity_prop + "_help\n";
            let prop_help_title =
                entity_prop.to_string() + "_help_title?=" + &entity_prop + "_help_title";
            prop_label + &prop_shorthelp + &prop_placeholder + &prop_help + &prop_help_title
        }
    }
}

fn localize_overview(entity_name: &str, overview_prop: &OverviewProp) -> String {
    match overview_prop {
        OverviewProp::Prop(prop_name) => {
            entity_name.to_string()
                + "_overview."
                + &prop_name
                + "="
                + entity_name
                + "_overview."
                + &prop_name
        }
        OverviewProp::SubProp(prop_name, sub_prop_name) => {
            entity_name.to_string()
                + "_overview."
                + &prop_name
                + "."
                + &sub_prop_name
                + "="
                + entity_name
                + "_overview."
                + &prop_name
                + "."
                + &sub_prop_name
        }
    }
}

fn localize_regular_entity(name: &str, overview: &Vec<OverviewProp>, props: &Vec<Prop>) -> String {
    let entity_title = name.to_string() + "_title=" + name + "_title\n";
    let entity_menu_name = name.to_string() + "_menu=" + name + "_menu\n";
    let entity_help_optional = name.to_string() + "_help?=" + name + "_help\n";

    let entity_overview: Vec<String> = overview
        .iter()
        .map(|o| localize_overview(name, o))
        .collect();
    let entity_overview = entity_overview.join("\n");

    let entity_props: Vec<String> = props.iter().map(|p| localize_property(name, p)).collect();
    let entity_props = entity_props.join("\n");

    entity_title
        + &entity_menu_name
        + &entity_help_optional
        + &entity_overview
        + "\n"
        + &entity_props
}

fn localize_enum_entity(name: &str, members: &Vec<String>) -> String {
    let enum_entity_localisations: Vec<String> = members
        .iter()
        .map(|m| name.to_string() + "_" + m + "=" + name + "_" + m)
        .collect();
    enum_entity_localisations.join("\n")
}

fn localize_entity(e: &Entity) -> String {
    match e {
        Entity::EnumEntity(name, members) => localize_enum_entity(name, members),
        Entity::Entity(name, overview, props) => localize_regular_entity(name, overview, props),
    }
}

pub fn localize(spec: &Vec<Entity>) -> String {
    let localizations: Vec<String> = spec.iter().map(localize_entity).collect();
    localizations.join("\n")
}

fn clean_dictionary_key(key: &str) -> &str {
    key.trim()
}

fn clean_dictionary_value(val: &str) -> &str {
    // TODO: Should all values be HTML encoded or should that be handled by the templator? 
    val.trim()
}

pub fn to_localization_dictionary(input: &str) -> HashMap<&str, &str> {
    let mut dict = HashMap::new();
    for line in input.split("\n") {
        let mut splitter = line.splitn(2, "=");
        let key = splitter.next().expect(&("Malformed localization file: ".to_string() + line));
        let value = splitter.next().expect(&("Malformed localization file: ".to_string() + line));
        let clean_key = clean_dictionary_key(key);
        let clean_value = clean_dictionary_value(value);
        dict.insert(clean_key, clean_value);
    }
    
    return dict;
}

#[cfg(test)]
mod tests {
    use super::*;
    use uniform_model::PropType;

    #[test]
    fn localize_template_should_handle_simple_enum_entity() {
        let input: Vec<Entity> = vec![Entity::EnumEntity(
            "Entity".to_string(),
            vec!["Helo".to_string(), "Bye".to_string()],
        )];

        let result = localize(&input);

        assert_eq!(
            result,
            "Entity_Helo=Entity_Helo\n\
            Entity_Bye=Entity_Bye"
        );
    }

    #[test]
    fn localize_template_should_handle_simple_enum_entity2() {
        let input: Vec<Entity> = vec![Entity::EnumEntity(
            "Entity2".to_string(),
            vec!["Bag".to_string(), "Rod".to_string()],
        )];

        let result = localize(&input);

        assert_eq!(
            result,
            "Entity2_Bag=Entity2_Bag\n\
             Entity2_Rod=Entity2_Rod"
        );
    }

    #[test]
    fn localize_template_key_order_is_dependant_on_order_in_vectors() {
        let input: Vec<Entity> = vec![Entity::EnumEntity(
            "Entity".to_string(),
            vec!["Rod".to_string(), "Bag".to_string()],
        )];

        let result = localize(&input);

        assert_eq!(
            result,
            "Entity_Rod=Entity_Rod\n\
            Entity_Bag=Entity_Bag"
        );
    }

    #[test]
    fn localize_template_should_handle_simple_regular_entity_with_one_prop() {
        let input: Vec<Entity> = vec![Entity::Entity(
            "Abro".to_string(),
            vec![OverviewProp::Prop("rod".to_string())],
            vec![Prop::Prop("rod".to_string(), PropType::Int)],
        )];

        let result = localize(&input);

        assert_eq!(
            result,
            "Abro_title=Abro_title\n\
            Abro_menu=Abro_menu\n\
            Abro_help?=Abro_help\n\
            Abro_overview.rod=Abro_overview.rod\n\
            Abro.rod_label=Abro.rod_label\n\
            Abro.rod_shorthelp=Abro.rod_shorthelp\n\
            Abro.rod_placeholder?=Abro.rod_placeholder\n\
            Abro.rod_help?=Abro.rod_help\n\
            Abro.rod_help_title?=Abro.rod_help_title"
        );
    }

    #[test]
    fn localize_template_should_handle_simple_regular_entity_with_sub_prop() {
        let input: Vec<Entity> = vec![Entity::Entity(
            "Emptor".to_string(),
            vec![OverviewProp::SubProp("rod".to_string(), "foo".to_string())],
            vec![Prop::Prop(
                "rod".to_string(),
                PropType::Entity("Fooer".to_string()),
            )],
        )];

        let result = localize(&input);

        assert_eq!(
            result,
            "Emptor_title=Emptor_title\n\
            Emptor_menu=Emptor_menu\n\
            Emptor_help?=Emptor_help\n\
            Emptor_overview.rod.foo=Emptor_overview.rod.foo\n\
            Emptor.rod_label=Emptor.rod_label\n\
            Emptor.rod_shorthelp=Emptor.rod_shorthelp\n\
            Emptor.rod_placeholder?=Emptor.rod_placeholder\n\
            Emptor.rod_help?=Emptor.rod_help\n\
            Emptor.rod_help_title?=Emptor.rod_help_title"
        );
    }

    #[test]
    fn localize_template_should_handle_simple_regular_entity_with_mix_of_props() {
        let input: Vec<Entity> = vec![Entity::Entity(
            "Emptor".to_string(),
            vec![
                OverviewProp::SubProp("rod".to_string(), "foo".to_string()),
                OverviewProp::Prop("boo".to_string()),
            ],
            vec![
                Prop::Prop("boo".to_string(), PropType::Int),
                Prop::Prop("rod".to_string(), PropType::Entity("Fooer".to_string())),
            ],
        )];

        let result = localize(&input);

        assert_eq!(
            result,
            "Emptor_title=Emptor_title\n\
            Emptor_menu=Emptor_menu\n\
            Emptor_help?=Emptor_help\n\
            Emptor_overview.rod.foo=Emptor_overview.rod.foo\n\
            Emptor_overview.boo=Emptor_overview.boo\n\
            Emptor.boo_label=Emptor.boo_label\n\
            Emptor.boo_shorthelp=Emptor.boo_shorthelp\n\
            Emptor.boo_placeholder?=Emptor.boo_placeholder\n\
            Emptor.boo_help?=Emptor.boo_help\n\
            Emptor.boo_help_title?=Emptor.boo_help_title\n\
            Emptor.rod_label=Emptor.rod_label\n\
            Emptor.rod_shorthelp=Emptor.rod_shorthelp\n\
            Emptor.rod_placeholder?=Emptor.rod_placeholder\n\
            Emptor.rod_help?=Emptor.rod_help\n\
            Emptor.rod_help_title?=Emptor.rod_help_title"
        );
    }

    #[test]
    fn localize_template_should_handle_all_entities_provided() {
        let input: Vec<Entity> = vec![
            Entity::EnumEntity(
                "Entity".to_string(),
                vec!["Rod".to_string(), "Bag".to_string()],
            ),
            Entity::Entity(
                "Abro".to_string(),
                vec![OverviewProp::Prop("rod".to_string())],
                vec![Prop::Prop("rod".to_string(), PropType::Int)],
            ),
            Entity::EnumEntity(
                "Caveat".to_string(),
                vec!["Ihildur".to_string(), "Oulipsius".to_string()],
            ),
        ];
        let result = localize(&input);

        assert_eq!(
            result,
            "Entity_Rod=Entity_Rod\n\
            Entity_Bag=Entity_Bag\n\
            Abro_title=Abro_title\n\
            Abro_menu=Abro_menu\n\
            Abro_help?=Abro_help\n\
            Abro_overview.rod=Abro_overview.rod\n\
            Abro.rod_label=Abro.rod_label\n\
            Abro.rod_shorthelp=Abro.rod_shorthelp\n\
            Abro.rod_placeholder?=Abro.rod_placeholder\n\
            Abro.rod_help?=Abro.rod_help\n\
            Abro.rod_help_title?=Abro.rod_help_title\n\
            Caveat_Ihildur=Caveat_Ihildur\n\
            Caveat_Oulipsius=Caveat_Oulipsius"
        );
    }

    #[test]
    fn localization_can_be_converted_to_flat_dictionary() {
        let input = "Entity_Rod=Entity_Rod_value\n\
                     Entity_Bag=Entity_Bag_val";

        let result = to_localization_dictionary(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result["Entity_Rod"], "Entity_Rod_value");
        assert_eq!(result["Entity_Bag"], "Entity_Bag_val");
    }
}
