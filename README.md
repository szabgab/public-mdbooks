# Public mdbooks

That is, mdbooks where the source code is available.

## Collect information about the books

e.g. which configuration options do they use with which values, which plugins they use etc.


## Data collected from:

* [Awesome mdbook](https://github.com/softprops/awesome-mdbook)
* [mdbooks](https://github.com/search?o=desc&q=mdbook&s=stars&type=Repositories)
* [mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers)
* [The Rust Book translations](https://doc.rust-lang.org/stable/book/appendix-06-translation.html)


## CONTRIBUTION

The `mdbooks.yaml` file contains the list of mdBooks. If you'd like to add another one, just add it to the end of the list.

* The `title` can be in the native language of the book.
* The `description` should be in English and it should mention the native language.
* The `repo` is the `https` URL of the repository.
* The `folder` is an optional field indicating in which subfolder is the `book.toml` file. If it is in the root of the repo then no need for this field.
* The `site` is the `https` URL to the generated book, if available.
* TODO: We will need to add a `branch` field to support projects where the book is not in the default branch. For now just add it as a comment. See the one we already have in the `mdbooks.yaml` file.

## RUN locally

In one terminal run:

```
mdbook serve report
```


Clone all the repositories to the `repos` folder. (You can also create a symbolic link called `repos` to point to some other disk.)

```
cargo run -- --clone
```

```
cargo run -- --report && mdbook build report
```

Because we generate all the `md` files in our own book they are listed in the `.gitignore` and that breaks the `watch` of mdbooks.
So is won't rebuild automatically when we re-generate the report. That's why we need to run `mdbook build` every time we generate
the report.

## TODO

Some ideas for future development

* Deal with non-default branches.
* Deal with all the plugins that are not public.
* Collect all the parameters and their values of all the plugins.
* Shall we run the most recent version of `mdbook` on these books to see if it works? We need to be careful running arbitrary code in the plugins!
