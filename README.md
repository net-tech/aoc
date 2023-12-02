# net-tech-'s Advent of Code

This repository contains my solutions to the [Advent of Code](https://adventofcode.com/) challenges. I'm using this as an opportunity to learn [Rust](https://www.rust-lang.org/).

## Running

To run the solutions, you'll need to have Rust installed. You can then run the solutions with:

```bash
cargo run <day> <part>
```

## Log

### 2023

Well, this is my first time doing Advent of Code. Normally I would use TypeScript, but I heard this could be an opportunity to learn a new language, so I chose Rust. I did a fundamentals course before this, so it should be just improving those skills and learning how to use them.

#### Day 1

##### Part 1

Okay so the first though I had was to use regex, but I wanted to challenge myself, so I elected not to use it. I don't like that I used a nested loop, but I don't know of a better way, at present. My solution took some trial-and-error, and I probably should have used tests to make it easier for myself. Overall, I'm happy.

##### Part 2

Although considering regex for part two, I still wanted to try to do it without it. This ended up being much, much harder than I thought it would be. My original solution was just replacing everything, but there were overlaps and edge cases to consider. After making my code pass all the examples from the AOC website, I was stuck for a while. After going to Reddit for help, and getting a hint ("What do you expect '1eightwo' to be?"), I knew what the problem was. My thought after that was to add a duplicate letter to everything that could be an overlap but that felt like cheating. In the end, I decided to only add a duplicate letter for the letter that was being replaced. I accidentally doubled-down on my loops here, but the code takes around 15 ms to run, so it's not _meh_.