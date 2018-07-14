
atadb
===============================

``atadb`` will be a simple relational database focused on speed, best practises and simplicity made in Rust.

It supports only a subset of SQL. Most notably, it will only allow joins that have indices in both tables (with equality constraints). This forces one to write performant queries. It also only supports prepared statements, to enforce best practises.

It is mostly being developed as a learning project. If you are learning about Rust and/or databases and would like to join, create an issue!

