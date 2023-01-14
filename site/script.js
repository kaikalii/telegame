
let canvas;
let ctx;

let socket = new WebSocket("ws://localhost:8001");

socket.onopen = function (e) {
    console.log("[open] Connection established");
}

socket.onmessage = function (e) {
    let resp = JSON.parse(e.data);
    if (resp.success)
        draw(resp.frame);
    else
        console.log(resp.error);
}

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
    if (socket.readyState === WebSocket.OPEN) {
        let input = {
            mouse_pos: mousePos,
            window_size: { x: windowWidth(), y: windowHeight() }
        }
        socket.send(JSON.stringify(input));
    }
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

function circle(x, y, radius, color) {
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fillStyle = color;
    ctx.fill();
}

function draw(frame) {
    if (frame.clear) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
    }

    for (const shape of frame.shapes) {
        switch (shape.type) {
            case "rectangle":
                rectangle(shape.pos.x, shape.pos.y, shape.size.x, shape.size.y, shape.color);
                break
            case "circle":
                circle(shape.pos.x, shape.pos.y, shape.radius, shape.color);
                break
        }
    }
}
