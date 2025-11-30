# Guessing Game
Guessing Game (from the Rust Book) by juniorpen01 finished on 11-30-2025 at 10:04 PM. Guesses are from 1 to 100.

## Class Diagram
Hacky class diagram I made. Both somewhat proud and very disappointed by it.
```mermaid
classDiagram

class Guesser {
  +guess(num: i32) std::cmp::Ordering
}

class GuessReader {
  +read() i32
}

class GuessResultProcessor {
  +process(result: std::cmp::Ordering) bool
}

class Game {
  +play()
}

GuessResultProcessor ..> Guesser
Guesser ..> GuessReader
Game ..> GuessReader
Game ..> Guesser
Game ..> GuessResultProcessor
```