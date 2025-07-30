Parsing instructions and expressions

- Depending on how we want to display instructions in the debugger, we may need a full on wasm parser and AST. Since wasm understands block statements, loops, conditionals, and expressions, we will want to display these constructs in the debugger. This means we need to represent them in code.
- Parsing: the parsing should not consume the input if it is not successful. There are instances where the next symbol could be one of several different types, and we can test that by trying to parse each type in succession. If one type fails to parse, try the next, etc. To make this work, the reader must be reset back to its original position upon each failure.
- Representing instructions: currently we're using parameterized enum variants for things like "signedness" and numeric type. This reduces the number of variants in our enums, but increases the verbosity of code. Six in one hand, half a dozen in the other, I guess. Maybe it would be better to not parameterize these things and expand them all into separate instructions. I kind of think enum parameters should be reserved for values, such as constants and labels, and for nested expressions and block statements.

TODO: need the full wasm grammar, then perhaps we can model that using Rust types.
