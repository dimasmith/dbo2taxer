use std::io::{stdin, stdout};

use dbo_csv::{DboRecord, DboStatement, deserialize_statement};
use taxer_csv::{TaxerRecord, serialize_taxer};

fn main() {
    let statement_input = stdin();
    let statement = deserialize_statement(statement_input).unwrap();
    let taxer_out = stdout();
    let records = convert_records(statement);
    serialize_taxer(taxer_out, &records).unwrap();
}

fn convert_records(statement: DboStatement) -> Vec<TaxerRecord> {
    statement.iter().map(convert_record).collect()
}

fn convert_record(record: &DboRecord) -> TaxerRecord {
    TaxerRecord::builder()
        .tax_code_raw(record.party_tax_id.clone())
        .unwrap()
        .amount_raw(record.coverage)
        .unwrap()
        .date(record.operation_date)
        .comment(record.payment_purpose.clone())
        .build()
        .unwrap()
}
