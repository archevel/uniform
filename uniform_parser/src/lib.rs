use nom;
use nom::character::complete::{
    space1,
    alphanumeric1,
};

use uniform_model::{ Entity, OverviewProp, Prop, PropType };

fn space0_eol(inp: &str) -> nom::IResult<&str, &str>{
    nom::combinator::recognize(
        nom::sequence::pair(
            nom::character::complete::space0,
            nom::character::complete::line_ending)
        )(inp)
}


fn overview_prop(inp: &str) -> nom::IResult<&str, OverviewProp> {
    nom::combinator::map(prop_name, |name| OverviewProp::Prop(String::from(name)))(inp)
}

fn overview_sub_prop(inp: &str) -> nom::IResult<&str, OverviewProp> {
    nom::combinator::map(
        nom::sequence::separated_pair(prop_name, nom::bytes::complete::tag("."), prop_name), 
        |(name,subprop)| OverviewProp::SubProp(String::from(name), String::from(subprop)))(inp)
}

fn overview(inp: &str) -> nom::IResult<&str, Vec<OverviewProp>> { 
    nom::multi::separated_list0(space1, nom::branch::alt((overview_sub_prop, overview_prop)))(inp)
}

fn entity_name(inp: &str) -> nom::IResult<&str, &str> {
   nom::combinator::verify(alphanumeric1, |s: &str| match s.chars().nth(0) { Some(c) => 'A' <= c && c <= 'Z', None => false } )(inp)
}

fn entity_head(inp: &str) -> nom::IResult<&str, (&str, Vec<OverviewProp>)> { 
    let (inp, _) = nom::character::complete::multispace0(inp)?;
    nom::sequence::separated_pair(entity_name, space1, overview)(inp)
}

fn prop_name(inp: &str) -> nom::IResult<&str, &str> {
    nom::combinator::verify(alphanumeric1, |s: &str| match s.chars().nth(0) { Some(c) => 'a' <= c && c <= 'z', None => false } )(inp)
}

fn prop_type(inp: &str) -> nom::IResult<&str, PropType> {
    nom::branch::alt((
        nom::combinator::map(entity_name, |name| PropType::Entity(String::from(name))),
        nom::combinator::map(nom::bytes::complete::tag("int"), |_| PropType::Int),
        nom::combinator::map(nom::bytes::complete::tag("number"), |_| PropType::Number),
        nom::combinator::map(nom::bytes::complete::tag("string"), |_| PropType::String),
    ))(inp)
}

fn prop(inp: &str) -> nom::IResult<&str,Prop> {
    nom::combinator::map(nom::sequence::separated_pair(prop_name, space1, prop_type),
        |(name, pt)| Prop::Prop(String::from(name), pt))
    (inp)
}

fn entity_prop(inp: &str) -> nom::IResult<&str,Prop> {
    let (inp, _) = nom::character::complete::multispace0(inp)?;
    prop(inp)
}

fn entity_props(inp: &str) -> nom::IResult<&str,Vec<Prop>> {
    nom::multi::separated_list1(space0_eol, entity_prop)(inp)
}

fn regular_entity(inp: &str) -> nom::IResult<&str, Entity> { 
    nom::combinator::map(nom::sequence::pair(entity_head, entity_props),
        |((name, entity_overview), properites)| Entity::Entity(String::from(name), entity_overview, properites))(inp)
}

fn enum_entity(inp: &str) -> nom::IResult<&str, Entity> {
    let (inp, _) = nom::character::complete::multispace0(inp)?;
    nom::combinator::map(nom::sequence::separated_pair(entity_name, space1, nom::multi::separated_list1(space1, nom::combinator::map(entity_name, String::from))), 
    |(name, values)| Entity::EnumEntity(String::from(name), values))(inp)
}

