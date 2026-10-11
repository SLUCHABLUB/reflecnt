# reflecnt

A dynamically typed scripting language without runtime reflections

## Types

- Null
- Boolean
- Number (complex)
- Text
- List
- Table
- Procedure
- (External Object)

## Object Terminals

### If statements

There is only one "truthy" value and it is `TRUE`, al others are "falsy".

### Display

All objects can be stringified

- Text: The text

### Debug

- Null: "null"
- Boolean: "true" or "false"
- Number: Rectangular form with least number of required digits
- List: [debug(value)]
- Table: [debug(key): debug(value)] ordered by the keys
- Procedure: Source code (and scope?)

### Iteration

- Null: Nothing
- Boolean: Nothing
- Number: Nothing
- Text: Graphemes
- List: The items
- Table: [key, value]
- Procedure: Nothing

### Ordering (not the operators, just for keys &c.)

+ null
+ false
+ true
+ Reals
+ Complex Numbers (lexicographically)
+ Text (unicode IG)
+ List (lexicographically)
+ Table (ordered, then lexicographically by pairs IG)

### Size

- Null: 0
- Boolean: 1 or 0
- Number: Magnitude
- Text: Grapheme count
- List: Length
- Table: Entry count
- Procedure: TODO

### Application

- Null: null
- Boolean: null
- Number: null
- Text: Grapheme Indexing
- List: Indexing
- Table: Indexing
- Procedure: Running

### Assignment

- Null: Create Table
- Boolean: Nothing
- Number: Nothing
- Text: Grapheme Assignment or Table Coercion
- List: Assignment or Table Coercion
- Table: Assignment
- Procedure: Modification? o_o

## Keywords

- `break`: Breaks from a loop
- `continue`: Skips to the next iteration of a loop
- `else`: Adds a non-true branch to an if-statement
- `for`: Iterates over a collection
- `if`: Branches if a value is true
- `let`: Introduces a new binding in the current scope
- `procedure`: Creates a procedure expression
- `return`: Returns a value from a procedure
- `while`: Loops while a condition is true

## Procedures

Procedures are run in the scope in which they are defined.

```
let print_number = procedure {
    print(number)
}

print_number() # prints null
let number = 2
print_number() # prints 2
number = 3
print_number() # prints 3
```

### Scope

I imagine scopes behaving something like this:

```rust
struct Scope {
    parent: Option<Arc<Scope>>,
    declarations: Table,
}

/// A procedure that has been declared in a script.
struct DeclaredProcedure {
    /// Used when instantiating the procedure.
    parent_scope: Arc<Scope>,
    ...
}

/// A currently running procedure.
/// It will be dropped when it returns.
struct RunningProcedure {
    /// This the `DeclaredProcedure::parent_scope` as `Scope::parent`.
    scope: Arc<Scope>,
    ...
}
```

When a module (file) is loaded, it's scope will be dropped, unless it declares some procedure, since they will clone an `Arc` to the scope.

## Examples

Hello World
```
print("Hello, World!")
```

Fizz Buzz
```
for number in range(100) {
    let divisible_by_3 = number % 3
    let divisible_by_5 = number % 5

    if divisible_by_3 & divisible_by_5 {
        print("FizzBuzz")
    } else if divisible_by_3 {
        print("Fizz")
    } else if divisible_by_5 {
        print("Buzz")
    } else {
        print(number)
    }
}
```

Fibonacci
```
let fibonacci = procedure n {
    if n == 0 | n == 1 {
        return 0
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```

Brain fuck
```
let instruction_pointer = 0
let data_pointer = 0
let memory = list(size: 1024, element: 0)

let program = text_from_utf_8(stdin)
let at_the_end = procedure {
    return program(instruction_pointer) == null;
}

let seek_closing_bracket = procedure {
    let depth = 0
    let target_depth = 0

    let found = false

    while !found & !at_the_end() {
        instruction_pointer += 1
        let instruction = program(instruction_pointer)

        if instruction == "[" {
            depth += 1
        } else if instruction == "]" {
            if depth == target_depth {
                found = true
            } else {
                depth -= 1
            }
        }
    }
}

let seek_opening_bracket = procedure {
    let depth = 0
    let target_depth = 0

    let found = false

    while !found & !at_the_end() {
        instruction_pointer -= 1
        let instruction = program(instruction_pointer)

        if instruction == "]" {
            depth += 1
        } else if instruction == "[" {
            if depth == target_depth {
                found = true
            } else {
                depth -= 1
            }
        }
    }
}

while !at_the_end() {
    let instruction = program(instruction_pointer)

    if instruction == ">" {
        data_pointer += 1
    } else if instruction == "<" {
        data_pointer -= 1
    } else if instruction == "+" {
        memory(data_pointer) += 1
        memory(data_pointer) %= 256
    } else if instruction == "." {
        stdout.write(text_from_unicode_scalar(memory(data_pointer)))
    } else if instruction == "," {
        memory(data_pointer) = stdin.read_byte()
    } else if instruction == "[" {
        if memory(data_pointer) == 0 {
            seek_closing_bracket()
        }
    } else if instruction == "]" {
        if memory(data_pointer) != 0 {
            seek_opening_bracket()
        }
    }

    instruction_pointer += 1
}
```
