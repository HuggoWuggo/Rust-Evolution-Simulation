import init, {Simulation} from "./pkg/simulation_wasm.js"

CanvasRenderingContext2D.prototype.drawTriangle =
function (x, y, size, rotation, col) {
	this.beginPath();
	this.moveTo(
		x - Math.sin(rotation) * size * 1.5,
		y + Math.cos(rotation) * size * 1.5,
	);

	this.lineTo(
		x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
		y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
	);
	this.lineTo(
		x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
		y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
	);	
	this.lineTo(
		x - Math.sin(rotation) * size * 1.5,
		y + Math.cos(rotation) * size * 1.5,
	);
	
	this.stroke();

	this.fillStyle = col;
	this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = 
	function(x, y, radius) {
		this.beginPath();

		this.arc(x, y, radius, 0, 2.0 * Math.PI);

		this.fillStyle = 'rgb(0,255, 128)';
		this.fill();
	}

function table(trained) {
		let txt = trained.split("min=")
		let min = txt[1].split(",", 1);
		
		txt = trained.split("max=");
		let max = txt[1].split(",", 1);

		txt = trained.split("avg=");
		let avg = txt[1].split(",", 1);
		
		txt = trained.split("max_index=");
		let max_index = txt[1].split(",", 1);

		txt = trained.split("min_index=");
		let min_index = txt[1].split(",", 1);
	
		document.getElementById("max").innerText = max;
		document.getElementById("max_index").innerText = max_index;
		document.getElementById("min").innerText = min;
		document.getElementById("min_index").innerText = min_index;
		document.getElementById("avg").innerText = avg;
}

async function run() {

	await init();

	const simulation = new Simulation();

	table(simulation.train())
	
	document.getElementById('train').onclick = function() {
		let trained = simulation.train();

		table(trained);
	};

	const viewport = document.getElementById('viewport');
	const viewportWidth = viewport.width;
	const viewportHeight = viewport.height;
	const viewportScale = window.devicePixelRatio || 1;

	viewport.width = viewportWidth * viewportScale;
	viewport.height = viewportHeight * viewportScale;
	viewport.style.width = viewportWidth + 'px';
	viewport.style.height = viewportHeight + 'px';

	const ctxt = viewport.getContext('2d');
	
	ctxt.scale(viewportScale, viewportScale);

	ctxt.fillStyle = 'rgb(0,0,0)';
	
	function redraw() {
		ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

		simulation.step();
		
		const world = simulation.world();
		
		for (const food of world.foods) {
			ctxt.drawCircle(
				food.x * viewportWidth,
				food.y * viewportHeight,
				(0.01 / 2.0) * viewportWidth,
			);
		}

		for (const [i, animal] of world.animals.entries()) {
			let colour = 'rgb(255, 255, 255)';

			//if (document.getElementById("max_index").innerText == i) {
				//colour = 'rgb(255, 0, 0)';
			//}

			//else if (document.getElementById("min_index").innerText == i) {
				//colour = 'rgb(0, 0, 255)';
			//}

			ctxt.drawTriangle(
				animal.x * viewportWidth,
				animal.y * viewportHeight,
				0.01 * viewportWidth,
				animal.rotation,
				colour,
			);
		}

		requestAnimationFrame(redraw)
	}
	redraw()
}

run();
