# Bread'n'Butter

<img align="right" alt="Logo" width="300" height="300" src="./.github/logo.png">

**A Buttery smooth [Breadboarding][] experience for software development.**

[breadboarding]: https://basecamp.com/shapeup/1.3-chapter-04

---

Bread'n'Butter is an open-source project aimed at simplifying and streamlining
the process of software design using a technique known as **breadboarding**.

Inspired by the concepts presented in Basecamp's "Shape Up," this project
provides tools to create, visualize, and share breadboards in a software
development context. Our goal is to enhance collaboration and clarity in the
early stages of software design, where functional relationships and user
interactions are key focal points.

At its core, Bread'n'Butter offers a Domain Specific Language (DSL) for
defining breadboards, a parser to translate this DSL into various formats, and
an interactive GUI for a visual exploration of designs.

Whether you're a developer, a designer, or someone interested in software
project planning, Bread'n'Butter is designed to be accessible, user-friendly,
and a valuable addition to your toolkit.

## Libraries

- [`bnb-parser`](./crates/parser) — Parses a breadboard string into an AST.

## Example

### Understanding the DSL Syntax

Bread'n'Butter's Domain Specific Language (DSL) allows you to define the
layout and interactions of a software project in a simple, readable format.
The DSL is designed to represent the screens (or "places") of your
application, the affordances within them, and the relationships between them.

Here's a basic example to illustrate the syntax and concepts:

```bnb
place Registration
    include Header

    Username
    Password
    Sign Up -> (success) Home
            -> (failure) Support

place Support
    include Header

    Error Message
    Try Again -> Registration

place Home
    include Header

    Dashboard

component Header
    Logo
    Contact
```

### Breaking Down the Example

1. **Defining Places:**
   - Each `place` represents a distinct section or view in your software, like
     a page or a screen.
   - In this example, there are three places: `Registration`, `Support`, and
     `Home`.

2. **Including Components:**
   - `include` allows you to insert predefined components into places.
   - `Header` is a component included in each place, symbolizing a shared UI
     element across different screens.

3. **Affordances within Places:**
   - Affordances like `Username`, `Password`, and `Dashboard` represent
     individual elements or features within each place.

4. **Defining Navigation and Actions:**
   - The `->` symbol is used to define actions or navigation paths.
   - In `Registration`, "Sign Up" can lead to either `Home` (on success) or
     `Support` (on failure).

5. **Creating Components:**
   - `component` defines reusable elements that can be included in multiple
     places.
   - `Header` component includes `Logo` and `Contact`, indicating these
     affordances are part of the header.

### Use-Case of the DSL

The DSL allows you to map out the structure and navigation flow of a software
project visually and logically. It is particularly useful in the early stages
of design, where understanding the interaction between different parts of the
application is crucial.

Using Bread'n'Butter's DSL, teams can:

- Clearly define and share the layout and user flow of a project.
- Quickly iterate on the structure and design of software.
- Easily communicate ideas and concepts across different roles, such as
  developers, designers, and stakeholders.
- Visualize historical changes of the software using Git versioning.

This approach streamlines the design process, fostering better collaboration
and more efficient development.
