/*

 * ******************************************************************************************
 * Copyright (c) 2019 Pascal Kuthe. This file is part of the VARF project.
 * It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/jamescoding/VARF/blob/master/LICENSE.
 *  No part of VARF, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 * *****************************************************************************************

 Adapted from https://github.com/rust-lang/rust src/librustc/symbol.rs under MIT-License

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/

use core::fmt;
use std::clone::Clone;
use std::convert::Into;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

use ahash::AHashMap as HashMap;
use bumpalo::Bump;

use crate::symbol::keywords::EMPTY_SYMBOL;
use crate::symbol::statics::with_interner;
use crate::Span;

#[derive(Copy, Clone, Debug)]
pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}
const DUMMY_SPAN: Span = Span::new_short_span(0, 0);
impl Ident {
    #[inline]
    /// Constructs a new identifier from a symbol and a span.
    pub const fn new(name: Symbol, span: Span) -> Self {
        Self { name, span }
    }

    pub const fn empty() -> Ident {
        Self {
            name: EMPTY_SYMBOL,
            span: DUMMY_SPAN,
        }
    }
    /// Maps a string to an identifier with a dummy span.
    pub fn from_str(string: &str) -> Ident {
        Self::new(Symbol::intern(string), DUMMY_SPAN)
    }

    /// Maps a string and a span to an identifier.
    pub fn from_str_and_span(string: &str, span: Span) -> Ident {
        Ident::new(Symbol::intern(string), span)
    }

    pub fn without_first_quote(self) -> Ident {
        Ident::new(
            Symbol::intern(self.as_str().trim_start_matches('\'')),
            self.span,
        )
    }

    /// Convert the name to a `SymbolStr`. This is a slowish operation because
    /// it requires locking the symbol interner.
    pub fn as_str(self) -> SymbolStr {
        self.name.as_str()
    }
}

impl PartialEq for Ident {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.name, f)
    }
}

/// An interned string.
///
/// Internally, a `Symbol` is implemented as an index, and all operations
/// (including hashing, equality, and ordering) operate on that index. The use
/// of `rustc_index::newtype_index!` means that `Option<Symbol>` only takes up 4 bytes,
/// because `rustc_index::newtype_index!` reserves the last 256 values for tagging purposes.
///
/// Note that `Symbol` cannot directly be a `rustc_index::newtype_index!` because it
/// implements `fmt::Debug`, `Encodable`, and `Decodable` in special ways.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(SymbolIndex);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SymbolIndex(u32);
impl SymbolIndex {
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl Symbol {
    const fn new(n: u32) -> Self {
        Symbol(SymbolIndex(n))
    }

    /// Maps a string to its interned representation.
    pub fn intern(string: &str) -> Self {
        with_interner(|interner| interner.intern(string))
    }

    /// Access the symbol's chars. This is a slowish operation because it
    /// requires locking the symbol interner.
    pub fn with<F: FnOnce(&str) -> R, R>(self, f: F) -> R {
        with_interner(|interner| f(interner.get(self)))
    }

    /// Convert to a `SymbolStr`. This is a slowish operation because it
    /// requires locking the symbol interner.
    pub fn as_str(self) -> SymbolStr {
        with_interner(|interner| unsafe {
            SymbolStr {
                string: std::mem::transmute::<&str, &str>(interner.get(self)),
            }
        })
    }

    pub fn as_u32(self) -> u32 {
        let index = self.0;
        index.0
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with(|str| fmt::Debug::fmt(&str, f))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with(|str| fmt::Display::fmt(&str, f))
    }
}

// The `&'static str`s in this type actually point into the arena.
#[derive(Default)]
pub struct Interner {
    arena: Bump,
    names: HashMap<&'static str, Symbol>,
    strings: Vec<&'static str>,
}

impl Interner {
    fn prefill(init: &[&'static str]) -> Self {
        Interner {
            strings: init.into(),
            names: init.iter().copied().zip((0..).map(Symbol::new)).collect(),
            ..Default::default()
        }
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol::new(self.strings.len() as u32);
        let string = self.arena.alloc_str(string);
        // It is safe to extend the arena allocation to `'static` because we only access
        // these while the arena is still alive.
        let string: &'static str = unsafe { &*(string as *const str) };
        self.strings.push(string);
        self.names.insert(string, name);
        name
    }

    // Get the symbol as a string. `Symbol::as_str()` should be used in
    // preference to this function.
    pub fn get(&self, symbol: Symbol) -> &str {
        self.strings[symbol.0.as_usize()]
    }
}
pub mod keywords {
    use crate::symbol::{Symbol, SymbolIndex};

    pub(super) static EMPTY_SYMBOL_STR: &str = ".EMPTY";
    pub const EMPTY_SYMBOL: Symbol = Symbol(SymbolIndex(0));

    pub(super) static FLOW_STR: &str = "flow";
    pub const FLOW: Symbol = Symbol(SymbolIndex(1));
    pub(super) static POTENTIAL_STR: &str = "potential";
    pub const POTENTIAL: Symbol = Symbol(SymbolIndex(2));
}
mod statics {
    use std::sync::Mutex;

    use crate::symbol::keywords::{EMPTY_SYMBOL_STR, FLOW_STR, POTENTIAL_STR};
    use crate::symbol::Interner;

    static interner: Mutex<Interner> = Mutex::new(Interner::prefill(&[
        EMPTY_SYMBOL_STR,
        FLOW_STR,
        POTENTIAL_STR,
    ]));
    #[inline]
    pub(super) fn with_interner<T, F: FnOnce(&mut Interner) -> T>(f: F) -> T {
        f(&mut *interner.lock().unwrap())
    }
}

/// An alternative to `Symbol`, useful when the chars within the symbol need to
/// be accessed. It deliberately has limited functionality and should only be
/// used for temporary values.
///
/// Because the interner outlives any thread which uses this type, we can
/// safely treat `string` which points to interner data, as an immortal string,
/// as long as this type never crosses between threads.
//
// FIXME: ensure that the interner outlives any thread which uses `SymbolStr`,
// by creating a new thread right after constructing the interner.
#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct SymbolStr {
    string: &'static str,
}

// This impl allows a `SymbolStr` to be directly equated with a `String` or
// `&str`.
impl<T: std::ops::Deref<Target = str>> std::cmp::PartialEq<T> for SymbolStr {
    fn eq(&self, other: &T) -> bool {
        self.string == other.deref()
    }
}

/// This impl means that if `ss` is a `SymbolStr`:
/// - `*ss` is a `str`;
/// - `&*ss` is a `&str`;
/// - `&ss as &str` is a `&str`, which means that `&ss` can be passed to a
///   function expecting a `&str`.
impl std::ops::Deref for SymbolStr {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        self.string
    }
}

impl fmt::Debug for SymbolStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.string, f)
    }
}

impl fmt::Display for SymbolStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.string, f)
    }
}
