use crate::language::Language;
use anyhow::{Context, Result};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt::{self, Display};
use std::fs;
use std::path::{Path, PathBuf};
use tree_sitter::{Parser, Query, QueryCursor};

#[derive(Debug)]
pub struct Extractor {
    language: Language,
    query: Query,
    captures: Vec<String>,
}

impl Extractor {
    pub fn new(language: Language, query: Query) -> Extractor {
        // TODO: disable capturing names that start with _ so it's easier to
        // make temporary matches for use in `#eq?` and stuff.

        Extractor {
            language,
            captures: query.capture_names().to_vec(),
            query,
        }
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn extract_from_file(
        &self,
        path: &Path,
        parser: &mut Parser,
    ) -> Result<Option<ExtractedFile>> {
        let source =
            fs::read(path).with_context(|| format!("could not read {}", path.display()))?;

        // TODO: do we need to avoid calling self.language.language()
        // repeatedly? Is this something we can move to the init somehow?
        parser
            .set_language(self.language.language())
            .context("could not set language")?;

        let tree = parser
            .parse(&source, None)
            // note: this could be a timeout or cancellation, but we don't set
            // that so we know it's always a language error. Buuuut we also
            // always set the language above so if this happens we also know
            // it's an internal error.
            .with_context(|| {
                format!(
                    "could not parse {}. This is an internal error and should be reported.",
                    path.display()
                )
            })?;

        let mut cursor = QueryCursor::new();

        let extracted_matches = cursor
            .matches(&self.query, tree.root_node(), |node| {
                // TODO: do this conditionally if there are no matchers?
                node.utf8_text(&source).unwrap_or("")
            })
            .flat_map(|query_match| query_match.captures)
            .map(|capture| {
                let node = capture.node;

                Ok(ExtractedMatch {
                    text: node
                        .utf8_text(&source)
                        .map(|unowned| unowned.to_string())
                        .context("could not extract text from capture")?,
                    start: Point(node.start_position()),
                    end: Point(node.end_position()),
                    kind: node.kind(),
                    // note: the cast here could potentially break if run
                    // on a 16-bit microcontroller. I don't think this is
                    // a huge problem, though, since even the gnarliest
                    // queries I've written have something on the order of
                    // 20 matches. Nowhere close to 2^16!
                    //
                    // TODO: is the clone going to be acceptably fast here?
                    name: self.captures[capture.index as usize].clone(),
                })
            })
            .collect::<Result<Vec<ExtractedMatch>>>()?;

        if extracted_matches.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ExtractedFile {
                file: path.to_path_buf(),
                matches: extracted_matches,
            }))
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExtractedFile {
    file: PathBuf,
    matches: Vec<ExtractedMatch>,
}

impl Display for ExtractedFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for extraction in &self.matches {
            write!(
                f,
                "{}:{}:{}:{}:{}\n",
                self.file.display(),
                extraction.start.0.row,
                extraction.start.0.column,
                extraction.name,
                extraction.text
            )?
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ExtractedMatch {
    kind: &'static str,
    name: String,
    text: String,
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Point(tree_sitter::Point);

impl Serialize for Point {
    fn serialize<S>(&self, sz: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut out = sz.serialize_struct("Point", 2)?;
        out.serialize_field("row", &self.0.row)?;
        out.serialize_field("column", &self.0.column)?;
        out.end()
    }
}
