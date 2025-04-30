[0mAn import was unresolved.[0m

[0mErroneous code example:[0m

[0m[0m[2muse something::Foo; // error: unresolved import `something::Foo`.[0m

[0mIn Rust 2015, paths in [0m[0m[2muse[0m statements are relative to the crate root. To[0m [0mimport items relative to the current and parent modules,
use the [0m[0m[2mself::[0m and[0m [0m[0m[2msuper::[0m prefixes, respectively.[0m

[0mIn Rust 2018 or later, paths in [0m[0m[2muse[0m statements are relative to the current[0m [0mmodule unless they begin with the name of a crate or a
literal [0m[0m[2mcrate::[0m, in[0m [0mwhich case they start from the crate root. As in Rust 2015 code, the [0m[0m[2mself::[0m [0mand [0m[0m[2msuper::[0m prefixes refer to the
current and parent modules respectively.[0m

[0mAlso verify that you didn't misspell the import name and that the import exists[0m [0min the module from where you tried to import it.
Example:[0m

[0m[0m[2muse self::something::Foo; // Ok.

mod something {
    pub struct Foo;
}[0m

[0mIf you tried to use a module from an external crate and are using Rust 2015,[0m [0myou may have missed the [0m[0m[2mextern crate[0m declaration (which
is usually placed in[0m [0mthe crate root):[0m

[0m[0m[2mextern crate core; // Required to use the `core` crate in Rust 2015.

use core::any;[0m

[0mSince Rust 2018 the [0m[0m[2mextern crate[0m declaration is not required and[0m [0myou can instead just [0m[0m[2muse[0m it:[0m

[0m[0m[2muse core::any; // No extern crate required in Rust 2018.[0m[0m
