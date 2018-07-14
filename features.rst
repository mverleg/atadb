
Features
===============================

Data types
-------------------------------

Planned
...............................

* Null (of all types)
* Boolean
* Int (32 and 64 bit)
* Float (64 bit)
* Decimal (shifted 64 bit int)
* Datetimes (64 bit timestamp)
* String (utf8)
* Binary

Implemented
...............................

Nothing yet!

Will not support (yet)
...............................

* Timedeltas (i.e. arithmetic on dates)
* Dates
* JSON
* XML
* Anything else

Select
-------------------------------

Planned v1
...............................

* Basic select-from
* Where on index (=)
* And, or, not
* As-alias
* Inner join on index
* Left outer join (and maybe right) on index
* Full outer join on index
* Limit
* Prepared statements (only)

Planned v2
...............................

* Arithmetic for numeric types
* Where without index (=)
* Where with <, <=, >, >=, not
* Aggregation (sum, avg, count)
* Group by
* Having
* In (with index)
* Like

Implemented
...............................

Nothing yet!

Will not support (yet)
...............................

* Distinct
* Order by (do that client-side)
* Between
* Exists, any, all (unsure)
* Match
* Union

Insert, update, delete
-------------------------------

Planned
...............................

* Insert literal
* Insert from query
* Delete
* Update
* Auto-increment
* Created datetime
* Last changed datetime
* Prepared statements (only)

Implemented
...............................

Nothing yet!

Will not support (yet)
...............................

* On-delete options

DDL (structure)
-------------------------------

Planned v1
...............................

* Create table
* Indices
* Foreign keys
* Ubiquity

Planned v2
...............................

* Alter
* Drop
* Row-based check

Implemented
...............................

Nothing yet!

Will not support (yet)
...............................

* Column defaults
* Non-row checks
* Most other stuff
* Views

Misc stuff
-------------------------------

Planned v1
...............................

* UTF-8 support
* Comments
* Control memory size

Planned v2
...............................

* Locks (full table)
* Permissions: read, write, ddl, grant
* Transactions (TCL)
* Auth/help server
* Binary protocol (?)
* Produce/use run stats
* String interning
* Dump/load binary backups
* Reindex, reorder commands?

Implemented
...............................

Nothing yet!

Will not support (yet)
...............................

* Multiple machines
* Any control flow (begin, return, break, throw, continue, goto, while...)
