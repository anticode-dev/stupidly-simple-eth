# Stupidly Simple Eth Client

Presenting a stupidly simple eth client.

You know, a client that is not pretending to be the most feature complete or the fastest one, but ideally the one that is so easy to to use it makes prototyping extremely fast (and fun).

This is more just a jsonrpsee wrapper with some eth imports, but I've been using this and the ease of use is quite nice, compared to the usual bulky clients with many methods. My favourite workflow: use this to pull code and storage into revm, then use rustry to test on top of it.

Strong believer that ease of use beats having tons of features more often than not. This crate should serve as a coutnerpoint to more and more complicated blockchain crates. 

For how to use this crate and how to add more methods, feel free to look into client.rs.

If you want to add more methods, push a PR or fork this ^^