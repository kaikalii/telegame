
let canvas;
let ctx;

function init() {
    canvas = document.getElementById("canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    ctx = canvas.getContext("2d");
    run();
}

window.onresize = function (event) {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
};

const windowWidth = () => canvas.width;
const windowHeight = () => canvas.height;

function run() {
    draw();
    setTimeout(run, 0);
}

let mousePos = { x: 0, y: 0 };

function setMousePos(evt) {
    var rect = canvas.getBoundingClientRect();
    mousePos.x = (evt.clientX - rect.left) / (rect.right - rect.left) * canvas.width;
    mousePos.y = (evt.clientY - rect.top) / (rect.bottom - rect.top) * canvas.height;
}

function rectangle(x, y, width, height, color) {
    ctx.fillStyle = color;
    ctx.fillRect(x, y, width, height);
}

function draw() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    rectangle(mousePos.x - 50, mousePos.y - 50, 100, 100, "red");
}

