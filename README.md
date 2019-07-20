# Mapping tool for discrete block systems

This library aims to provide a framework for block-based state management where each element holds references to a parent

Several different models are possible:
* Single list of elements
* Tree of elements

Our application needs to be optimized for:
* Inserts into the tree
* Rendering of all elements
* Dropping entire sets of elements at a time

As our application is building a tree, we will store the structure in a tree as well.
This is essentially a network constructor.

Element:
    Parent:
    Children:











Dependencies:
- Notion-rs for access to Notion's API
    - Reqwest for making http POST/GET requests
    - Serde for JSON serialization / deserialization

- Joint-rs for rendering and laying out elements
    - A collection of extensible Yew elements intended to be used for canvas-graph style apps
        - Sidemenu bar,
        - Top menu bar,
        - Canvas area
        - Elements on the canvas
        - Interactions with elements on the canvas
    - Data model for managing the elements on the canvas

        

- Yew for laying out all of our components

