# Regex Replacer

This is simple package for fast replace texts with regex.
Support Rust regex syntax: https://docs.rs/regex/latest/regex/#syntax

## Installation
```bash
pip install regex_replace --upgrade
```

## Usage
```python
from regex_replacer import RegexReplacerTransformer

transformer = RegexReplacerTransformer([
    (r'[0-9]+', " NUMBER "),
    (r"[a-z]+", " WORD ")
])
test_strings = [
    "this is a SIMPLE TEXT 12345 with number"
]
print(transformer.transform(test_strings))

# Output: [' WORD   WORD   WORD  SIMPLE TEXT  NUMBER   WORD   WORD ']
```

for multithread transform:
```python
transformer = RegexReplacerTransformer([
        (r'[0-9]+', " NUMBER "),
        (r"[a-z]+", " WORD ")
    ], n_jobs=3)
```

to back to single thread:
```python
transformer.to_single_thread()
```