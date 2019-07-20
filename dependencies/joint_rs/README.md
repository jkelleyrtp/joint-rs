# JointRS

A graph modeling 


Add item -> O(1)


Owner: asdad
children: asdads, asdasd, asdad, 

get all elements without children:
    loe -> iterate through all and find ones with missing child field OR look at children of root and their children and return the ones without children
    tree -> 2nd method 

get len
    loe -> remember number of items in counter or use the hashmap.len field
    tree -> remember number of items in counter

hashmap vs tree


changing parent:
    loe -> change parent field of element, re render


"
- Joint-rs for rendering and laying out elements
    - A collection of extensible Yew elements intended to be used for canvas-graph style apps
        - Sidemenu bar,
        - Top menu bar,
        - Canvas area
        - Elements on the canvas
        - Interactions with elements on the canvas
    - Data model for managing the elements on the canvas



Joint-rs focuses on the canvas area and provides a set of methods/callbacks/etc. to make it easier to develop wasm applications that focus on creating a graphical representation of a data model.



Data model -> Display

Display element basic objects:


We provide a template for a data model and all the display components.

# Data model


# Display
- Layout mode -> enum for the data 

Data Model -> Renderer

Renderer -> Renderable 
- Layout Mode (enum)

Traits:
- RenderableData


```rust 


enum OutputFormat {
    Html
}

struct HtmlGraph {

}

impl HtmlGraph {
    fn render() -> Result<T,Error> {

    }
}


impl RenderableModel for HtmlGraph {
    fn render( OutputFormat: OutputFormat, ) -> Result<T, Error>{
        match OutputFormat {
            Html => {
                self.render_()
            },
            WebGl => {
                
            },
            Conord => {

            }
            _ => ()
        }

    }

}

Renderer.render( OutputFormat::Html , )


```


RenderableModel controls both the state and display of the renderered output.


```rust 
trait RenderableModel {

}

struct HtmlGraphState {

}


struct HtmlGraph {

}
impl Renderable for HtmlGraph {

}

```


```rust 
struct DataModelOutput {

}

impl RenderableData for DataModelOutput {
    fn update() -> Self
    fn display() -> Self
}
```



