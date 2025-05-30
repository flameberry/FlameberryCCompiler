use std::collections::HashMap;
use std::fmt;

use crate::analysis::ast::StorageClassFlags;
use crate::common::errors::{CompilerError, CompilerErrorKind};
use crate::common::typedefs::{Constant, Type};

#[derive(Debug, Clone)]
pub struct SymbolDefinition {
    pub typeinfo: Type,
    pub storageclass: StorageClassFlags,
    pub value: Option<Constant>,
    pub scopeid: u32,
}

#[derive(Debug)]
struct SymbolHashMapEntry {
    // Stores the indices of symbols defined in all scopes
    // Note: These indices are to be stored sorted in ascending order of their depth
    // ...so that binary search can be performed for every lookup
    depthsortedindices: Vec<usize>,
}

impl SymbolHashMapEntry {
    fn default() -> Self {
        SymbolHashMapEntry {
            depthsortedindices: Vec::new(),
        }
    }

    #[allow(clippy::comparison_chain)]
    fn bsearch(&self, scopeid: u32, symbolbuffer: &[SymbolDefinition]) -> Option<usize> {
        let mut beg = 0;
        let mut end = self.depthsortedindices.len() as i32 - 1;
        let mut mid = (beg + end) / 2;

        while beg <= end {
            let midscopeid = symbolbuffer[self.depthsortedindices[mid as usize]].scopeid;

            if scopeid < midscopeid {
                end = mid - 1;
                mid = (beg + end) / 2;
            } else if scopeid > midscopeid {
                beg = mid + 1;
                mid = (beg + end) / 2;
            } else {
                return Some(self.depthsortedindices[mid as usize]);
            }
        }

        // Return None indicating that symbol definition within the given scopeid was not found
        None
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    hashmap: HashMap<String, SymbolHashMapEntry>,
    symbolbuffer: Vec<SymbolDefinition>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            hashmap: HashMap::new(),
            symbolbuffer: Vec::new(),
        }
    }

    /// Returns the symbol with the given name and the given scope ID
    pub fn lookup(&self, name: &str, scopeid: u32) -> Option<&SymbolDefinition> {
        self.hashmap
            .get(name)
            .and_then(|entry| entry.bsearch(scopeid, &self.symbolbuffer))
            .map(|index| &self.symbolbuffer[index])
    }

    /// Returns all symbols with the given name (Includes same symbol names from multiple scopes)
    pub fn lookup_all(&self, name: &str) -> Option<Vec<SymbolDefinition>> {
        self.hashmap.get(name).map(|entry| {
            entry
                .depthsortedindices
                .iter()
                .map(|idx| self.symbolbuffer[*idx].clone())
                .collect()
        })
    }

    pub fn insert(
        &mut self,
        name: &str,
        scopeid: u32,
        typeinfo: Type,
        storageclass: StorageClassFlags,
        value: Option<Constant>,
    ) -> Result<(), CompilerError> {
        let index = self.symbolbuffer.len();

        self.symbolbuffer.push(SymbolDefinition {
            typeinfo,
            storageclass,
            value,
            scopeid,
        });

        if let Some(entry) = self.hashmap.get_mut(name) {
            match entry.bsearch(scopeid, &self.symbolbuffer) {
                None => {
                    // Insert into the existing symbol table hash map entry
                    entry.depthsortedindices.push(index);
                    Ok(())
                }
                Some(idx) => Err(CompilerError {
                    kind: CompilerErrorKind::InternalError,
                    message: format!(
                        "Failed to insert symbol: Duplicate Symbols are not allowed in the Symbol Table.\n{:?}",
                        self.symbolbuffer[idx]
                    ),
                    location: None,
                }),
            }
        } else {
            let mut entry = SymbolHashMapEntry::default();
            entry.depthsortedindices.push(index);
            self.hashmap.insert(name.to_string(), entry);
            Ok(())
        }
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "----------------------------------- Symbol Table -----------------------------------"
        )?;

        writeln!(
            f,
            "{0: <20} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
            "Name", "Type", "Storage Class", "Value", "Scope"
        )?;

        for (symbolname, symboldef) in &self.hashmap {
            for idx in &symboldef.depthsortedindices {
                let symbol = &self.symbolbuffer[*idx];
                writeln!(
                    f,
                    "{0: <20} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
                    symbolname,
                    symbol.typeinfo.to_string(),
                    format!("{:b}", symbol.storageclass),
                    symbol.value.as_ref().map_or("None".to_string(), |v| v.to_string()),
                    symbol.scopeid
                )?;
            }
        }
        Ok(())
    }
}
