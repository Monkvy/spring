## Initialize
* SFML
* Mouse state
* Variables: `running, selected`
* Particle vector
* Spring vector


## Mainloop
### Events
* Default close operation
* Pause / Resume
* Mouse event
	* Left -> if mouse on particle set `selected`
	* Update mouse state

### Update
* Mouse states events
	* Left -> if not mouse on particle, create particle
	* Right -> delete hovered particle / spring
* if running, update particles

### Draw
* Clear screen
* Springs, then particles
* `window.display();`

