# jam

A tasty utility to find your tasty methods, just like Mom used to make

![jam-jars](https://github.com/JAS-lzn/jam/assets/62945622/3b3b5db2-101a-4cc5-9685-af064281aa04)

The goal of this utility is to help developers auto-generate documentation for
thier code, spesifically, document when different methods were added to a code
base, the properties of that method, and also document any changes to those
methods.

This is accomplished by cloning the repo, and reading all the source code. `Jam`
will read thru the source code and look for the tell-tale signs of methods (fn,
function, ()=>{}, etc) and extract the method's name, arguments, return type,
etc from that. `Jam` will then note what commit the method was added in, and
generate some `markdown` to later be injected into a `readme` or docs site.
`Jam` will then check the diffs between commits on the main branch of the repo,
looking for new methods, and changes to the already existing ones.
