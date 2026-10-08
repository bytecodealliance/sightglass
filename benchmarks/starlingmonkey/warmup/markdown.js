// marked builds its inline-lexer regexes lazily on first use. Calling `main`
// here, during pre-initialization, puts them in the Wizer snapshot instead of
// the measured region.
main("---\ntitle: t\n---\n# h\n\n## h2\n\n> q\n\n1. a *b* **c** _d_ `e` [f](http://g) <h> &amp;\n- i\n\n    code\n\n```\nfence\n```\n\ntext\nmore\n");
