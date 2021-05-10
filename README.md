# Rusty Beaver
Implementation of Busy Beaver problem in Rust

Link to rust playground:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9750515113ade36327de394e2cabe3aa
## How to run
Program can be build by `cargo build`. After that binary can be found in a subfolder of `target/`.

Program can be run either by `cargo run` or by running binary (for example `.target/debug/busy-beaver`).

Program takes transitions in stdin separated by whitespaces.

Example usage:
```
cargo run < example.in
```
```
cargo build
.target/debug/busy-beaver < example.in
```
## Input Layout
Input should be written accodring to following rules:

Each transition is a 3 sign string \<sing\>\<transition\>\<state\>
* sign is either `0` or `1`. Denotes which sign should be written on a tape
* transition is either `L` or `R`. Denotes which way the head of tape should move (left or right)
* state is a capital letter `A`-`Z` | `h`. If a capital letter is accessible during simulation of busy beaver, all letters in alphabet before this letter should have transitions provided
* `h` is a special state and no transitions can be provided for it. It is the halting state

Numer of transitions should be even. `n`-th pair of transitions represents transitions for `n`-th letter in alphabet. First transition in pair denotes transition when there is `0` under the head of machine, and second transition is a transition when there is `1` under the head of machine.
So input:
```
0RB	1Rh	0LC	1RA	1RB	1LC	
```
Represents turing machine with state transitions:
```
(A,0) -> (0,right,B)   (A,1) -> (1,right,h)
(B,0) -> (0,left,C)    (B,1) -> (1,right,A)
(C,0) -> (1,right,B)   (C,1) -> (1,left,C)
(h,0) -> halt          (h,1) -> halt
```