#![recursion_limit="128"]

/*!
This crate provides a framework for building graph+diagramming based web-apps leveraging the yew component system.

joint_rs aims to implement some of the useful but boring functionality like an undo-redo system, saving to local storage, and accomodating different remote backends

Much of joint_rs should just work out of the box from adding components to saving the canvas state for later (local storage in the browser or to an endpoint).

## Goals of joint_rs:

- Store a set of network/graph data 
- Visualize a set of network/graph data
- Allow interactivity with data 
- Load data from a remote backend
- Modify remote data from user interactivity
- Render to html (wasm target)

## Some niceties of joint_rs:

- undo-redo system
- Solid Default components

*/

/// The heart of the joint_app - the struct that manages the state and all the elements
///
/// The joint_app instance that handles all internal communication and provides
/// callbacks for user interaction that is relevant to external services.
pub mod core;



/// Converts your state and associated elements into the final html rendered to the screen
///
/// Provides a framework to assemble your elements that can be handled by the joint_app
/// Also determines the layout of the elements in the workspace
/// A renderer needs to output some sort of renderable *thing* (anything) and is only valid
/// if it generates the rendered output from the JointApp 
pub mod render;



/// A bunch of default elements that might be useful to your joint_app
///
/// These include menu containers, the canvas, and buttons for menus.
/// If you want to override the defaults, just design your own yew component
/// and make sure it satisfies the relevant traits for each element type.
pub mod elements;
