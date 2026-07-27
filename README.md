-- title

Neopolitan

A parser for the neopolitan format. 


-- h2

Possible Domains

-- list

- neodoc.style (feels more like supporting
the format without the goal of building
a service which I think is the right
direction)

- neodoc.garden

- neodoc.site

- neodoc.space

- neodoc.network

- neodoc.nexus

- neodoc.ing

- neodoc.studio





-- h2

Thinking about NeoProse

I'm Thinking about making an
AT Proto lexicon for pages using
Neopolitan. That name is pretty
covered in terms of available domains
so I'm thinking about a rename to
`neoprose` (or neodoc, or something). 
These are some notes about that. 

Here's some

-- ideas checklist/ 
-- h3: For the lexicon

[] Maybe do do a ``status`` but with
only two values "draft" and "done"?
or is it better to do that with tags?
status is more explicit, but again,
probably not since there's no way
to control what apps will do with 
it (or if they ignore it)


[] Do add a ``flags`` key as 
a known thing for processing. It's
a way to pass data to apps. 
The specific handling of any given
flag value is up to the app 
itself. 

[] Custom/Arbitrary key/value pairs
can be used in the ``-- metadata``
block beyond the ones specifically
defined. Handling them is 
the responsibility of each app and
there's no guarantee that apps
will have the same behavior.




[] The rust parser is available for
use of parsing. It's the reference that
other parsers can be built off of. 

[] The lexicon provides these for the
documents:

    -- list/

    - The content in Neopolitan format

    - Metadata about the file

    - Optional CSS styles to apply to the
        document. These styles are 
        always added after an apps 
        default styles and the 
        publication specific styles 
        (i.e. the ones that are applied
        to all posts from a given account)

    - Optinal templates to supplement
        or override the default 
        templates. 
    
    - No JavaScript

    -- /list

[] iFrames can be used to include other content.
This is up to the display app to 
determine if they're allowed. 

[] There's a default set of neopolitan sections
and templates defined as part of the lexicon. 

[] The ``-- metadata`` section is special. It's never
rendered directly on the page. It's content is used
for preview text in open graph style handling. 

TODO: Update the rest of these notes to 
convert from the blurb to the metadata. 

It's not rendered by default. It's
only used to preview the content 
when it's linked to from another page. 

The blurb section is processed as a
raw section. The output is converted
to a single HTML escaped string (so, 
text spacing and new lines don't
matter)

The metadata can be multiple paragraphs. 

