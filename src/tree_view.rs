use anyhow::{Context, Result};
use std::collections::HashSet;
use std::io::Write;
use tree_sitter::Tree;

pub fn tree_view(tree: &Tree, source: &[u8], mut out: impl Write) -> Result<()> {
    let mut cursor = tree.walk();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut indent = 0;
    let indent_str = String::from("  ");

    loop {
        let node = cursor.node();
        let node_visited = visited.contains(&node.id());
        let is_leaf = node.child_count() == 0;

        visited.insert(node.id());

        if !node_visited {
            write!(
                out,
                "{}{} {}:{}",
                indent_str.repeat(indent),
                node.kind(),
                node.start_position().row + 1,
                node.start_position().column + 1,
            )?;
            if is_leaf {
                writeln!(
                    out,
                    ": {}",
                    node.utf8_text(source)
                        .context("could not get node source")?
                )?;
            } else {
                writeln!(out)?;
            }
        }

        if !is_leaf && !node_visited && cursor.goto_first_child() {
            indent += 1;
            continue;
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        if cursor.goto_parent() {
            indent -= 1;
            continue;
        }

        break;
    }

    Ok(())
}
