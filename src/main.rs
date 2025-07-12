use std::io::{stdin, stdout};

use dbo_csv::{DboRecord, DboStatement, deserialize_statement};
use taxer_csv::{TaxerRecord, serialize_taxer};

fn main() -> anyhow::Result<()> {
    let statement_input = stdin();
    let statement = deserialize_statement(statement_input)?;
    let taxer_out = stdout();
    let records = convert_records(statement)?;
    serialize_taxer(taxer_out, &records)?;
    Ok(())
}

fn convert_records(statement: DboStatement) -> anyhow::Result<Vec<TaxerRecord>> {
    let mut taxer_operations = vec![];
    for dbo in statement.into_inner() {
        taxer_operations.push(convert_record(dbo)?);
    }
    Ok(taxer_operations)
}

fn convert_record(record: DboRecord) -> anyhow::Result<TaxerRecord> {
    TaxerRecord::builder()
        .tax_code_raw(record.party_tax_id)?
        .amount_raw(record.coverage)?
        .date(record.operation_date)
        .comment(record.payment_purpose)
        .build()
        .map_err(|err| anyhow::anyhow!(err))
}
