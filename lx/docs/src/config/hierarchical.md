Whether taxonomy terms can be nested.

For example, categories often include top-level "parent" items and
children of those items, e.g. "Tech" and "Tech -> programming",
where "programming" is a child of "Tech". If the flag is set to
`true`, the user can do this in their YAML header for each item
(assuming a taxonomy named `categories`):

```yaml
categories:
  - tech:
    - programming languages
    - agile software development
```

The resulting item would belong to the top-level "tech" category and
also to each of the nested "programming languages" and "agile
software development" categories. (The mechanics for managing nested
taxonomy terms are described elsewhere.)

If this value is set to `false`, the site will report an error for
the
