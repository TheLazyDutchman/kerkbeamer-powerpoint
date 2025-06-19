import Slideshow from "slideshow";
import url, { URL } from "node:url";
import { spawn } from "node:child_process";

class PowerPoint {
	slideshow;

	constructor() {
		const runner_url = new URL("runner/target/release/runner.exe", import.meta.url);
		const runner_path = url.fileURLToPath(runner_url);
		const runner = spawn(runner_path);

		return new Promise((resolve, reject) => {
			runner.stdout.on("data", (data) => {
				if (data == "Spawned\n") {
					this.slideshow = new Slideshow("powerpoint");

					resolve(this);
				}
			})
		}).then((_) => {
			this.slideshow.boot();
			return this;
		});
	}

	open(path) {
		return this.slideshow.open(path).catch((e) => console.log(`Got an error when opening slide at path '${path}': ${e}`));
	}

	goto(slide) {
		return this.slideshow.goto(slide).catch((e) => console.log(`Got an error when moving through the slide: ${e}`))
	}
}

async function run() {
	let powerpoint = await new PowerPoint();

	console.log("Opened PowerPoint");
	await powerpoint.open("TestData/Test.pptx");
	console.log("Opened Presentation");

	await powerpoint.goto(3);

	process.exit();
}

run();