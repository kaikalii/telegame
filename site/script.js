
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

function getInput() {
    return {
        closed: false,
        mouse_pos: mousePos,
        window_size: { x: windowWidth(), y: windowHeight() }
    };
}

function run() {
    if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify(getInput()));
    }
    setTimeout(run, 0);
}

let mousePos = { x: 0, y: 0 };

function setMousePos(evt) {
    var rect = canvas.getBoundingClientRect();
    mousePos.x = (evt.clientX - rect.left) / (rect.right - rect.left) * canvas.width;
    mousePos.y = (evt.clientY - rect.top) / (rect.bottom - rect.top) * canvas.height;
}

function draw(frame) {

    for (const com of frame.commands) {
        switch (com.type) {
            case "clear": ctx.clearRect(0, 0, canvas.width, canvas.height);
                break;
            case "color": ctx.fillStyle = com.color;
                break;
            case "rectangle": ctx.fillRect(com.pos.x, com.pos.y, com.size.x, com.size.y);
                break
            case "circle": ctx.beginPath();
                ctx.arc(com.pos.x, com.pos.y, com.radius, 0, 2 * Math.PI);
                ctx.fill();
                break
            default:
                console.log("Unknown command: " + com.type);
                break;
        }
    }
}
