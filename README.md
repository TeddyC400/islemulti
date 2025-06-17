# islemulti

islemulti is a server software designed to transform [Lego Island (1997)](https://en.wikipedia.org/wiki/Lego_Island) from a singleplayer game into a multiplayer one.

This repository is focused on the server side. For the client side network compatibility, check out [this repository here](https://github.com/TeddyC400/isle/tree/multiplayer) which is a fork of the [Lego Island decompilation project](https://github.com/isledecomp/isle).

## Setup
1. Install the programming language: [Rust](https://www.rust-lang.org/tools/install)
2. Download the ```islemulti``` source code
3. Open command prompt and ```cd``` into the project
4. Execute the following command: ```cargo run```
5. That's it, the server now runs!

Note: The server IP address and port is currently set to ```127.0.0.1:9001```

## Is Making Lego Island Multiplayer Compatible Possible?
Kind of. As long as we have access to the original game code, we can do extensive modding with it. That includes implementing networking code to interact with the server and synchronize updates with other players.

My hope with this project is that multiplayer will work to some degree, players walking around in the main world. Everything else will be major technical challenges, such as maybe trying to get racing (cars and jet skis) to work.

## What's The Point? The Gameplay Is Not Designed For Multiplayer.
True. This project is experimental and just for fun. However, this is to demonstrate the game can be modded far beyond the original gameplay. I don't expect much to come out of the multiplayer project beyond players walking around in the main world, but you never know!
