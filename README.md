# K means clustering

Toy app to demonstrate the clustering of two datasets using a kmeans algorithm implemented in the Rust programming language.

It is live at [mlshowcase.cleverapps.io](http://mlshowcase.cleverapps.io).

## Project Scope

This is a learning project, started with no prior experience of Rust. The goal was to arrive at live application in a couple weeks, with following features:

- An implementation of the K-means algorithm.
- A minimal REST API to serve the results.
- Database interactions to store datasets.

## Stack

The limited scope of the project both in time and features provided me with clear criteria for my stack, it had to be:

- Straightforward: make use of technologies that are easy to learn and grasp during code reviews.
- Productive: keep time to production minimal.
- A great learning experience : teach me as much as possible about Rust and its ecosystem.

As a result I selected the Rocket framework to provide the API and serve a couple of static HTML/CSS/JS files.

## Learnings

Coming with no background in Rust, the project was very enriching. For anyone wishing to use the repo as learning material, here is what I covered over the past two weeks.

### Languague features

- Structs and Types
- Ownership and Borrowing
- Traits and Generics
- Error Handling (delightful in Rust!)

### The Rust Ecosystem

- Cargo and deployment
- The Rocket webframework
- Diesel ORM
- Serde for (de-)serialization
- Nalgebra for low dimensional linear algebra

## Perspective

The project remains a nice sandbox for playing and learning, and has much room for improvement.

Here are the most pressing improvements to the codebase:

- [ ] Fix the endpoint listing dataset ids
- [ ] Make the API more REST compliant
- [ ] Document said API with Okapi
- [ ] Implement Rocket's error catchers

And a bunch of ideas:

- [ ] Store reproducible benchmarks in database
- [ ] Run algorithm in separate threads
- [ ] Parallelize data processing using rayon
- [ ] Build a proper frontend with a rust framework like Yew
- [ ] Run processing client-side with WASM
- [ ] And so much more...
