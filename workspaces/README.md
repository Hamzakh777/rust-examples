## Cargo workspaces
A workspace is a set of packages that share the same `Cargo.toml` and output directory.
This is a workspace containing two libraries and one binary.
All the artifacts from the workspace packages go into one top level `target` directory.
By sharing one `target` directory, the crates can avoid unnecessary rebuilding.

Cargo doesn't assume the crates inside the workspace will depend on each other, 
so we need to be explicit about the dependency relationships.

We can specify which package in the workspace we want to run by using `-p`, for example
`cargo run -p adder`

Because we only have one `Cargo.lock`, this means all the packages will use the same version
of the dependencies. This means the crates will always be compatible with one another.

We can also run test for one particular package in the workspace by using `-p`, for example
`cargo test -p adder`.

If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately 