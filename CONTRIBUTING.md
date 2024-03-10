<h1>Contributing</h1>

This guide is for the people who are interested in contributing to consumet.rs. It is not a complete guide yet, but it should help you get started. If you have any questions or any suggestions, please open a [issue](https://github.com/consumet-rs/consumet.rs/issues/new?assignees=&labels=Bug&template=bug-report.yml) or join the [discord server](https://discord.gg/qTPfvMxzNH).

See our [informal contributing guide](./docs/guides/contributing.md) for more details on contributing to this project.

<h2>Table of Contents</h2>

- [Prerequisites](#prerequisites)
  - [Cloning the repository](#cloning-the-repository)
  - [Project structure](#project-structure)
- [Writing a provider](#writing-a-provider)
    - [Setting up the provider](#setting-up-the-provider)
- [Updaing codebase](#updaing-codebase)
  - [Updating documentation](#updating-documentation)
  - [Fixing a provider](#fixing-a-provider)
- [Commit message](#commit-message)


## Prerequisites
To contribute to Consumet code, you need to know the following:
   - [Cargo](https://www.rust-lang.org/learn/get-started)
   - [Rust](https://www.rust-lang.org/)
   - Web scraping
       - [Reqwest](https://crates.io/crates/reqwest/)
       - [Css Selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors)
       - [DevTools](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/What_are_browser_developer_tools)

### Cloning the repository
1. [Fork the repository](https://github.com/consumet-rs/consumet.rs/fork)
2. Clone your fork to your local machine using the following command **(make sure to change `<your_username>` to your GitHub username)**:
```sh
git clone https://github.com/<your-username>/consumet-api.git
```
3. Create a new branch:
```sh
git checkout -b <new-branch-name>
```

### Project structure
I believe that project structure is needed to make it simple to contribute to consumet.ts.

***\<category>*** is the category of the provider. For example, `anime` or `book`, `etc`.\
***\<provider-name>*** is the name of the provider. For example, `libgen` or `gogoanime`, `etc`. (must be in camel case)

```sh
> tree
docs/
├── guides/
|   ├── ...
|   ├── anime.md
|   ├── getting-started.md
│   └── contributing.md (informal guide)
├── providers/
│   └── <provider-name>.md (provider documentation)
├── README.md
src/
├── lib.rs
|── models
├── providers
│   ├── <category>
│   │   ├── mod.rs
│   │   └── <provider-name>.rs
│   └── <category>
└── utils
```

#### Setting up the provider
1. Create a new file in the `src/providers/<category>/<provider-name>.rs` folder.
2. Start writing your provider code.
3. Add the provider to the `src/providers/<category>/mod.rs` file.


## Updaing codebase
### Updating documentation
1. Update the documentation.
2. [Commit the changes](#commit-message).

### Fixing a provider
1. Update the provider code.
2. [Commit the changes](#commit-message).

## Commit message
When you've made changes to one or more files, you have to *commit* that file. You also need a
*message* for that *commit*.

You should read [these](https://www.freecodecamp.org/news/writing-good-commit-messages-a-practical-guide/) guidelines, or that summarized:

- Short and detailed
- Prefix one of these commit types:
   - `feat:` A feature, possibly improving something already existing
   - `fix:` A fix, for example of a bug
   - `refactor:` Refactoring a specific section of the codebase
   - `test:` Everything related to testing
   - `docs:` Everything related to documentation
   - `chore:` Code maintenance

Examples:
 - `feat: Speed up parsing with new technique`
 - `fix: Fix 9anime search`
 - `refactor: Reformat code at 9anime.rs`
