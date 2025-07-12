# DBO2TAXER

Simple cli utility to convert banking statements in DBO format to CSV format of [Taxer](https://taxer.ua).

## Usage

Pass dbo records in standard input and receive taxer operations in standard output.

```bash
cat dbo-statement.csv | dbo2taxer > taxer-operations.csv
```
