use itertools::Itertools;

use pandoc_types::definition as pandoc;

fn main() {
    let d: pandoc::Pandoc = serde_json::from_reader(std::io::stdin()).unwrap();
    for block in d.blocks {
        match block {
            pandoc::Block::Table(table) => process_table(&parse_table(&table)),
            _ => println!("Skipping {block:?}"),
        }
    }
}

#[derive(Debug)]
struct Table<'a> {
    name: &'a str,
    columns: Vec<&'a str>,
    rows: Vec<Vec<&'a str>>,
}

fn process_table(table: &Table) {
    dbg!(table);
}

fn parse_table(table: &pandoc::Table) -> Table {
    let pandoc::Table {
        caption: pandoc::Caption {
            long: long_caption, ..
        },
        head,
        bodies,
        ..
    } = table;
    Table {
        name: get_first_plain(long_caption),
        columns: parse_table_head(head),
        rows: parse_table_bodies(bodies),
    }
}

fn get_first_plain(blocks: &[pandoc::Block]) -> &str {
    match blocks
        .iter()
        .exactly_one()
        .unwrap_or_else(|e| panic!("{e} in get_first_plain on {:?} of len > 1", blocks))
    {
        pandoc::Block::Plain(inlines) => {
            match inlines
                .iter()
                .exactly_one()
                .unwrap_or_else(|e| panic!("{e} get_first plain on multiple inlines {inlines:?}"))
            {
                pandoc::Inline::Str(s) => s,
                other => panic!("Unexpected inline {other:?}"),
            }
        }
        other => panic!("Unexpected block {other:?}"),
    }
}

fn parse_table_head(head: &pandoc::TableHead) -> Vec<&str> {
    rows_to_vec_str(
        head.rows
            .iter()
            .exactly_one()
            .unwrap_or_else(|e| panic!("{e} {:?} of len > 1", head.rows)),
    )
}

fn parse_table_bodies(bodies: &[pandoc::TableBody]) -> Vec<Vec<&str>> {
    bodies
        .iter()
        .exactly_one()
        .unwrap_or_else(|e| panic!("{e} table bodies {:?} of len != 1", bodies))
        .body
        .iter()
        .map(rows_to_vec_str)
        .collect::<Vec<_>>()
}

fn rows_to_vec_str(row: &pandoc::Row) -> Vec<&str> {
    row.cells
        .iter()
        .map(|c| get_first_plain(&c.content))
        .collect::<Vec<_>>()
}
