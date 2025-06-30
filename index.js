import url, { URL } from "node:url";
import { spawn } from "node:child_process";

class PowerPoint {
	inner

	constructor() {
		const runner_url = new URL("runner/target/x86_64-pc-windows-msvc/release/runner.exe", import.meta.url);
		const runner_path = url.fileURLToPath(runner_url);
		this.inner = spawn(runner_path, ["node_modules/slideshow/slideshow-cli.js"]);

		return new Promise((resolve, reject) => {
			this.inner.stdout.on("data", (data) => {
				console.log(`Read: ${data}`);
				if (data.includes("Spawned")) {
					resolve(this);
				}
			})
		});
	}

	async wait(value) {
		return new Promise((resolve, reject) => {
			this.inner.stdout.on("data", (data) => {
				if (data.includes(value)) {
					resolve({})
				}
			})
		})
	}

	async open(path) {
		this.inner.stdin.write(`open ${path}\r\n`);
		await this.wait("slideshow(powerpoint)>");
		this.inner.stdin.write("start\n");
		await this.wait("slideshow(powerpoint)>");
	}

	async goto(slide) {
		this.inner.stdin.write(`goto ${slide}\r\n`);
		await this.wait("slideshow(powerpoint)>");
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