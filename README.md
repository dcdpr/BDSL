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

- [`bnb-ast`](./crates/ast) — The public `Breadboard` AST.
- [`bnb-butter`](./crates/butter) — Interactive breadboarding GUI.
- [`bnb-converter-json`](./crates/converters/json) — Convert a `Breadboard`
  type to/from JSON.
- [`bnb-parser`](./crates/parser) — Parses the "bnb" DSL into a `Breadboard`
  type.

## Example

### Understanding the DSL Syntax

Bread'n'Butter's Domain Specific Language (DSL) allows you to define the
layout and interactions of a software project in a simple, readable format.
The DSL is designed to represent the screens (or "places") of your
application, the affordances within them, and the relationships between them.

Here's a basic example to illustrate the syntax and concepts:

```bnb
// Breadboard documents can have regular comments (marked with `//`) to add
// context for the reader of the DSL.
//
// These comments are stripped before generating the AST.

/// However, certain items in the DSL support *descriptions*. These
/// descriptions are denoted using `///`.
///
/// Descriptions can be added to places and affordances.
place Registration
    include Header

    /// This is a description for the `User Fields` affordance.
    User Fields
    > include CommonUserFields
    > Username
    > Password
    > > Show Characters
    > > Forgot Password
    > Full Name

    Sign Up -> (success) Home
            -> (failure) Support

    sketch sketches/registration.png
        [50,20 110,40] Sign Up

place Support
    include Header

    Error Message
    Try Again -> Registration

    position > Registration
    sketch sketches/registration.png
        [50,20 110,40] Try Again

place Home
    include Header

    Dashboard

    position 0, ^ Registration - 12
    sketch sketches/home.png

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

6. **Positioning:**

   The `position` attribute in the DSL is designed to specify the preferred
   placement of a `Place` within the breadboard layout. It serves as a
   suggestion for layout engines to optimally display the breadboard
   components.

   The syntax for `position` is defined as `position <x>, <y>`, where `<x>`
   and `<y>` represent the coordinates and can be expressed in several ways:

   1. **Absolute Coordinates**: Direct numeric values (positive or negative)
      indicating a specific point on the board, e.g., `10`, `-12`.

   2. **Relative Coordinates**: Reference to another `Place`'s position, e.g.,
      `Home`. This sets the current `Place` in relation to the referenced
      `Place`.

   3. **Relative with Offset**: A `Place` name followed by an offset value,
      e.g., `Home - 10`. This positions the current `Place` relative to the
      named `Place`, adjusted by the specified offset.

   4. **Relative with Pivot Point**: A `Place` name with an optional pivot
      indicator (`<`, `^`, `>`, `_`), defaulting to "center" if not specified.
      Each pivot point indicates a specific direction relative to the named
      `Place`:

       - `<`: Left
       - `^`: Above
       - `>`: Right
       - `_`: Below

   5. **Assumed Coordinates**: If only one coordinate is given and it's a
      relative position, the second coordinate is assumed to be relative to
      the same `Place` with no offset and a center pivot. However, if the
      first coordinate uses a `top` (`^`) or `bottom` (`_`) pivot, then it's
      interpreted as a `y` coordinate, and the assumed second coordinate is
      treated as an `x` axis position.

7. **Defining Sketches:**
   - `sketch` is used to provide visual representations of places.
   - Each sketch can have one or more "clickable areas" (using the notation
     `[<y-top>,<x-left> <y-bottom>,<x-right>]` in pixels), used by the GUI to
     navigate by clicking on the sketch images.
   - Each clickable area must reference one affordance in the same place that
     has one or more connections.

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
