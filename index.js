import { spawn } from 'node:child_process'
import WebSocket from 'ws'
import path, { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

export default class PowerPoint {
	ws
	msgs

	constructor(file_path) {
		let global_path = path.resolve(file_path);
		console.log("Opening powerpoint at: ", global_path);

		let __file = fileURLToPath(import.meta.url);
		let __dir = dirname(__file);
		const exe = 'PowerPointController/bin/Release/PowerPointController.exe';

		let child = spawn(path.join(__dir, exe), [global_path]);
		child.stdout.on('data',  (data) => {
			console.log(`Got data in child: ${data}`);
		});
		child.stderr.on('data',  (data) => {
			console.error(`Got error in child: ${data}`);
		});

		function sleepFor(sleepDuration){
			var now = new Date().getTime();
			while(new Date().getTime() < now + sleepDuration){ /* Do nothing */ }
		}

		sleepFor(2000);

		this.ws = new WebSocket('ws://localhost:8181/ppt');
		this.msgs = [];

		this.ws.onmessage = (event) => {
		  if (event.data.startsWith('slide:')) {
		    const current = event.data.split(':')[1];
		    console.log('Actieve dia:', current);
		  }
		};

		this.ws.onopen = () => {
			while (this.msgs.length > 0) {
				this.ws.send(this.msgs.shift());
			}
		};

		this.ws.onerror = console.error
	}

	// Voor besturing vanuit UI:
	next() {
	  send(this, 'next');
	}

	goto(n) {
	  send(this, `goto:${n}`);
	}
}

function send(pp, msg) {
	if (pp.ws.readyState !== 1) {
		pp.msgs.push(msg);
	} else {
		pp.ws.send(msg);
	}
}