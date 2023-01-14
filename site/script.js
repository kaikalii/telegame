
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

let keys = []
let keysDown = []

window.onkeydown = function (event) {
    keys.push({
        key: event.key,
        pressed: true,
        alt: event.altKey,
        ctrl: event.ctrlKey,
        shift: event.shiftKey,
        meta: event.metaKey,
        repeat: event.repeat
    });
    if (!keysDown.includes(event.key))
        keysDown.push(event.key);
}

window.onkeyup = function (event) {
    keys.push({
        key: event.key,
        pressed: false,
        alt: event.altKey,
        ctrl: event.ctrlKey,
        shift: event.shiftKey,
        meta: event.metaKey,
        repeat: event.repeat
    });
    var index = keysDown.indexOf(event.key);
    if (index !== -1) {
        keysDown.splice(index, 1);
    }
}

const windowWidth = () => canvas.width;
const windowHeight = () => canvas.height;

let lastTime = new Date().getTime();

function getInput() {
    let inputKeys = keys;
    keys = []
    let time = new Date().getTime();
    let dt = time - lastTime;
    lastTime = time;
    return {
        closed: false,
        mouse_pos: mousePos,
        window_size: { x: windowWidth(), y: windowHeight() },
        key_events: inputKeys,
        keys_down: keysDown,
        dt: dt / 1000
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
            case "font":
                ctx.font = com.font
                break;
            case "text":
                ctx.fillText(com.text, com.pos.x, com.pos.y);
                break;
            default:
                console.log("Unknown command: " + com.type);
                break;
        }
    }
}
