# Demo of a NotionMindmap tool

Joint-rs provides the bones and some muscle for you to start building your "batteries-included" canvas-DOM PWA. 

Here, we implement a backend system (Notion-rs) that hooks into some of the default Joint-rs backend loading and updating methods.

The app is:
- [ ] Fast
- [ ] Progressive
- [ ] Offline capable
- [ ] Backend-api agnostic

All right out of the box.




---
Structure of this project:

- Extend the JointApp (all the functionality and interactivity) in ` app.rs `
- Lay out the actual design of your app in ` renderer.rs `
- Design variants of your JointElemnet using ` elements/graphelement.rs `
