fn print_localization_template(entities: Vec<uniform::model::Entity>) {
    let localizations = uniform::patterner::localize(&entities);
    println!("{:?}", localizations)
}

fn main() {
    let inp = "Invoice number totals.sum\n\
               \tnumber int\n\
               \ttotals InvoiceTotals\n\
               InvoiceTotals sum vat\n\
               \tsum number\n\
               \tvat VAT\n
               \n\
               \n\
               VAT VAT25 VAT12\n";
    println!("Hello, world!");
    match uniform::parser::entities(inp) {
        Ok((_rest, entities)) => print_localization_template(entities), //--println!("{:?} -- {:?}", entities, rest),
        Err(err) => println!("Error: {:?}", err),
    }
}
