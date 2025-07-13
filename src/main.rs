use std::io::{stdin, stdout};

use dbo_csv::{DboRecord, DboStatement, deserialize_statement};
use dbo2taxer::TaxerConfig;
use taxer_csv::{TaxerRecord, serialize_taxer};

fn main() -> anyhow::Result<()> {
    let taxer_config = TaxerConfig::from_configuration()?;
    let statement_input = stdin();
    let statement = deserialize_statement(statement_input)?;
    let taxer_out = stdout();
    let records = convert_records(statement, &taxer_config)?;
    serialize_taxer(taxer_out, &records)?;
    Ok(())
}

fn convert_records(
    statement: DboStatement,
    config: &TaxerConfig,
) -> anyhow::Result<Vec<TaxerRecord>> {
    let mut taxer_operations = vec![];
    for dbo in statement.into_inner() {
        taxer_operations.push(convert_record(dbo, config)?);
    }
    Ok(taxer_operations)
}

fn convert_record(record: DboRecord, config: &TaxerConfig) -> anyhow::Result<TaxerRecord> {
    TaxerRecord::builder()
        .tax_code_raw(record.party_tax_id)?
        .amount_raw(record.coverage)?
        .date(record.operation_date)
        .comment(record.payment_purpose)
        .operation(config.operation.clone())
        .income_type(config.income_type.clone())
        .account_name(config.account_name.clone())
        .currency_code(config.currency_code.clone())
        .build()
        .map_err(|err| anyhow::anyhow!(err))
}
