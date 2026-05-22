// Event Script demo
// Showcasing Pratt parsing (operator precedence) and type resolution

// === Pratt Parsing: operator precedence ===
// * and / bind tighter than + and -
let a = 1 + 2 * 3;       // 2*3 first -> 7
let b = (1 + 2) * 3;     // parens override -> 9
let c = 1 + 2 * 3 == 7;  // 2*3=6, 1+6=7, 7==7 -> true

// === Type Resolution ===
// The type checker infers the smallest fitting integer type
let t_i8  = 42;          // fits in i8  (8  bits)
let t_i16 = 300;         // fits in i16 (16 bits)
let t_i32 = 100000;      // fits in i32 (32 bits)
let t_i64 = 3000000000;  // needs i64  (64 bits)

// === Block scope ===
{
    let scoped = 99;
}