fn unverified_entities(inp: &str) -> nom::IResult<&str, Vec<Entity>> { 
    nom::combinator::map(nom::sequence::pair(
        nom::multi::many1(nom::branch::alt((regular_entity, enum_entity))),
        space0_eol),
        |(e, _)| e)(inp)
}

fn find_prop_with_name<'a>(name: &str, props: &'a[Prop]) -> Option<&'a Prop> {
    props.into_iter().find(|p| match p {
        Prop::Prop(n, _) => n == name,
    })
}

fn find_entity_with_name<'a>(name: &str, entities: &'a[Entity]) -> Option<&'a Entity> {
    entities.iter().find(|e| match e {
        Entity::Entity(n, _, _) => n == name,
        Entity::EnumEntity(n, __) => n == name
    })
}

fn verify_entity(entity: &Entity, entities: &[Entity]) -> bool {
    match entity {
        Entity::Entity(_, ops, props) => {
            for op in ops {
                match op {
                    OverviewProp::Prop(name) => {
                        if find_prop_with_name(&name, &props).is_none() {
                            return false
                        }
                    },
                    OverviewProp::SubProp(name, sub_prop) => {
                        match find_prop_with_name(&name, &props) {
                            None => return false,
                            Some(Prop::Prop(_, PropType::Entity(prop_entity))) => {
                                match find_entity_with_name(&prop_entity, entities) {
                                    Some(Entity::Entity(_,_,sub_entity_props)) => {
                                        if find_prop_with_name(&sub_prop, &sub_entity_props).is_none() {
                                            return false
                                        }
                                    },
                                    _ => return false
                                }
                            }
                            Some(_) => return false,
                        }
                    }
                }
            }
            return true
        },
        Entity::EnumEntity(_, _) => return true
    }
}

fn verify_entities(entities: &[Entity]) -> bool {
    for entity in entities.iter() {
        if !verify_entity(entity, entities) {
            return false;
        }
    }
    return true;
}

