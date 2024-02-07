# Turinger
#### Interpreter for the Turing machine language made by me, written in Rust

Have you ever wanted to code on Turing machine? No? Me neither, but I made this anyway and even more developed (???) a language to do this. 

## Features

- This thing interprets code quite fast;
- Code doesn't look like a complete shitload of garbage (IMHO);
- States table even looks like a f-ing table; 
- Even more, if you are a pervert of some kind, you can debug this thing by viewing the tape and the state of the machine in the console on every step of the execution.
- If you are pervert of another kind, you can do assertion thing, which will give you an error if the final result of the machine doesn't match the expected result.
- If you code on Turing machine as bad as I do, you can stop the execution emergently after reaching a certain number of repeating steps.
- Wanna more debugging stuff? Here you go! You can define a cell in the table which can't be reached normally — if machine reaches the cell, it will stop and give you an error.
- Not enough? You can make machine print debug info about current state and the state via making it print a dollar sign (`$`).
- You can write comments, BTCH, because cool languages have comments and I just wanna worm in.
- You can predefine the tape for execution — you can even predefine the caret position — cool, ain't it?

## `.tur` file format

Here we come to the intersting part: how do you write code for this thing? Well, you can write it in a `.tur` file. The file **should**/*can* contain the following sections:

1. **Alphabet** contains symbol that machine can use in its work. Written as just a row of symbols separated by spaces. It can contain both upper- and lowercase Latin and Cyrillic letters, digits, asterisk (`*`), slash (`/`), plus (`+`), minus (`-`), equals (`=`), exclamation mark (`!`), at sign (`@`), hash (`#`). Dollar (`$`) sign, underscore (`_`), ampersand (`&`) sign are preserved and using them in the alphabet will raise an error.
2. **States** contains the list of states that machine can use. Written as just a row of states separated by spaces. State name consists of `q`-letter and a digit (or few of them).
3. *Tape* contains the initial tape for the machine. Written as a row of symbols separated by spaces. If you want to set the caret position, you can surround a symbol in square brackets — `[]` — to do this. If you don't set the caret position, it will be set to the first cell of the tape.
4. *Expected tape* contains the expected tape for the machine. Written as a row of symbols separated by spaces. If you don't set this section, the machine will not check the result of the execution.
5. *Overflow definer* contains the number of **repeating** steps, after which program will raise an error automatically.  If you don't set this section and somehow will mess up the things, the machine will slowly consume your tasty RAM and CPU until you stop it manually. Written as a number.
6. **State table** consists of table root, column headers and rows.
Table root is written as `[r]` — after it go column headers, which are just symbols from alphabet in double quotes (like `"a"`).
On lines after it should be placed rows — they begin with `|` sign followed by state name and **actions**, separated by spaces — like that: `|q1 bR_`.

Comments are written as `//` and everything after them is ignored.

### Actions

Action is a string of symbols, which consists of *letter*, *direction* and *state*.

- *Letter* is a symbol from the alphabet, which will be written to the current cell. Can also be `_` (don't write anything), `&` (erase the cell contents) or `$` (print debug info).
- *Direction* is a direction, in which the caret will move after writing the letter. It can be `L` (left), `R` (right) or `S` (stay). As shorthabds you can use `<` (left), `>` (right) and `.` (stay). You can use `_` again to do the skip thing — it's equal to `S` here.
- *State* is a state, which the machine will go to after writing the letter and moving the caret. It should be a state from the list of states. For short you can use just a digit. Special symbols are `!` (unreachable cell), `e` (exit point). Skip thing is also available here — it leaves the state unchanged.

### Complete example

```tur
alphabet: a b
states: q1 q2

tape: abababa
expect: bbbbbbb

overflow on: 1000 // 1000 steps is enough for everyone

[r] "a" "b" "&"
|q1 $R_ aR_ _L2
|q2 bL_ _L_ _Re
```

## CLI usage

`turinger.exe` is a command-line tool. You can use it like that:

```sh
turinger.exe <path-to-tur-file>
```

You also can use flag `-d`/`--debug` to make the machine print debug info on every step of the execution.