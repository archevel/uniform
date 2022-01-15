#[derive(Debug, PartialEq)]
pub enum Entity {
    Entity(String, Vec<OverviewProp>, Vec<Prop>),
    EnumEntity(String, Vec<String>)
}

#[derive(Debug, PartialEq)]
pub enum OverviewProp {
    Prop(String),
    SubProp(String, String)
}

#[derive(Debug, PartialEq)]    
pub enum PropType {
    Int, Number, String, Entity(String)
}

#[derive(Debug, PartialEq)]
pub enum Prop {
    Prop(String, PropType)
}