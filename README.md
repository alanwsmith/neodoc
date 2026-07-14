-- title

Neopolitan

A parser for the neopolitan format. 

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

