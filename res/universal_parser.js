const pressedKeys = new Set();
let togglePriority = false;

function sendKeyStates() {
    togglePriority = !togglePriority;

    if (togglePriority) {
        handleArrows();
        handleWASD();
    } else {
        handleWASD();
        handleArrows();
    }

    if (pressedKeys.has("Enter")) ans("Select");
    if (pressedKeys.has("Escape")) ans("Back");
}

function handleArrows() {
    if (pressedKeys.has("ArrowUp")) {
        if (pressedKeys.has("ArrowLeft"))
            ans("Left_Shoot");
        else if (pressedKeys.has("ArrowRight"))
            ans("Right_Shoot");
        else ans("Up");
    } 
    else if (pressedKeys.has("ArrowDown")) ans("Down");
    else if (pressedKeys.has("ArrowLeft")) ans("Left");
    else if (pressedKeys.has("ArrowRight")) ans("Right");
}

function handleWASD() {
    if (pressedKeys.has("w")) {
        if (pressedKeys.has("a"))
            ans("Left2_Shoot");
        else if (pressedKeys.has("d"))
            ans("Right2_Shoot");
        else ans("Up2");
    }
    else if (pressedKeys.has("s")) ans("Down2");
    else if (pressedKeys.has("a")) ans("Left2");
    else if (pressedKeys.has("d")) ans("Right2");
}

window.addEventListener("keydown", (e) => {
    pressedKeys.add(e.key);
});

window.addEventListener("keyup", (e) => {
    pressedKeys.delete(e.key);
});

setInterval(sendKeyStates, 10);

function ans(value) {
    let command = "";

    if (typeof value === "string") {
        command = value;
    } else if (value instanceof HTMLElement) {
        command = value.name || value.id;
    }

    const xhr = new XMLHttpRequest();
    xhr.open(command, "/", true);
    xhr.send();
}


const touchIntervals = {}; // Stores active intervals for each button

function startSending(button) {
    const name = button.name || button.id;
    if (touchIntervals[name]) return; // Already sending

    ans(name); // Send immediately on touch
    touchIntervals[name] = setInterval(() => ans(name), 10); // Send every 100ms
}

function stopSending(button) {
    const name = button.name || button.id;
    clearInterval(touchIntervals[name]);
    delete touchIntervals[name];
}
