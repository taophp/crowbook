Crowbook
========

[![Build Status](https://travis-ci.org/lise-henry/crowbook.svg?branch=master)](https://travis-ci.org/lise-henry/crowbook)

Render a markdown book in HTML, Epub or PDF.

Crowbook's purpose is to allow you to automatically generate multiple
outputs formats from a book written in Markdown. Its main focus is
novels, and the default settings should (hopefully) generate readable
books with correct typography.

Example
-------

To see what Crowbook's output looks like, you can read (a
not-necessarily up-to-date version of) the Crowbook
guide (containing this README.md file and additional documentation)
rendered in [HTML](http://lise-henry.github.io/crowbook/book.html),
[PDF](http://lise-henry.github.io/crowbook/book.pdf) or [EPUB](http://lise-henry.github.io/crowbook/book.epub).


Installing
----------

### Packages ###

If you are on Debian GNU/Linux or Ubuntu (on a PC architecture), you can
download `.deb` packages on
[the releases page](https://github.com/lise-henry/crowbook/releases). 

### Binaries ###

See [the releases page](https://github.com/lise-henry/crowbook/releases)
to download a precompiled binary for your architecture (currently:
Linux, Windows and MacOSX). Just extract the archive and run
`crowbook` (or `crowbook.exe` on Windows). You might also want to copy
the binary somewhere in your `PATH` for later usage.

### Building ###

You'll need to have the [Rust](https://www.rust-lang.org/) compiler
on your machine first; you can
[download and install it here](https://www.rust-lang.org/downloads.html). Once
it is down:

```
$ cargo install crowbook
```

will automatically download the latest `crowbook` release on
[crates.io](https://crates.io/crates/crowbook) and install it.

Usage
-----

The simplest command is:

```bash
$ crowbook <BOOK>
```

where `BOOK` is a configuration file. Crowbook will parse this
file and generate a book in HTML, Epub, LaTeX, and/or PDF,
according to the settings in the configuration file. So if you clone
this repository and run

```bash
$ crowbook config.book
```

you'll generate the example book in various formats. The
HTML version should look
[like that](http://lise-henry.github.io/crowbook/book.html).

To create a new book, assuming you have a
list of Markdown files, you can generate a template configuration file
with the `--create` argument:

```bash
$ crowbook --create my.book chapter_*.md
```

This will generate a default `my.book` file, which you'll need to
complete. This configuration file contains some metadata, options, and lists the
Markdown files. Here is a basic example:

```
author: Joan Doe
title: Some book
lang: en

output_html: some_book.html

+ chapter_1.md
+ chapter_2.md
+ chapter_3.md
+ ...
```

For more information see
[the configuration file](book_example/config.md).

It is also possible to give additional parameters to `crowbook`;
we have already seen `--create`, but if you want the full list, see
[the arguments](book_example/arguments.md).

Current features
----------------

### Output formats ###

Crowbook (to my knowledge) correctly supports HTML and EPUB (either
version 2 or 3) as output formats: rendered files should pass
respectively the [W3C validator](https://validator.w3.org/) and the
[IDPF EPUB validator](http://validator.idpf.org/) for a wide range of
(correctly Markdown formatted) input files. See the example book
rendered in [HTML](http://lise-henry.github.io/crowbook/book.html) and
[EPUB](http://lise-henry.github.io/crowbook/book.epub) on github.io.

LaTeX output is a bit more tricky: it should work reasonably well for
novels (the primary target of Crowbook), but `pdflatex` might occasionally
choke on some "weird" unicode character. Moreover, images are not yet
implemented (but should come soon). See the example book rendered in
[PDF](http://lise-henry.github.io/crowbook/book.pdf) on github.io.

ODT output is experimental at best. It might work if your inputs files
only include very basic formatting (basically, headers, emphasis and
bold), it will probably look ugly in the rest of the cases, and it
might miserably fail in some. See the example book rendered in
[ODT](http://lise-henry.github.io/crowbook/book.odt) on github.io if
you want to hurt your eyes.

### Input format ###

Crowbook uses
[pulldown-cmark](https://crates.io/crates/pulldown-cmark) and thus
should support most of CommonMark Markdown. Inline HTML, however, is
not implemented, and probably won't be, as the goal is to have books
that can also be generated in PDF (and maybe eventually ODT).

Maybe the most specific "feature" of Crowbook is that (by default, it
can be deactivated) it tries to "clean" the input files. By default this
doesn't do much (except removing superfluous spaces), but if the
book's language is set to french it tries to respect french
typography, replacing spaces with non-breaking ones when it is
appropriate (e.g. in french you are supposed to put a non-breaking
space before '?', '!', ';' or ':'). This feature is relatively limited
at the moment, but I might try to add more options and support for
more languages.

### Links handling ###

Crowbook tries to correctly translate local links in the input
Markdown files: e.g. if you have a link to a markdown file that is
part of your book, it will be transformed into a link inside the
document. 

### Inline YAML blocks ###

Crowbook supports inline YAML blocks. These are blocks
delimited with lines containing `---` (three dashes). An example of
such block:

```markdown
---
author: Me
title: My title
---
```

These blocks must contain *valid* YAML syntax. If they are not at the
beginning of the Markdown file, they must also be preceded by an empty
line.

It is possible to use inline YAML blocks to modify options for the
book (`author` and `title` in the previous example). This is mostly
useful when Crowbook is runned with the `--single` argument (receiving
a single Markdown file instead of a book configuration file).

### Bugs ###

See the [github's issue tracker](https://github.com/lise-henry/crowbook/issues).

Acknowledgements
----------------

Besides the [Rust](https://www.rust-lang.org/) compiler and standard library, Crowbook uses the
following libraries:

* [pulldown-cmark](https://crates.io/crates/pulldown-cmark) (for
parsing markdown)
* [yaml-rust](https://crates.io/crates/yaml-rust) (for parsing YAML blocks)
* [mustache](https://crates.io/crates/mustache) (for templating)
* [clap](https://github.com/kbknapp/clap-rs) (for parsing command line arguments)
* [chrono](https://crates.io/crates/chrono) (date and time library)
* [uuid](https://crates.io/crates/uuid) (to generate uuid)

It also uses configuration files from
[rust-everywhere](https://github.com/japaric/rust-everywhere) to use
[Travis](https://travis-ci.org/) and
[Appveyor](http://www.appveyor.com/) to generate binaries for
various platforms on each release.

While Crowbook directly doesn't use them, there was also inspiration
from [Pandoc](http://pandoc.org/) and
[mdBook](https://github.com/azerupi/mdBook).

Also, the [W3C HTML validator](https://validator.w3.org/) and the
[IDPF EPUB validator](http://validator.idpf.org/) proved very useful
during development.

ChangeLog
---------

See [ChangeLog](ChangeLog.md).

Library
-------

While the main purpose of Crowbook is to be runned as a command line,
the code is written as a library, so if you want to build on it you can
use it as such. You can look at the generated documentation [here](http://lise-henry.github.io/rust/crowbook/).

License 
-------

Crowbook is free software: you can redistribute it and/or modify it
under the terms of the GNU Lesser General Public License (LGPL),
version 2.1 or (at your option) any ulterior version. See 
[LICENSE](LICENSE.md) for more information.


