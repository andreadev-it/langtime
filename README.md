# Langtime - Parsing dates in rust
This library is a personal project that allows rust programmers to 
parse dates written in the english language, both absolute and relative.
These are some examples of dates that you can currently parse:
- `2024-01-01 at 20:15`
- `28/02/2024 at 10 a.m.`
- `25 minutes ago`

Why creating this repo when [chrono-english](https://github.com/stevedonovan/chrono-english)
already exists? Well for two reasons. First of all, I didn't knew it existed.
Second, there are some formats, or combination thereof, that are
not parsable with chrono_english.

This library uses [nom](https://github.com/rust-bakery/nom), which
makes it extremely easy to add new formats to the parsable inputs.

## How to use it
The most basic use of this library looks like this:

```rust
fn main() {
    match langtime::parse("12/05/2024 at 8pm") {
        Ok(datetime) => println!("{:?}", datetime),
        Err(_) => println!("Cannot parse input as a date")
    };
}
```

By default, the parse function will discard any input that
is found beyond the datetime string. For example, this code
would also correctly match a string such as "12/05/2024 is the date",
even though "is the date" is not recognized as a time or date format.

There are some options that you can pass through a configuration
struct. The first one is the english dialect (US or UK). This
is currently only used to discern whether the date format for the
language is "dd/mm/yyyy" or "mm/dd/yyyy". 
Then, you can also force the library to check that the full string is
parsable as a date, so that the text "12/05/2024 is the date" will not 
be considered correct anymore, and will result in an error.

Here is how you can do it:

```rust
use langtime::{parse_with_config, ParseConfig, Dialect};

fn main() {
    let config = ParseConfig {
        dialect: Dialect::US,
        full_string_match: true
    };
    
    match parse_with_config("05/23/2024 at 9pm", &config) {
        Ok(datetime) => println!("{:?}", datetime),
        Err(_) => println!("Cannot parse input as a date")
    }
}
```

## Next goals
- [ ] Expand allowed tokens to separate parts of sentences
- [x] Correct month and year calculation
- [ ] Implement unit tests
- [x] Add missing time format
- [x] Cleanup text before parsing
- [x] Add configuration for english dialects (UK/US)
- [x] Add configuration to force matching to the full string

## Parsable data
### Dates
- [x] 2024-01-20 (ISO)
- [x] 20/01/2024
- [x] yesterday / tomorrow
- [x] 01/2024 (beginning of the month)
- [x] january 2024 (same as above)
- [x] 1st jan 2024

### Times
- [x] 17:00
- [x] 17:00:30
- [x] 5 p.m. / 5pm
- [x] 8 o'clock / half past 9 / a quarter to 10

### Relative times and dates
- [x] in 5 hours
- [x] 8 minutes ago
- [x] 2 hours, 8 minutes and 10 seconds ago
- [x] last friday
- [x] next tuesday
- [x] saturday / this saturday
- [x] 2 days ago
- [x] in 3 months

### Full dates and times
- [x] 2024-01-01T20:30:10
- [x] yesterday at 17:00
- [x] tomorrow at 8 p.m.
- [x] 2 days ago at 5 a.m.
- [x] last friday at 9:00 
