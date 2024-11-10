---
id: code_sum_up
aliases: []
tags: []
---


* State machine for switching from game to editor?

  -> e key pressed: "editor_asked"
  -> on editor_asked: 
  --> user input -> "disabled"
  --> if editor is loaded : editor_setup
  --> if editor is not_loaded : editor_setup
  -> on editor_loaded: -> editor_setup
  -> on editor_loaded_finish -> user input : enable.

* Structure:
  - editor
  -- main -> keyboard input / move camera.
  - game
  -- main -> keyboard input
  -- game logic / keyboard input.
  - common
  -- level
  -- camera
  -- hedgehog
  -- asset_loader

