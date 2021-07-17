# MuTNT query language

## Introduction

Each datastructure provides its own functionality, and hence **might** have
a different interface. But beyond specifics, the whole point of the query
language is to enable a user to:
- create a datastructure
- populate it with data
- query the data
- remove data

(note: in the future, updating data, for some datastructures might also be possible)

## Creating a datastructure

```rust
unimplemented!()
```

## Populating a datastructure
```rust
unimplemented!()
```

## Querying a datastructure
There are different typs of queries. One can query a single result or a range of results.

**Single result**

Key-value style

```
GET FROM <datastructure name> WHERE KEY=<key_name>
```

Indexable style

```
GET FROM <datastructure name> FIRST
```

```
GET FROM <datastructure name> LAST
```

```
GET FROM <datastructure name> WHERE INDEX=<usize>
```
