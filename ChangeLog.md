ChangeLog
=========

0.5.0 (unreleased)
------------------
* Crowbook now requires Rustc 1.7.0.
* It is now possible to render HTML in multiple files:
     * `output.html_dir` will activate this renderer, and specify in
       which directory to render these files;
     * `html_dir.css` allows to override the CSS for this rendering;
     * `html_dir.index.html` allows to specify a template for the
       `index.html` page;
     * `html_dir.chapter.html` allows to specify a template for the
       chapters pages.
* New book options:
    * `tex.short`: if set to true, the LaTeX renderer will use
      `article` instead of `book` as document class, and will use the
      default `\maketitle` command for article. This option is by
      default set to false, except when Crowbook is called with
      `--single`.
    * `enable_yaml_blocks`: parsing YAML blocks is no longer activated
      by default, except when using `--single`. This is because you
      might want to have e.g. multiple short stories using YAML blocks
      to set their titles and so on, *and* a separate `.book` file to
      render a book as a collection of short stories. In this case,
      you wouldn't want the displayed title or the
      output.pdf/html/epub files be redefined by the short stories .md
      files.
    * `html.script`: allows to specify a custom javascript file.
    * `resources.base_path`: by default, Crowbook resolves local links in
      markdown files relatively to the markdown file. This option
      allows to resolve them relatively to a base path. This option
      comes with two variants, `resources.base_path.images` and
      `resources.base_path.links`, which only activate it for respectively
      images tags and links tags. These two options are ignored when
      `base_path` is set. There is also `resources.base_path.files`
      which specify where additional files (see below) should be read,
      but this is one is set to `.` (i.e.,  the directory where the
      `.book` file is) by default.
    * `resources.files`: indicate a (whitespace-separated) list of
      files that should be embedded. Currently only used with the EPUB
      renderer.
    * `resources.out_path`: indicate where `resources.files` should be
      copied in the final document. Default to `data`, meaning that
      files will be placed in a `data` directory in the EPUB.
* Rendering:
    * Standalone HTML now includes locale files using base64.
    * HTML/EPUB: default CSS now uses the `lang` value do determine
      how to display lists (currently the only difference is it uses
      "–" when `lang` is set to "fr" and standard bullets for other
      languages).
* Bugfixes:
    * Fixed a bug of filename "resolution" when Crowbook was called
      with `--single` (e.g., `crowbook -s tests/test.md` would
      previously try to load `tests/tests/test.md).
    * Epub renderer now uses the `mime_guess` library to guess the
      mime type based on extension, which should fix the mime type
      guessed for a wide range of extensions (e.g., svg).
* Internal/API:
    * The `Book::new`, `new_from_file`, and `new_from_markdown_file`
      take an additional `options` parameter. To create a book with
      default options, set it to `&[]`.



0.4.0 (2016-03-01)
------------------
* Crowbook now internally uses a true YAML parser, `yaml_rust`, for its
  options. Since the "old" Crowbooks's config format was similar, but
  had some subtle differences, this is somewhat of a breaking change:
    * strings should now be escaped with "" in some cases (e.g. if it
      contains special characters). On the other hand, it *allows* to
      optionally escape a string with these quotes, which wasn't
      possible until then and might be useful in some cases.
    * multiline strings now follow the YAML format, instead of the
      previous "YAML-ish" format. This can impact the way newlines are
      added at the end of a multiline string. See
      e.g. [this link](http://stackoverflow.com/questions/3790454/in-yaml-how-do-i-break-a-string-over-multiple-lines)
      for the various ways to include mulitiline strings in Yaml.
* Crowbook now parses YAML blocks (delimited by two lines with "---")
  in Markdown files, ignoring keys that it doesn't recognize. This
  allows crowbook to be compatible(-ish) with Markdown that contains
  YAML blocks for Jekyll or Pandoc.
* New option `--single` allows to give Crowbook a single Markdown file
  (which can contain options within an inline YAML block) instead of a
  book configuration file. This is useful for e.g. short stories.
* Enhanced the way debugging/warning/info messages are handled and
displayed:
    * Added a `--debug` option to the binary.
    * Internal: added a `Logger` struct.
    * Different levels of information (debug/warning/info/error) get
      different colours.
* Bugfixes:
    * Crowbook no longer crashes when called with the `--to` argument
      if it can't create a file. 


0.3.0 (2016-02-27)
------------------
* Crowbook now tries to convert local links. That is, if you link to a
  Markdown file that is used in the book.
  (e.g. [README.md](README.md)), it *should* link to an appropriate
  inner reference inside the book.
* Latex renderer now supports (local) images.
* Epub renderer now embed (local) images in the EPUB file.
* Some changes to the HTML/Epub stylesheets.
* Internal (or usage as a library):
    * Crowbook no longer changes current directory, which worked in
      the binary but could cause problem if library was used in
      multithreaded environment (e.g. in `cargo test`).
    * More modules and methods are now private.
    * Improved documentation.
    * Added more unit tests.
* Bugfixes: 
    * Epub renderer now correctly renders unnumbered chapter without a
      number in its toc.ncx file



0.2.2 (2016-02-25)
------------------
* Bugfixes:
    * French cleaner now correctly replaces space after — (in
      e.g. dialogs) with "em space".


0.2.1 (2016-02-25)
------------------
* Bugfixes:
    * HTML/Epub rendering no longer incorrectly increment chapter
      count for unnumbered chapters.
    * Latex: makes what is possible to avoid orverflowing the page.
* Minor changes:
    * Latex: improvement of the default way URLs are displayed.
      

0.2.0 (2016-02-25) 
------------------
* Command line arguments:
    * New argument `--print-template` now allows to print a built-in
      template to stdout. 
    * New argument `--list-options` prints out all valid
      options in a config file (or in `set`), their type and default
      value.
    * New argument `--set` allows to define or override whatever
      option set in a book configuration.
    * `--create` can now be used without specifying a `BOOK`, printing
      its result on `stdout`.
* Configuration file:
    * Added support for multiline strings in `.book` files, with
      either '|' (preserving line returns) or '>' (transforming line
      returns in spaces) 
    * New option `display_toc` allows to display the table of contents
      (whose name, at least for HTML, is specified by `toc_name`) in
      HTML and PDF documents.
    * Option `numbering` now takes an int instead of a boolean,
      allowing to specify the maximum level to number (e.g. `1`:
      chapters only, `2`: chapters and sectino, ..., `6`: everything).
* Rendering: 
    * Added support for numbering all headers, not just level-1 (e.g.,
      having a subsection numbered `2.3.1`).
    * Tables and Footnotes are now implemented for HTML/Epub and LaTeX
    output.
* Internal:
    * Refactored `Book` to use an HashMap of `BookOption`s instead of
      having like 42 fields.


0.1.0 (2016-02-21)
------------------
* initial release
