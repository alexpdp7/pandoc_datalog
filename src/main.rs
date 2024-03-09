use pandoc_types::definition as pandoc;

fn main() {
    let d: pandoc::Pandoc = serde_json::from_reader(std::io::stdin()).unwrap();
    for block in d.blocks {
        match block {
            pandoc::Block::Table(table) => process_table(parse_table(table)),
            _ => println!("Skipping {block:?}"),
        }
    }
}

#[derive(Debug)]
struct Table {
    name: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

fn process_table(table: Table) {
    dbg!(table);
}

fn parse_table(table: pandoc::Table) -> Table {
    match table {
        pandoc::Table {
            caption: pandoc::Caption {
                long: long_caption, ..
            },
            head,
            bodies,
            ..
        } => Table {
            name: get_first_plain(long_caption),
            columns: parse_table_head(head),
            rows: parse_table_bodies(bodies),
        },
    }
}

fn get_first_plain(blocks: Vec<pandoc::Block>) -> String {
    match blocks.first().expect(&format!(
        "Unexpected get_first_plain on {:?} of len > 1",
        blocks
    )) {
        pandoc::Block::Plain(inlines) => {
            if inlines.len() != 1 {
                panic!("Unexpected get_first plain on multiple inlines {inlines:?}");
            }
            match inlines.get(0).unwrap() {
                pandoc::Inline::Str(s) => s.clone(),
                other => panic!("Unexpected inline {other:?}"),
            }
        }
        other => panic!("Unexpected block {other:?}"),
    }
}

fn parse_table_head(head: pandoc::TableHead) -> Vec<String> {
    if head.rows.len() != 1 {
        panic!("Unexpected {:?} of len > 1", head.rows);
    }
    rows_to_vec_str(head.rows.get(0).unwrap())
}

fn parse_table_bodies(bodies: Vec<pandoc::TableBody>) -> Vec<Vec<String>> {
    if bodies.len() != 1 {
        panic!("Unexpected table bodies {:?} of len > 1", bodies);
    }
    bodies
        .first()
        .unwrap()
        .body
        .iter()
        .map(rows_to_vec_str)
        .collect()
}

fn rows_to_vec_str(row: &pandoc::Row) -> Vec<String> {
    row.cells
        .iter()
        .map(|c| c.content.clone())
        .map(get_first_plain)
        .collect()
}
