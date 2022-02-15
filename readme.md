# Solving the Mountain Car Problem Using Genetic Learning Algorithm

This project solves the mountain car problem (discretized) using a Genetic Learning Algorithm implemented in Rust.

## How to use the code

To run the code, run
```
$ cargo build --bin mountain_cars
$ ./target/debug/mountain_cars
```

## Goals

1) Become more familiar with the Rust programming language for my Summer 2022 Collins internship
2) Fool around with "learning-based algorithms".

## Accomplishments
This simple project provided an opportunity for me to practice and brush up my Rust coding skills. At the same time, I built a simulation that simulates the physics of a mountain car problem in car_sim.rs. The physics of the system (engine power, discretization quality,  slope and shape of the mouintain, gravity, etc) can be adjusted in the settings.rs file.

Furthermore, the code for the genetic learning algorithm is taken from https://github.com/innoave/genevo/tree/master/examples. This code is adopted in mountain_cars.rs to fit the needs of the mountain car problem.

## Basic idea

The objective of the mountain car problem is to get the car to a finish line at the top of the mountain. We can change this derivation slightly to maximize the car's x-coordinate. Without loss of generality of assuming the finish line is on the right, if it is possible to get to this finish line, this slight derivation will get the car over the finish line. 

The infinite monkey problem that is often solved using genetic learning provides a good baseline to build off of. Since every character has 8 bits, and every 2 bits has 4 possibilities, we can map each of these 4 abilities down to 3 possible commands for the car: go backwards, forwards, or engine idle. Consequently, each string of length n represents a sequence of actions over the span of 4*n discretised time. 

Feeding this string into the car simulation produces a final result of where the car is at the end. The algorithm has proven this objective to be quite smooth and convex for the genetic algorithm to solve successfully.
