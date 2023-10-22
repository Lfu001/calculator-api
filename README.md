# Calculator API [![CI](https://github.com/Lfu001/calculator-api/actions/workflows/rust.yml/badge.svg)](https://github.com/Lfu001/calculator-api/actions/workflows/rust.yml)

## Usage

| Operator | API endpoint    |
| -------- | --------------- |
| +        | /api/adder      |
| -        | /api/subtractor |
| *        | /api/multiplier |
| /        | /api/divider    |

### Parameters

- `a`: `f64` Left operand
- `b`: `f64` Right operand

### Returns

JSON object.

```rust
{
    result: Option<f64>,
    operand: Operator,
    a: Option<f64>,
    b: Option<f64>,
}
```

## Examples

### Addition

request:

```text
/api/adder?a=1.5&b=2.3
```

response:

```json
{
    "a": 1.5,
    "b": 2.3,
    "operator": "ADD",
    "result": 3.8
}
```

### Division

request-1:

```text
/api/divider?a=4.5&b=1.25
```

response-1:

```json
{
    "a": 4.5,
    "b": 1.25,
    "operator": "DIVIDE",
    "result" :3.6
}
```

request-2:

```text
/divide?a=7.3&b=0
```

response-2:

```json
{
    "a": 7.3,
    "b": 0.0,
    "operator": "DIVIDE",
    "result": null
}
```

## Licence

[UNLICENSE](UNLICENSE)