TODO: Set a size limit expectation for 
the blurb. (This won't be enforced
in the content itself, but the
blurb is included in the lexicon
and when it's pulled from the 
content it's truncated if necessary. 


It's only used to store info
about the page. Apps aren't expected to use
this data. It's more for the tools that publish
the AT Proto entries to generate a valid 
entry (e.g. ``created`` from the ``-- metadata``
section gets used as the created datetime in
the JSON object of the entry. 

[] The parser always return a specific AST
for a given input, but the AST is not stored. 
Only the original text is. It's the
responsibility of the app to parse the document. 

[] Optional templates are in the MiniJinja
format with the `{{ }}` replaced with
`[@ @]` (TODO: Add the other two token
replacements here).

[] There are two sized thumbnails 
for preview content: large and small, 
The large is the current standard:
1200x600. The small one is a 1:1 ratio
with a recommended min size of 600px. 

[] Each publication has a root URL
for images. 

[] Image calls in posts can 
either be full link (e.g. https://...
or relative to a doc root (in
which case the URL from the
publication is used for the base). 

Relative paths aren't permitted. 
(again, someone can put them in
but if they do it's up to the 
parser to ignore them or figure
out what to do with them, but
they should be considered out
of spec)

[] There are no comment type
sections in the prose. The 
expectation is that everything 
in the document is visible  

[] There are two image formats
for open graph style images. 
This first is the same as
the current de facto standard
(which I think is 1200x600)
The second is a square size.
TODO: figure out the expected
display size. Something smaller
than the full unfurl of
the larger images. The idea
being the square image is
for going with the text on 
smaller cards. 


[] The publication level
has avatar images. There
are two one square and one
circular. They are both 1:1 ratio
images with the expectation
that the circular one will
be cropped into a circle. 

[] Template fallback is:

    -- list/

    - Start with the specific section 
        name template in the post
        itself. 
    - Fall back to the specifi section
        name from the publication. 
    - Fall back to the specific 
        section name from the app.
    - Fall back to the generic
        section type from the post
    - Fall back to the generic
        section type from the publication
    - Fall back to the generic
        section type from the app

    -- /list


[] Define how ``tags`` should be processed
in the metadata. Specifically, they
should be split on commas. If a
comma needs to be included in a tag
it can be escaped with a backslash. 
If a backslash needs to be escaped
another backslash can be used. Those
are the only two escape characters. 

[] All neodocs are UTF-8. 

[] Define standard metadata keys:

    -- list/

    - title: STRING

    - created: RFC3339

    - completed: RFC3339

    - updated: RFC3339
    
    - PROBABLY NOT: status: ENUM (TBD on strings, but case insentive)

    -- /list

[] Should there be a status? Maybe not. 
The idea being that if it's out there, it's
out there and there's no way to control
apps to do things like only show "done"
and not "draft" posts. Also, trying 
to spec the possible keys would be
an exercise in futility.

[] Define the cascade for looking 
for a title. Probably start with the
first block from  `-- title` if it
exists. Otherwise, use the ``title``
key from ``-- metadata``.

Actually, maybe there isn't a title
field in the metadata. If you want
a title, add a title section. That 
way the methodology is always the
same. 

[] Note that titles are not required. 

[] Set up the parser so the first 
section is automatically a paragraph
without requiring ``-- p`` to
start the string. The idea being you can
just type a paragraph by itself
and it's valid. 

[] Everything that can be optional
should be. Specifically, you should
be able to write a single string
of text with no sections at all
and it be valid. That probably means
auto generating some things
in the lexicon metadata. 

[] Try to require all fields in the 
JSON and provide defaults (e.g. ``null``)
if their values don't exist. Goal
being to not have to check for 
undefined for the shape of a JSON. 

[] Should there be versioning in the
format? Could be done and done in 
a way where users don't see it
if they don't need it. But, then
that could be surprising, so
probably not? And, adding versioning
would add complexity. 

[] Add a canonical URL for linking
out somewhere. 


-- /checklist



-- h2

Neopolitan notes

things on the list to do regardless
of the name change or AT Proto
lexicon. 


-- todo

[] Convert to using ssb for the time
being. Eventually the site will be
built with Neopoligen.

[] Define the default section types
that go into each section category. 

[] Accept a serialized JSON string
as a config. 

[] Accept a path to a config file.

[] Provide for a default config location. 

[] Don't have any expectation for the
sections (e.g. ``page`` or ``metadata``
sections are not required. Everything
just loads in a standard manner. It's
up to the external processes to 
determine what to do with the content) 

[] Provide for inline definition of 
section categories. (e.g. ``-- custom-section raw``)

[] Define the possible shortcodes and
what they represent. The specifics are
backed into the AST to ensure consistency. 


-- h2

Overview

Neopolitan is a plain-text format
in the spirit of markdown. It has the
following features:

-- list/

- Content is split into sections. 

- There are different types of sections:

    -- list/

    - basic 

    - checklist

    - description-list

    - json

    - list

    - numbered-list

    - raw


    -- /list

- Each section can have attributes and flags. 

- Sections can nest inside each other.  

- Content can include spans of various types.
They provide shorthands for links, formatting,
etc... 

- Spans can include attributes and flags. 

- Custom spans can be used for advanced formatting. 

-- /list

The parser does not output HTML like markdown 
processors. It returns an AST in JSON format that
other processed can use to generate their
desired output. 


-- h2

Default Section Categories

-- list

- checklist/cl - checklist, cl, runbook, todo, todos

- description-list/dl - description-list, dl

- generic - everything that's not part of another category 

- json - json, metadata

- list - list, notes

- numbered-list/nl/ol - nl, numbered-list, ol

- plugin - plugin

- raw - cli, code, comment, css, data, html, javascript, markdown, output, raw, result, results, path, pre, text