pub fn entities(inp: &str) -> nom::IResult<&str, Vec<Entity>> { 
    nom::combinator::verify(unverified_entities, verify_entities)(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::{ErrorKind, Error};
    use nom::Err;
    fn test_error<P>(s: &str, err_kind: ErrorKind) -> nom::IResult<&str, P> {
        Err(Err::Error(Error::new(s, err_kind)))
    }
    #[test]
    fn parses_valid_property() {
        let types = vec![("int", PropType::Int), ("number", PropType::Number), ("string", PropType::String)];
        for (uniform_type, pt) in types {
            let (_, result) = prop(&(String::from("hello ") + uniform_type)).unwrap();
            let expect = Prop::Prop(String::from("hello"), pt);
            assert_eq!(result, expect);            
        }
    }

    #[test]
    fn fails_invalid_property_with_capitalized_name() {
        let result = prop("A int");
        let expect = test_error("A int", ErrorKind::Verify);
        assert_eq!(result, expect);
    }

    #[test]
    fn fails_invalid_property_with_non_existent_uniform_type() {
        let result = prop("properName floaty");
        let expect = test_error("floaty", ErrorKind::Tag);
        assert_eq!(result, expect);
    }

    #[test]
    fn fails_invalid_entity_head_starting_with_lowercase() {
        let result = entity_head("entityName floaty");
        let expect = test_error("entityName floaty", ErrorKind::Verify);
        assert_eq!(result, expect);
    }

    #[test]
    fn parses_valid_entity_head() {
        let (_, result) = entity_head("Entity head").unwrap();
        let expect = ("Entity", vec![OverviewProp::Prop(String::from("head"))]);
        assert_eq!(result, expect);            
    }

    #[test]
    fn parses_valid_entity() {
        let (_, result) = regular_entity("Entity head\nhead int\n").unwrap();
        let expect = Entity::Entity("Entity".to_string(), vec![OverviewProp::Prop(String::from("head"))], vec![Prop::Prop(String::from("head"), PropType::Int)]);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parses_entity_with_missing_props() {
        let result = entities("Entity prop1 prop2\n");
        let expect = test_error("prop1 prop2\n", ErrorKind::Verify);
        assert_eq!(result, expect);         
    }

    #[test]
    fn fails_to_parses_entity_with_missing_overviewprops() {
        let result = entities("Entity \nprop1 prop2");
        let expect = test_error("\nprop1 prop2", ErrorKind::AlphaNumeric);
        assert_eq!(result, expect);         
    }

    #[test]
    fn parses_valid_enum_entity() {
        let (_, result) = enum_entity("Food Banana Pineapple").unwrap();
        let expect = Entity::EnumEntity("Food".to_string(), vec!["Banana".to_string(), "Pineapple".to_string()]);
        assert_eq!(result, expect);            
    }
    
    #[test]
    fn parses_valid_enum_entity_with_expected_rest() {
        let (rest, _result) = enum_entity("Food Banana Pineapple\nNonFood Razors Glass").unwrap();
        let expect = "\nNonFood Razors Glass";
        assert_eq!(rest, expect);            
    }

    #[test]
    fn fails_to_parse_ambigous_entity() {
        let result = entities("Food Banana prop1");
        let expect = test_error("prop1", ErrorKind::CrLf);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_entity_without_final_endofline() {
        let result = entities("Entity prop1 prop2\nprop1 string\nprop2 int");
        let expect = test_error("", ErrorKind::CrLf);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_entity_with_bad_props_body() {
        let result = entities("Entity prop1 prop2\nprop1 string prop2 int\n");
        let expect = test_error("prop2 int\n", ErrorKind::CrLf);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_ambigous_entity_with_body() {
        let result = entities("Food prop1 Banana \n prop1 number");
        let expect = test_error("prop1 Banana \n prop1 number", ErrorKind::Verify);
        assert_eq!(result, expect);            
    }

    #[test]
    fn parses_valid_mix_of_entities() {
        let (_, result) = entities("Food Banana Pineapple\nEntity prop1 prop2\nprop1 string\nprop2 int\nNonFood Razors Glass\n").unwrap();
        let expect = vec![
            Entity::EnumEntity("Food".to_string(), vec!["Banana".to_string(), "Pineapple".to_string()]),
            Entity::Entity("Entity".to_string(), vec![OverviewProp::Prop("prop1".to_string()), OverviewProp::Prop("prop2".to_string())],
                vec![Prop::Prop("prop1".to_string(), PropType::String), Prop::Prop("prop2".to_string(), PropType::Int)]),
            Entity::EnumEntity("NonFood".to_string(), vec!["Razors".to_string(), "Glass".to_string()]),
        ];
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_invalid_entity_with_overview_prop_missing_from_props_body() {
        let result = entities("Entity prop1 prop2 \n prop1 number\n");
        let expect = test_error("Entity prop1 prop2 \n prop1 number\n", ErrorKind::Verify);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_invalid_entity_with_overview_prop_missing_from_props_body2() {
        let result = entities("Entity prop1 prop2 \n prop1 number\n prop3 string\n");
        let expect = test_error("Entity prop1 prop2 \n prop1 number\n prop3 string\n", ErrorKind::Verify);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_invalid_entity_with_overview_subprop_missing_from_prop() {
        let result = entities("Entity prop1.foo prop2 \n prop1 number\n prop2 string\n");
        let expect = test_error("Entity prop1.foo prop2 \n prop1 number\n prop2 string\n", ErrorKind::Verify);
        assert_eq!(result, expect);            
    }

    #[test]
    fn fails_to_parse_invalid_entity_with_overview_subprop_entity_not_part_of_spec() {
        let result = entities("Entity prop1.foo prop2 \n prop1 Entity2\n prop2 string\n");
        let expect = test_error("Entity prop1.foo prop2 \n prop1 Entity2\n prop2 string\n", ErrorKind::Verify);
        assert_eq!(result, expect);            
    }
}
