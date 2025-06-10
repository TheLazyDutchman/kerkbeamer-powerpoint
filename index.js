import SlideShow from "slideshow";
import url, { URL } from "url";
import { spawn } from "node:child_process";


var runner_url = new URL("runner/target/release/runner.exe", import.meta.url);
var runner_path = url.fileURLToPath(runner_url);
var runner = spawn(runner_path);

runner.stdout.on("data", (data) => {
	console.log(`From runner: ${data}`);
	if (data == "Spawned\n") {
			var slideshow = new SlideShow("powerpoint");
			console.log(slideshow);
			slideshow.boot()
				.then(() => slideshow.open("test.pptx"))
				.then(() => slideshow.goto(2))
				.then(() => process.exit());
	} else {
		console.log(`Could not handle: ${data}`);
	}
});
runner.on("close", (code) => console.log(`Runner exited with code: ${code}`));