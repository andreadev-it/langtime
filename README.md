# Langtime - Parsing dates in rust
This library is a personal project, currently in development, that
aims at easily allow rust programmer to parse dates written in the
english language, both absolute and relative. These are some examples
of dates that will be parsed:
- `2024-01-01 at 20:15`
- `28/02/2024 at 10 a.m.`
- `25 minutes ago`

Why creating this repo when [chrono-english](https://github.com/stevedonovan/chrono-english)
already exists? Well for two reasons: first, I didn't knew it exists.
Second, there are some formats, or combination thereof, that are
not parsable with chrono_english.

This library uses [nom](https://github.com/rust-bakery/nom), which
makes it extremely easy to add new formats to the parsable inputs.

## Parsable data
### Dates
- [x] 2024-01-20 (ISO)
- [x] 20/01/2024
- [x] yesterday / tomorrow

### Times
- [x] 17:00
- [x] 17:00:30
- [x] 5 p.m. / 5pm
- [ ] 8 o'clock / half past 9 / a quarter to 10

### Relative times and dates
- [x] in 5 hours
- [x] 8 minutes ago
- [x] 2 hours, 8 minutes and 10 seconds ago
- [ ] last friday
- [ ] next tuesday
- [x] 2 days ago
- [x] in 3 months *

\* months and years are currently not calculated correctly: a month is
considered as 4 weeks, a year as 365 days.
