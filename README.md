# DBO2TAXER

Simple cli utility to convert banking statements in DBO format to CSV format of [Taxer](https://taxer.ua).

## Usage

Pass dbo records in standard input and receive taxer operations in standard output.

```bash
cat dbo-statement.csv | dbo2taxer > taxer-operations.csv
```

### Overriding additional values

You can override a few additional values making Taxer format more complete.
The values are:

- Operation type
- Income kind
- Account name
- Currency code

Here's the example

```bash
cat dbo-statement.csv | DBO2TAXER_ACCOUNT_NAME="ФОП" dbo2taxer
```

The list of environment variables and default values.

| Variable               | Default value    | Meaning                            |
| ---------------------- | ---------------- | ---------------------------------- |
| DBO2TAXER_ACCOUNT_NAME | ""               | Name of account in the income list |
| DBO2TAXER_OPERATION    | "Дохід"          | Type of operation                  |
| DBO2TAXER_INCOME_TYPE  | "Основний дохід" | The type of income for taxation    |

### Specify input and output files

The command supports `-i` and `-o` parameters to specify input and output files respectively.

```bash
dbo2csv -i dbo-statement.csv -o taxer-operations.csv
```

With that you can omit tinkering with stream redirection.

### Filter by years and quarters

You may use `-q` and `-y` options to include only records of a given quarter and/or year.

Examples:

| Parameters    | Meaning                        |
| ------------- | ------------------------------ |
| -q q1         | Only records of Q1 of any year |
| -q q1 -y 2025 | Only records of Q1 of 2025     |
| -y 2025       | All records of 2025            |

The tool processes all records if you don't specify any filtering.
