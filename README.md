# MuTNT

In many of today's applications there are multiple systems that need to
interact with some sort of state. Our go-to tool for this kind of problem
are databases. Some of them provide very interesting guarantees (ACID), while
also allowing the developers to register some interest to some events (triggers).

Sometimes we do not want to have to maintain a full database for our application, or
just need some extra features/invariants that a database does not bring out of the box.
That's what `MuTNT` is: a toolbox of shareable data-structures that can be queried,
and updated from multiple clients. It also allows to register certain actions, based
on changes on the state.

## Goals
- Deterministic and probabilist data structures toolbox
- Authorization and authentication mechanisms out of the box
- Middleware/Plugin system to hook to triggers (slack notifications, updates to other data structures, etc.)

## Non goals
- Ultra low latency