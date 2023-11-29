# Documentation

This folder contains the documentation that's published to [pavex.dev](https://pavex.dev).  
The auto-generated documentation for the Rust crates serves as an API reference; this documentation, instead,
serves as a higher-level guide to the project: quickstart, tutorials, recipes, concept deep-dives, etc.

It's built using [MkDocs](https://www.mkdocs.org/) and [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/).

## Prerequisites

We don't want to force you to set up a Python dev environment: we rely on Docker to build and preview the docs.
Make sure you have [Docker](https://www.docker.com/) installed and running. 

You'll then need to pull the relevant image:

```bash
docker pull squidfunk/mkdocs-material:9
```

## Commands

You can preview the docs locally by running:

```bash
docker run --rm -it -p 8000:8000 -v ${PWD}:/docs squidfunk/mkdocs-material:9
```

The docs will be available at [http://localhost:8000](http://localhost:8000) and will auto-reload when you make changes.



