# small music player built with rust and svelte (Tauri)

- To run, clone repo and run `bun tauri dev`
- I've only tested it on Mac, but should compile on windows

# To play music once running, click the three context menu and select browse.
- navigate to a folder where there's music (mp3 only for now)
- Once folder has selected, click on Audio, and window with songs will pop up.


### N.B: this is a small project I did as a proof of concept while learning run,
- There are many things which can be improved such as error handling,
- Improvement on UI, such as close nav when clicking away and adding some animations.
- Completely operates with an inmemory vector. I plan to rewrite the app and add sqlite as a data store
