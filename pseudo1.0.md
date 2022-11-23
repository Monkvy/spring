## Initialize
* SFML
* Mouse state
* Variables: `running, selected, brush, tools`
* Particle vector
* Spring vector


## Mainloop
### Events
* Default close operation
* Pause / Resume
* Iterate tools
* Mouse event
	* Right if tool create,
		* if mouse on particle, set selected / create particle 
		* else, set selected to none, create particle
	* Update mouse state

### Update
* Mouse states events
	* Left -> if no particle selected, move particle 
	* Right 
		* if tool is delete, delete hovered particle / spring
* if running 
	* if a / b of a spring doesnt exist, remove spring.
	* update particles, then springs

### Draw
* Clear screen
* Springs, then particles
* `window.display();`

