import Slideshow from "slideshow";

export default class PowerPoint {
	constructor()
	declare slideshow: Slideshow
	open(path: string): Promise<"OK">
	goto(slide: number): Promise<"OK">
}