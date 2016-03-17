<a name="v0.5.0"></a>
## v0.5.0 (2016-03-17)


#### Features

*   Regex for TMS parsing now statically created ([ce9fa5f4](ce9fa5f4))



<a name="v0.4.0"></a>
## v0.4.0 (2016-03-07)


#### Bug Fixes

*   Make num_tiles_in_zoom more robust & return Option ([d340bb0d](d340bb0d))

#### Features

*   Add size_hint to AllTilesToZoomIterator ([048632b2](048632b2))
*   Add iterator that stops after a certain zoom ([621454e6](621454e6))
*   Add num_tiles_in_zoom function ([b8890c28](b8890c28))



<a name="v0.3.1"></a>
### v0.3.1 (2016-03-07)


#### Bug Fixes

*   Make Tile.{zoom/x/y} pub so we can access them ([6dca0ac3](6dca0ac3))



<a name="v0.3.0"></a>
## v0.3.0 (2016-03-03)


#### Bug Fixes

*   Reject zoom >= 100 ([1127a422](1127a422))

#### Features

*   Add Tile::from_tms to parse a TMS string ([1bf8accb](1bf8accb))



