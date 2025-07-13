use std::fs::File;
use std::io::{stdin, stdout};

use clap::Parser;
use dbo_csv::{DboRecord, DboStatement, deserialize_statement};
use dbo2taxer::config::TaxerConfig;
use dbo2taxer::filter::DateFilter;
use taxer_csv::{TaxerRecord, serialize_taxer};

use crate::cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let taxer_config = TaxerConfig::from_configuration()?;
    let filter = DateFilter::new(cli.quarter, cli.year);

    let statement = match cli.input {
        Some(input_path) => {
            let input_file = File::open(input_path)?;
            deserialize_statement(input_file)?
        }
        None => deserialize_statement(stdin())?,
    };

    let records = convert_records(statement, &taxer_config)?
        .into_iter()
        .filter(|r| filter.matches(r.date))
        .collect::<Vec<_>>();
    match cli.output {
        Some(output_path) => {
            let output_file = File::create(output_path)?;
            serialize_taxer(output_file, &records)?;
        }
        None => serialize_taxer(stdout(), &records)?,
    };
    Ok(())
}

fn convert_records(
    statement: DboStatement,
    config: &TaxerConfig,
) -> anyhow::Result<Vec<TaxerRecord>> {
    let mut taxer_operations = vec![];
    for dbo in statement.into_inner() {
        if dbo.credit.is_some() {
            taxer_operations.push(convert_record(dbo, config)?);
        }
    }
    Ok(taxer_operations)
}

fn convert_record(record: DboRecord, config: &TaxerConfig) -> anyhow::Result<TaxerRecord> {
    TaxerRecord::builder()
        .tax_code_raw(record.party_tax_id)?
        .amount_raw(record.credit.unwrap())?
        .date(record.operation_date)
        .comment(record.payment_purpose)
        .operation(config.operation.clone())
        .income_type(config.income_type.clone())
        .account_name(config.account_name.clone())
        .currency_code(record.currency.clone())
        .build()
        .map_err(|err| anyhow::anyhow!(err))
}
