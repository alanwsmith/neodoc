-- title

neodoc

(formerly neopolitan)

A parser for the neodoc format. 



-- h2

Old Notes/Overview

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


