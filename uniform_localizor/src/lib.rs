use uniform_model::{ Entity, OverviewProp, Prop, PropType };

fn localize_enum_entity(name: &str, members: &Vec<String>) -> String {
    let enum_entity_localisations: Vec<String> = members.iter().map(|m| name.to_string() + "_" + m + "=" + name + "_" + m).collect();
    enum_entity_localisations.join("\n")
}

fn localize_entity(e: &Entity) -> String {
    match e {
        Entity::EnumEntity(name, members) => localize_enum_entity(name, members),
        _ => "".to_string()
    }
}

pub fn localize(spec: Vec<Entity>) -> String {
    spec.iter().map(localize_entity).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_localize_simple_enum_entity() {
        let input:Vec<Entity> = vec![Entity::EnumEntity("Entity".to_string(), vec!["Helo".to_string(), "Bye".to_string()])];

        let result = localize(input);

        assert_eq!(result, 
            "Entity_Helo=Entity_Helo\n\
            Entity_Bye=Entity_Bye");
    }

    #[test]
    fn should_localize_simple_enum_entity2() {
        let input:Vec<Entity> = vec![Entity::EnumEntity("Entity2".to_string(), vec!["Bag".to_string(), "Rod".to_string()])];

        let result = localize(input);

        assert_eq!(result, 
            "Entity2_Bag=Entity2_Bag\n\
             Entity2_Rod=Entity2_Rod");
    }

    #[test]
    fn localization_key_order_is_dependant_on_order_in_vectors() {
        let input:Vec<Entity> = vec![Entity::EnumEntity("Entity".to_string(), vec!["Rod".to_string(), "Bag".to_string()])];

        let result = localize(input);

        assert_eq!(result, 
            "Entity_Rod=Entity_Rod\n\
            Entity_Bag=Entity_Bag");
    }

    #[test]
    fn should_localize_simple_regular_entity_with_one_prop() {
        let input:Vec<Entity> = vec![Entity::Entity("Abro".to_string(), vec![OverviewProp::Prop("rod".to_string())], vec![Prop::Prop("rod".to_string(), PropType::Int)])];
        
        let result = localize(input);

        assert_eq!(result, 
            "Abro_title=Abro_title\n\
            Abro_menu=Abro_menu\n\
            Abro_help?=Abro_help
            Abro_overview.rod=Abro_overview.rod\n\
            Abro.rod_label=Abro.rod_label\n\
            Abro.rod_shorthelp=Abro.rod_shorthelp\n\
            Abro.rod_help?=Abro.rod_help\n\
            Abro.rod_help?=Abro.rod_help_title");
    }
}
