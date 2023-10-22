# Calculator API [![CI](https://github.com/Lfu001/calculator-api/actions/workflows/rust.yml/badge.svg)](https://github.com/Lfu001/calculator-api/actions/workflows/rust.yml)

1000 digit precision calculator API.

## Usage

| Operator | API endpoint    |
| -------- | --------------- |
| +        | /api/adder      |
| -        | /api/subtractor |
| *        | /api/multiplier |
| /        | /api/divider    |

### Parameters

- `a`: Left operand
- `b`: Right operand

### Returns

JSON object.

```ts
{
    result: Number?,
    operand: Operator,
    a: Number?,
    b: Number?,
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
    "result": 3.6
}
```

request-2:

```text
/api/divider?a=7.3&b=0
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
