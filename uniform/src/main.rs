fn main() {
    let inp = "Invoice number totals.sum\n\
               \tnumber int\n\
               \ttotals InvoiceTotals\n\
               InvoiteTotals sum vat\n\
               \tsum number\n\
               \tvat VAT\n
               \n\
               \n\
               VAT VAT25 VAT12";
    println!("Hello, world!");
    
    match uniform::parser::entities(inp) {
        Ok((rest, name)) => println!("{:?} -- {:?}", name, rest),
        Err(err) => println!("Error: {:?}", err),
    }
}
