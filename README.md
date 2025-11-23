# imvi
Just another **im**age **vi**ewer. Main goal is to be extremely responsive without significant freezing an stuttering during loading and image scrolling.

## Notes on compiling on Windows
- use [TDM-GCC](https://github.com/jmeubank/tdm-gcc) (tutorial [here](https://gist.github.com/glycerine/355121fc4cc525b81d057d3882673531))
- if there is an error **"This app can't run on your PC"** after build, set GOEXPERIMENT=nodwarf5 (`go env -w GOEXPERIMENT=nodwarf5`)

## TODO
- [ ] loaded images caching
- [ ] image zooming
- [ ] gallery view
- [ ] thumbnail make in app