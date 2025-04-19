# Architecture and Design

## Design decisions

Initial requirements:

- TUI app.
  - Initially wanted to use Qt and C++, but Qt's policies are garbage,
    so Rust was chosen instead as a favorite language.
- Support the latest game version.
  - Less work to do. ![:ohh_yeah:](https://steamcommunity-a.akamaihd.net/economy/emoticon/:ohh_yeah:)

Desired Quality Attributes:

- Simplicity / Usability - not everyone likes TUI,
  so it must be easy to read, understand, and use.
- Portability - some players use Linux.
- Testability.
- Safety - make sure it doesn't ruin the game and files.

Extras:

- Prefer to use Domain Driven Design (for extra practice with it).

## Domains

The following domains exist within the editor:

- Savefile handling.
- Editor App.

These domains are not accurately reflected by crates structure.

## Savefile domain

Savefile domain consists of multiple crates.

### Savefile handling

Initially, there was only one crate for all the save files.

However, individual files are independent units.

- Player Progress is an "encrypted" binary file.
- Online Profile is a simple unencrypted binary file.

Despite using similar strategies for data representation
(e.g., length-data pattern, versioning, etc.),
one file does not depend on the others,
so it makes sense so keep them as separate units.

Thus, each individual file is an independent subdomain
within a savefile domain.
This decision results in less cluttered and more focused crates.

### Savefile Environment

There is an Environment crate reused in all savefile crates.

This little subdomain simply allows to locate the saves folder,
which may be different from the directory used by the game.

This subdomain sounds unnecessary, but it improves flexibility and testability.

- No need to remember file name and location.
- No need to mock inputs.

## Editor App Domain

The Editor domain is the biggest and the hardest,
so it was intentionally modularized and simplified.

Multiple interesting design decisions were applied.

### Modules are subdomains

Initially, modules were organized based on the object type:

- Collections.
- Widgets.
- Styles.
- Components (tables, tabs, and other UI related items).

For example, the player progress tab logic was in roughly the following modules:

- Tab and its behavior - in `Components`
- Collections (internal but reusable) - in `Collections`
- Visual components - in `Widgets`
- Styles for those widgets - in `Styles`

This project structure made subdomains of the Editor domain less clear.
It turned out confusing, and resulted an unnecessary nesting.

Thus, a more fitting solution was chosen - organizing by subdomains.
Each subdomain consists of everything it needs to work,
except for generic, reusable commons.
Specifically, a subdomain consists of the following parts:

- Business logic (funny to call it "business" logic for this project,
  but technically this is the fitting term).
- Specialized, non-reusable dependencies.
  - Custom collections, widgets, and styles.
  - Any internals for business logic.

For example, the same player progress tab now resides on a dedicated module,
and its internals are in the same module too.

This way of organizing files resulted in a cleaner developer experience
and a clearer dependencies between modules.

### Global state and channels

Despite synchronous environment and single-threaded execution,
Watch channels from `tokio` crate store the global app state.
Those channels are easy-to-use and save from manual work a lot.

Initial prototype held an entire savefile in a channel
with multiple senders (i.e., every tab/table),
and a single consumer that saved the file to the disk on any change.

This approach works well enough, but it makes app components less modular.
For example, a table of unlocked characters
does not have to know about music and arenas,
but with this design it could modify data it shouldn't
and cause some raised eyebrows.

Therefore, in an attempt to modularize and simplify the logic,
more channels were introduced at first.
Instead of one channel that holds the entire savefile,
there were multiple channels that held only parts of the struct.
Frankly, there was one channel per each field.

Then, every part of the struct had to be "collected" roughly as follows:

```rust
if field1_recv.has_changed() {
    let field1 = *field1_recv.borrow_and_update();
    big_struct.field1 = field1;
}

if field2_recv.has_changed() {
    let field2 = *field2_recv.borrow_and_update();
    big_struct.field2 = field2;
}
```

Unfortunately, this design required repeating those
`if` blocks for EVERY channel (for every field).
Then, only if any of the fields changes,
it could update the savefile and save it to the disk.
This design was also error-prone,
because it's easy to forget to write
an `if` block when adding a new channel.
Simply put, it's just annoying, repetitive, and
unfun - smells like bad design, and needs a re-design.

Thus, a new killer way was designed: proxies.

- Inside a proxy, there is a sender that holds the entire savefile.
- The proxy only allows access to a specific part of the savefile.
  - For example, there are separate proxies
    to each field of player progress.
  - If needed, one could write a proxy that accesses any fields.
- The receiver and of the channel reacts to changes to the entire savefile,
  instead of reacting to separate fields and combining them together.

These proxies allow for modular enough components (tabs and tables)
without being error-prone, cumbersome, and annoying to use as previous designs.
Although having them smells like "bad design" too,
it achieves the goals "better" than previous solutions.

## Domain types and "Safety"

The approach to type design was inspired
by the book [_"Secure by Design"_][secure-by-design].

The book suggests to avoid primitive types to describe the business units.
Instead, each unit should be represented by its unique type,
and all the invariants of this business unit should be held by the type.
In short, the rule is "either you create a valid instance only, or don't create it".

This approach was applied to several types,
such as Nickname, Arenas, Titles, and others.

Considering the scope of the project, it was not particularly necessary.
However, this statement does not sounds like a convincing argument
against using this approach.

In addition, this approach provides the following benefits:

- Readability.
  - You always know what this type means,
    and can always see how it works - all in one place.
  - "Code is documentation" approach becomes real,
    no need to write extra docs about how it should work.
    Only "why it's designed this way".
  - Custom methods and names express the intent more clearly.
- Safety - less error-prone.
  - Type-level invariants prevent the invalid state.
  - Since all the necessary safety checks are done in one place,
    you won't need to add them somewhere else later.
  - The compiler makes sure you don't cause silly problems.
- Modularity.
  - Business logic related to one (sub)domain stays in one place,
    instead of being spread over the entire codebase.

~~Yes, this was almost like a conspectus.~~

[secure-by-design]: https://isbnsearch.org/isbn/9781617294358
