LARVAE
======

if you wanna know how to use the program, just check the __usage__.

LARVAE lang.
------------

it's real simple don't worry.

tags work like this: `[TAG;ARGS:CONTENTS]`.

e.g. `[HEADING;2:LARVAE]`, which converts to `<h2>LARVAE</h2>`

tags can be expanded across multiple lines like this:

```
[HEADING:
LARVAE
]
```

and nested like this (all whitespace is pretty much ignored):

```
[HEADING:
LARVAE
[SUBTITLE:
A markup language.
]]
```

so basically that's it. now i'm gonna run down all the different tags and their (optional) args.

### TAGS
__HEADING__ [SIZE]

    a heading tag with the optional argument of a size, which is a number.

__SUBTITLE__
    
    a subtitle, intended to be nested inside of HEADING tags.

__LINK__ {URL}
    
    a link.

__E__ {B,I,U,S}
    
    the emphasis tag, the options are {bold, italic, underlined, secondary color}
    and can be used simultaneously.

__ALIGN__ {LEFT,CENTER,RIGHT}
    
    an alignment of either left, center or right.

### SPECIAL CHARACTERS

__~__
    
    a new line.

__ \` __
    
    an indent.
