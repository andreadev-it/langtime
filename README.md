# Langtime - Parsing dates in rust
This library is a personal project, currently in development, that
allows rust programmers to parse dates written in the
english language, both absolute and relative. These are some examples
of dates that you can currently parse:
- `2024-01-01 at 20:15`
- `28/02/2024 at 10 a.m.`
- `25 minutes ago`

Why creating this repo when [chrono-english](https://github.com/stevedonovan/chrono-english)
already exists? Well for two reasons.
First of all, I didn't knew it existed.
Second, there are some formats, or combination thereof, that are
not parsable with chrono_english.

This library uses [nom](https://github.com/rust-bakery/nom), which
makes it extremely easy to add new formats to the parsable inputs.

## Next goals
- [ ] Expand allowed tokens to separate parts of sentences
- [x] Correct month and year calculation
- [ ] Implement unit tests
- [x] Add missing time format
- [ ] Cleanup text before parsing
- [ ] Add configuration for english dialects (UK/US)
- [ ] Add configuration to force matching to the full string

## Parsable data
### Dates
- [x] 2024-01-20 (ISO)
- [x] 20/01/2024
- [x] yesterday / tomorrow
- [ ] 01/2024 (beginning of the month)
- [ ] january 2024 (same as above)

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
- [x] in 3 months *

\* months and years are currently not calculated correctly: a month is
considered as 4 weeks, a year as 365 days.

### Full dates and times
- [x] 2024-01-01T20:30:10
- [x] yesterday at 17:00
- [x] tomorrow at 8 p.m.
- [x] 2 days ago at 5 a.m.
- [x] last friday at 9:00 
