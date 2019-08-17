# Joint-rs 

A set of Yew elements designed to make implementing interactive network/graph visualizations easier in Rust.


## Relies on:

- Rustux for web-focused state managemenet with Yew hooks
- Yew for graphical elements

## Goals of joint_rs:

- Store a set of network/graph data 
- Visualize a set of network/graph data
- Allow interactivity with data 
- Load data from a remote backend
- Modify remote data from user interactivity
- Render to html (wasm target)

## Coming Up:

- WebSocket and WebRTC collaboration

<!-- ### Structure
Each element has its own "API" backed by Rustux. Traditional functionality of Yew remains, with the option of logging some actions in the store which is retained with Global and Local service worker. -->

