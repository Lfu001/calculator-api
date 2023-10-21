# Calculator API [![CI](https://github.com/Lfu001/calculator-api/actions/workflows/rust.yml/badge.svg)](https://github.com/Lfu001/calculator-api/actions/workflows/rust.yml)

## Usage

| Operator | API endpoint |
| -------- | ------------ |
| Add      | /add         |
| Subtract | /subtract    |
| Multiply | /multiply    |
| Divide   | /divide      |

### Parameters

- `a`: `f64` Left operand
- `b`: `f64` Right operand

### Returns

JSON object.

```rust
{
    result: Option<f64>,
    operand: Operator,
    a: f64,
    b: f64,
}
```

## Examples

### Addition

request:

```text
/add?a=1.5&b=2.3
```

response:

```json
{
    "result": 3.8,
    "operator": "ADD",
    "a": 1.5,
    "b": 2.3
}
```

### Division

request-1:

```text
/divide?a=4.5&b=1.25
```

response-1:

```json
{
    "result" :3.6,
    "operator": "DIVIDE",
    "a": 4.5,
    "b": 1.25
}
```

request-2:

```text
/divide?a=7.3&b=0
```

response-2:

```json
{
    "result": null,
    "operator": "DIVIDE",
    "a": 7.3,
    "b": 0.0
}
```

## Licence

[UNLICENSE](UNLICENSE)
