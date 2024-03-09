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

fn get_first_plain(blocks: Vec<pandoc::Block>) -> String {
    match blocks
        .expect_one()
        .unwrap_or_else(|| panic!("Unexpected get_first_plain on {:?} of len > 1", blocks))
    {
        pandoc::Block::Plain(inlines) => {
            match inlines.expect_one().unwrap_or_else(|| {
                panic!("Unexpected get_first plain on multiple inlines {inlines:?}")
            }) {
                pandoc::Inline::Str(s) => s.clone(),
                other => panic!("Unexpected inline {other:?}"),
            }
        }
        other => panic!("Unexpected block {other:?}"),
    }
}

fn parse_table_head(head: pandoc::TableHead) -> Vec<String> {
    rows_to_vec_str(
        head.rows
            .expect_one()
            .unwrap_or_else(|| panic!("Unexpected {:?} of len > 1", head.rows)),
    )
}

fn parse_table_bodies(bodies: Vec<pandoc::TableBody>) -> Vec<Vec<String>> {
    bodies
        .expect_one()
        .unwrap_or_else(|| panic!("Unexpected table bodies {:?} of len != 1", bodies))
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

trait ExpectOne<T> {
    fn expect_one(&self) -> Option<&T>;
}

impl<T> ExpectOne<T> for Vec<T> {
    fn expect_one(&self) -> Option<&T> {
        match &self[..] {
            [] => None,
            [s] => Some(s),
            _ => None,
        }
    }
}
