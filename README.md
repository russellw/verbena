# verbena
Server-side web framework. Work in progress.

It has been said that the two hard problems in computer science are cache invalidation, naming things, and off-by-one errors. In application programming, on the other hand, the hardest problem must surely be [the object-relational impedance mismatch](https://blog.codinghorror.com/object-relational-mapping-is-the-vietnam-of-computer-science/). The most common solution today is to use an ORM (Object Relational Mapper), the subject of a perennial argument:

- ORMs are a headache. On the object side, they only *almost* give you the semantics you want. On the relational side, they tend to generate [severely suboptimal queries](https://stackoverflow.com/questions/97197/what-is-the-n1-selects-problem-in-orm-object-relational-mapping) that bog down the performance of the whole system.

- Yes, but writing SQL as concatenated strings within a general programming language is awkward enough that no one does it for long (not to mention a security hazard if you fall into the trap of concatenating parameters to the query string instead of using the proper parameterized query facility), so eschewing an off-the-shelf ORM just means you end up with an ad-hoc homebrew one.

This argument is perennial because *both sides are right*.

The problem is inherent in the mismatch between the object semantics with which data is handled within general programming languages, and the relational semantics with which it is handled within databases. Can we solve this by eliminating one side of the mismatch? Completely abolishing objects does not seem practical. Starting in the 1980s, attempts were made to do the reverse, using object database management systems, but these proved unsatisfactory in the general case, and most business data today still lives in relational databases.

The Verbena thesis:

1. Internal software machinery may well prefer objects, but application data belongs in relational tables. Hash tables, file handles and windows are good candidates for objects, but customers, products and invoices are not.

2. SQL is a good enough language for handling relational data. It has hitherto been found inadequate, not so much because of the (easily attributable) imperfections in SQL itself, as because of the (pervasive) clumsiness of writing it as concatenated strings within a general programming language.

3. The challenge should therefore be reframed as that of more ergonomically integrating SQL with a general programming language.

Icon by [icon king1](https://freeicons.io/profile/3) on [freeicons.io](https://freeicons.io)
