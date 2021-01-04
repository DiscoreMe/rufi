# About

Rufy is the 2d game engine mainly focused on creating novells with scripts in the Lua language.

Each scene must be in a separate folder and has the following architecture:

project/
- main.lua
- assets/
- scene-name/
    - main.lua
    - other_script.lua
- scene-two/
    - main.lua

### Available callbacks:
| name | description | arguments |
| --- | --- | --- |
| onInit | Called when loading the scene. In the root file, main.lua is used to call the very first scene | - |


### Available functions:
| name | description | arguments |
| --- | --- | --- |