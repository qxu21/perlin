# perlin
An experiment to build Perlin noise with Rust.

As of right now, when compiled with Rust, the code outputs ASCII Perlin noise to stdout. The characters `.,;#` are used for (-1,-0.25), [0.25,0), [0,0.25), and [0.25,1) respectively.

See [adrianb.io](https://adrianb.io/2014/08/09/perlinnoise.html) and [Wikipedia](https://en.wikipedia.org/wiki/Perlin_noise) for more on Perlin noise.

```
qxu21@genesis: perlin$ ./target/release/perlin 
;;;;####;;;;;;;;;;,,,,,,;,,,,,,,;;;;
;;;#####;;;,;;;;;;,,,,,,,,,,,,,;;;;;
;;;####;;;,,;;;;;;;,,,,,,,,,,,,;;;;;
;;#####;;,,,;;;;;;;,,,,,....,,,;;;##
;;####;;,,,,,;;;;;;,,,,;.....,,;;###
;#####;;,,,.,,,;;;;;;;;;.....,,;;###
;####;;;,,..,,,,,,;;;;;;.....,,;####
;####;;,,,..,,,,,,;;;;;;.....,;;####
;;##;;;,,,,,,,,,,,,;;;;;....,,;;####
;;;;;;;,,,,,,,,,,,,;;;;;...,,,;;;##;
;;;;;;;,,,,,,,,,,,,;;;;;,,,,,;;;;;;;
;;;;;;,,,,,,,,,,,,,;;;;;,,,,;;;;;;;;
;;;;;;;;;,,,;,,,,,,,,,,;;;,,,,,,,,,,
,,,;;;,,,,,,,,,,,,,,,;;;;;;;,,,,,,,,
,,,,,,,,,,,,;,,,,,,,;;;;;;;;;,,,,,,,
,,,,,,,,,,,,;,,,,,,,;;;;;#;;;;,,,.,,
.,,,,,,,,,,,;;,,,,,;;;;;####;;,,,..,
.....,,,,,,,;;;;,,,;;;;;####;;,,,...
......,,,,,,;;;;;;;;;;;;####;;,,,...
,.....,,,;;;;;;;;;,,,;;;#####;;,,...
,,...,,,,;;;;;;;;;,,,,,;;####;;,,...
,,,..,,,;;;;;;;;;,,,,,,,;;##;;;,,,,,
,,,,,,,,,;;;;;;;;,,,,,,,;;;;;;;;,,,,
,,,,,,,,,;;;;;;;,,,,,,,,;;;;;;;;,,,,
```
