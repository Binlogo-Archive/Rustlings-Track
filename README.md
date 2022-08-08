# Rustlings-Track

This is my mrogress track of [rustlings](https://github.com/rust-lang/rustlings), 🦀 Small exercises to get you used to reading and writing Rust code!

📎 If you are intrested, welcome to try [it](https://github.com/rust-lang/rustlings).

![crab pet](https://i.imgur.com/LbZJgmm.gif)

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keeps them from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run myExercise1
```

Or simply use the following command to run the next unsolved exercise in the course:

```bash
rustlings run next
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

```bash
rustlings hint myExercise1
```

You can also get the hint for the next unsolved exercise with the following command:

```bash
rustlings hint next
```

To check your progress, you can run the following command:

```bash
rustlings list
```

## Testing yourself

After every couple of sections, there will be a quiz that'll test your knowledge on a bunch of sections at once. These quizzes are found in `exercises/quizN.rs`.

## Progress-Track

![Progress](https://img.shields.io/badge/Progress-100%25-brightgreen) 

```
Name             	Path                                          	Status
intro1           	exercises/intro/intro1.rs                     	Done
intro2           	exercises/intro/intro2.rs                     	Done
variables1       	exercises/variables/variables1.rs             	Done
variables2       	exercises/variables/variables2.rs             	Done
variables3       	exercises/variables/variables3.rs             	Done
variables4       	exercises/variables/variables4.rs             	Done
variables5       	exercises/variables/variables5.rs             	Done
variables6       	exercises/variables/variables6.rs             	Done
functions1       	exercises/functions/functions1.rs             	Done
functions2       	exercises/functions/functions2.rs             	Done
functions3       	exercises/functions/functions3.rs             	Done
functions4       	exercises/functions/functions4.rs             	Done
functions5       	exercises/functions/functions5.rs             	Done
if1              	exercises/if/if1.rs                           	Done
if2              	exercises/if/if2.rs                           	Done
quiz1            	exercises/quiz1.rs                            	Done
primitive_types1 	exercises/primitive_types/primitive_types1.rs 	Done
primitive_types2 	exercises/primitive_types/primitive_types2.rs 	Done
primitive_types3 	exercises/primitive_types/primitive_types3.rs 	Done
primitive_types4 	exercises/primitive_types/primitive_types4.rs 	Done
primitive_types5 	exercises/primitive_types/primitive_types5.rs 	Done
primitive_types6 	exercises/primitive_types/primitive_types6.rs 	Done
vecs1            	exercises/vecs/vecs1.rs                       	Done
vecs2            	exercises/vecs/vecs2.rs                       	Done
move_semantics1  	exercises/move_semantics/move_semantics1.rs   	Done
move_semantics2  	exercises/move_semantics/move_semantics2.rs   	Done
move_semantics3  	exercises/move_semantics/move_semantics3.rs   	Done
move_semantics4  	exercises/move_semantics/move_semantics4.rs   	Done
move_semantics5  	exercises/move_semantics/move_semantics5.rs   	Done
move_semantics6  	exercises/move_semantics/move_semantics6.rs   	Done
structs1         	exercises/structs/structs1.rs                 	Done
structs2         	exercises/structs/structs2.rs                 	Done
structs3         	exercises/structs/structs3.rs                 	Done
enums1           	exercises/enums/enums1.rs                     	Done
enums2           	exercises/enums/enums2.rs                     	Done
enums3           	exercises/enums/enums3.rs                     	Done
strings1         	exercises/strings/strings1.rs                 	Done
strings2         	exercises/strings/strings2.rs                 	Done
strings3         	exercises/strings/strings3.rs                 	Done
strings4         	exercises/strings/strings4.rs                 	Done
modules1         	exercises/modules/modules1.rs                 	Done
modules2         	exercises/modules/modules2.rs                 	Done
modules3         	exercises/modules/modules3.rs                 	Done
hashmaps1        	exercises/hashmaps/hashmaps1.rs               	Done
hashmaps2        	exercises/hashmaps/hashmaps2.rs               	Done
hashmaps3        	exercises/hashmaps/hashmaps3.rs               	Done
quiz2            	exercises/quiz2.rs                            	Done
options1         	exercises/options/options1.rs                 	Done
options2         	exercises/options/options2.rs                 	Done
options3         	exercises/options/options3.rs                 	Done
errors1          	exercises/error_handling/errors1.rs           	Done
errors2          	exercises/error_handling/errors2.rs           	Done
errors3          	exercises/error_handling/errors3.rs           	Done
errors4          	exercises/error_handling/errors4.rs           	Done
errors5          	exercises/error_handling/errors5.rs           	Done
errors6          	exercises/error_handling/errors6.rs           	Done
generics1        	exercises/generics/generics1.rs               	Done
generics2        	exercises/generics/generics2.rs               	Done
traits1          	exercises/traits/traits1.rs                   	Done
traits2          	exercises/traits/traits2.rs                   	Done
traits3          	exercises/traits/traits3.rs                   	Done
traits4          	exercises/traits/traits4.rs                   	Done
traits5          	exercises/traits/traits5.rs                   	Done
quiz3            	exercises/quiz3.rs                            	Done
tests1           	exercises/tests/tests1.rs                     	Done
tests2           	exercises/tests/tests2.rs                     	Done
tests3           	exercises/tests/tests3.rs                     	Done
lifetimes1       	exercises/lifetimes/lifetimes1.rs             	Done
lifetimes2       	exercises/lifetimes/lifetimes2.rs             	Done
lifetimes3       	exercises/lifetimes/lifetimes3.rs             	Done
iterators1       	exercises/standard_library_types/iterators1.rs	Done
iterators2       	exercises/standard_library_types/iterators2.rs	Done
iterators3       	exercises/standard_library_types/iterators3.rs	Done
iterators4       	exercises/standard_library_types/iterators4.rs	Done
iterators5       	exercises/standard_library_types/iterators5.rs	Done
box1             	exercises/standard_library_types/box1.rs      	Done
arc1             	exercises/standard_library_types/arc1.rs      	Done
threads1         	exercises/threads/threads1.rs                 	Done
threads2         	exercises/threads/threads2.rs                 	Done
threads3         	exercises/threads/threads3.rs                 	Done
macros1          	exercises/macros/macros1.rs                   	Done
macros2          	exercises/macros/macros2.rs                   	Done
macros3          	exercises/macros/macros3.rs                   	Done
macros4          	exercises/macros/macros4.rs                   	Done
clippy1          	exercises/clippy/clippy1.rs                   	Done
clippy2          	exercises/clippy/clippy2.rs                   	Done
clippy3          	exercises/clippy/clippy3.rs                   	Done
using_as         	exercises/conversions/using_as.rs             	Done
from_into        	exercises/conversions/from_into.rs            	Done
from_str         	exercises/conversions/from_str.rs             	Done
try_from_into    	exercises/conversions/try_from_into.rs        	Done
as_ref_mut       	exercises/conversions/as_ref_mut.rs           	Done
Progress: You completed 92 / 92 exercises (100.00 %).

🎉 All exercises completed! 🎉

+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                          \\/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒

We hope you enjoyed learning about the various aspects of Rust!
If you noticed any issues, please don't hesitate to report them to our repo.
You can also contribute your own exercises to help the greater community!

Before reporting an issue or contributing, please read our guidelines:
https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md
```

## Thanks ✨

All exercises source from [rustlings](https://github.com/rust-lang/rustlings)