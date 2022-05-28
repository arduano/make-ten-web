# Make Ten

Based on the make ten train game that me and friends played. The train carraiges here have 4 digit numbers, and the game involves using all 4 digits to make 10. For example, for 1234, the solution could be `4 + 3 + 2 + 1` or `4 * 3 - (2 * 1)` among others.

This simple website has a number text box that lets you enter a number up to 6 digits, and displays all the possible solutions, sorted by complexity.

<p align="center">
  <img width="460" height="300" src="https://github.com/arduano/make-ten-web/blob/main/preview.png?raw=true">
</p>

## Project structure

The root folder has [Nextjs](https://nextjs.org/) for serving the website, and the `calculator` folder is written in Rust and uses wasm-bindgen to generate a Typescript file (along with wasm binaries) that gets imported into nextjs.

## Dependencies

This project depends on Node.js (tested on v16), and Rust (1.60.0), [yarn](https://github.com/yarnpkg/yarn) (package manager for node.js), and also [wasm-pack](https://github.com/rustwasm/wasm-pack)

## Running

To run the project, make sure you have the dependencies installed, then run `yarn` to install all node modules, and `yarn dev` to run the dev server.

## Deployment

The project can be built using `yarn build`, which puts all the outputs into the `out` folder, then you can statically serve the contents in that folder.
