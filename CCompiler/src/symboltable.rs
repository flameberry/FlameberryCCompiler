use std::collections::HashMap;

use crate::analysis::ast::StorageClassSpecifier;
use crate::errors::{CompilerError, CompilerErrorKind};
use crate::typedefs::{Constant, TypeInfo};

#[derive(Debug, Clone)]
pub struct SymbolDefinition {
    pub typeinfo: TypeInfo,
    pub storageclass: Option<StorageClassSpecifier>,
    pub value: Option<Constant>,
    pub scopeid: u32,
}

#[derive(Debug)]
struct SymbolHashMapEntry {
    // Stores the indices of symbols defined in all scopes
    // Note: These indices are to be stored sorted in ascending order of their depth
    // ...so that binary search can be performed for every lookup
    depthsortedindices: Vec<i32>,
}

impl SymbolHashMapEntry {
    fn default() -> Self {
        SymbolHashMapEntry {
            depthsortedindices: Vec::new(),
        }
    }

    fn bsearch(&self, scopeid: u32, symbolbuffer: &[SymbolDefinition]) -> i32 {
        let mut beg = 0;
        let mut end = self.depthsortedindices.len();
        let mut mid = (beg + end) / 2;
        let mut probecount = 0;

        while beg <= end {
            let midscopeid = symbolbuffer[self.depthsortedindices[mid] as usize].scopeid;
            probecount += 1;

            if scopeid < midscopeid {
                end = mid - 1;
                mid = (beg + end) / 2;
            } else if scopeid > midscopeid {
                beg = mid + 1;
                mid = (beg + end) / 2;
            } else {
                return self.depthsortedindices[mid];
            }
        }
        println!("Probe count for searching symbol within scopes: {probecount}");

        // Return -1 indicating that symbol definition within the given scopeid was not found
        -1
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
        match self.hashmap.get(name) {
            Some(entry) => match entry.bsearch(scopeid, &self.symbolbuffer) {
                -1 => None,                                        // Return None when symbol not found
                index => Some(&self.symbolbuffer[index as usize]), // Return symbol definition when found
            },
            None => None,
        }
    }

    /// Returns all symbols with the given name (Includes same symbol names from multiple scopes)
    pub fn lookup_all(&self, name: &str) -> Option<Vec<SymbolDefinition>> {
        self.hashmap.get(name).map(|entry| {
            entry
                .depthsortedindices
                .iter()
                .map(|idx| self.symbolbuffer[*idx as usize].clone())
                .collect()
        })
    }

    /// Insert a symbol into the table without checking whether it already exists
    pub fn insert_unsafe(
        &mut self,
        name: &str,
        scopeid: u32,
        typeinfo: TypeInfo,
        storageclass: Option<StorageClassSpecifier>,
        value: Option<Constant>,
    ) {
        let index = self.symbolbuffer.len() as i32;
        let mut entry = SymbolHashMapEntry::default();
        entry.depthsortedindices.push(index);

        self.hashmap.insert(name.to_string(), entry);

        self.symbolbuffer.push(SymbolDefinition {
            typeinfo,
            storageclass,
            value,
            scopeid,
        });
    }

    /// By default this looks up the symbol first then inserts it in a safe way
    pub fn insert(
        &mut self,
        name: &str,
        scopeid: u32,
        typeinfo: TypeInfo,
        storageclass: Option<StorageClassSpecifier>,
        value: Option<Constant>,
    ) -> Result<(), CompilerError> {
        match self.lookup(name, scopeid) {
            Some(symboldef) => Err(CompilerError {
                kind: CompilerErrorKind::InternalError,
                message: format!(
                    "Failed to insert symbol: Duplicate Symbols are not allowed in the Symbol Table.\n{:?}",
                    symboldef
                ),
                location: None,
            }),
            None => {
                // Symbol doesn't exist so safely can be inserted
                self.insert_unsafe(name, scopeid, typeinfo, storageclass, value);
                Ok(())
            }
        }
    }
}
