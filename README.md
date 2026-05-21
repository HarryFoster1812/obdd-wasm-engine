# OBDD WASM Engine

A step-by-step Ordered Binary Decision Diagram (OBDD) engine compiled to WebAssembly and deployed as an interactive visualization tool at:

https://tools.harryfoster.tech

This engine allows users to input propositional logic formulas as well as the desired ordering and observe their full transformation into an OBDD in real time, including parsing, simplification, recursion, and DAG construction.

Built with Rust and compiled to WebAssembly for execution directly in the browser.


---

## Overview

This engine takes a propositional logic formula (e.g. `~a & b -> c`) and:

1. Lexes and parses it into an AST
2. Simplifies the formula using boolean identities
3. Constructs an Ordered Binary Decision Diagram (OBDD)
4. Exposes each execution step for visualization
5. Allows step-by-step forward/backward execution

---

## Supported Syntax

| Operator  | Meaning  | Example   |
| --------- | -------- | --------- |
| `~` / `!` | NOT      | `~a`      |
| `&`       | AND      | `a & b`   |
| `\|`      | OR       | `a \| b`  |
| `->`      | IMPLIES  | `a -> b`  |
| `<->`     | IFF      | `a <-> b` |
| `( )`     | Grouping | `(a & b)` |

Reserved identifiers:

* `T` → True
* `F` → False

---

## Operator Rules

All binary operators are **left-associative**.

This ensures deterministic parsing of ambiguous expressions:

```
a -> b -> c  ≡  (a -> b) -> c
```

---

## Architecture

The system is split into four modules:

* **lexer** → tokenization of input strings
* **parser** → Pratt parser producing AST
* **formula** → logical expression representation
* **engine** → OBDD construction + execution tracing

---

## Execution Model

The engine is designed around **state snapshots**:

* Each step mutates a cloned execution state
* All states are stored in `state_history`
* UI can replay or rewind execution deterministically
* Execution is driven by a stack of frames

---

## OBDD Algorithm Specification

This engine directly implements a step-wise version of the classical OBDD construction algorithm.

### OBDD Construction

```
procedure obdd(F)
input: propositional formula F
parameters: global DAG D
output: node n representing F
begin
    F := simplify(F)

    if F = ⊥ then return 0
    if F = ⊤ then return 1

    p := max_variable(F)

    n1 := obdd(F[p := false])
    n2 := obdd(F[p := true])

    return integrate(n1, p, n2, D)
end
```

---

### Node Integration

```
procedure integrate(n1, p, n2, D)
input: nodes n1, n2 in D, variable p
output: node n representing (if p then n2 else n1)

begin
    if n1 = n2 then return n1

    if D contains node (p, n1, n2) then
        return existing node

    create new node n = (p, n1, n2)
    add n to D

    return n
end
```

---

## Mapping to Implementation

| Algorithm Concept | Rust Implementation          |
| ----------------- | ---------------------------- |
| simplify(F)       | `simplify_step()`            |
| max_variable(F)   | `choose_variable()`          |
| F[p := value]     | `substitute_bool_ast_walk()` |
| recursive obdd    | `recurse_var()`              |
| integrate()       | `call_integrate()`           |
| DAG D             | `unique_table + nodes`       |

---

## OBDD Node Structure

```rust
Node {
    id: usize,
    var: Option<String>,
    left: Option<usize>,
    right: Option<usize>,
}
```

Terminal nodes:

* `0` → False
* `1` → True

---

## WASM API

### Create Engine

```ts
const engine = new WasmOBDDEngine(formula, ordering);
```

---

### Step Forward

```ts
engine.step();
```

Advances execution by one state transition.

---

### Step Back

```ts
engine.step_back();
```

Rewinds execution.

---

### Get State

```ts
const state = engine.get_state();
```

Returns:

```ts
{
  step_number: number,
  nodes: Node[],
  execution_stack: ExecutionFrame[],
  message: string
}
```

This is the primary interface for UI rendering.

---

### Validate Formula

```ts
validate_formula("(a & b) -> c");
```

Returns `true` if syntactically valid.

---

## Execution Frames

The engine exposes internal computation steps for visualization:

### OBDD Frames

* simplify
* check bottom/top
* choose variable
* recurse low/high
* integrate call/return

### Integration Frames

* check equality
* lookup node
* create node
* return

This enables a **fully animated OBDD construction debugger**.

---

## Example Usage

```ts
const engine = new WasmOBDDEngine("~a & b -> c", ["a", "b", "c"]);

while (true) {
  const state = engine.get_state();
  render(state);

  engine.step();
}
```

---

## Build

```bash
wasm-pack build --target web
```
