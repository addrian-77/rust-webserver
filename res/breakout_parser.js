const pressedKeys = new Set();
let togglePriority = false;

function sendKeyStates() {
    if (pressedKeys.has("a") && pressedKeys.has("ArrowLeft"))
        ans("LeftLeft");
    else if (pressedKeys.has("a") && pressedKeys.has("ArrowRight"))
        ans("RightLeft");
    else if (pressedKeys.has("d") && pressedKeys.has("ArrowLeft"))
        ans("LeftRight");
    else if (pressedKeys.has("d") && pressedKeys.has("ArrowRight"))
        ans("RightRight");
    else if (pressedKeys.has("a"))
        ans ("Left2");
    else if (pressedKeys.has("d"))
        ans("Right2");
    else if (pressedKeys.has("ArrowLeft"))
        ans("Left");
    else if (pressedKeys.has("ArrowRight"))
        ans("Right");

    if (pressedKeys.has("ArrowUp")) ans("Up");
    if (pressedKeys.has("w")) ans("Up2");

    if (pressedKeys.has("ArrowDown")) ans("Down");
    if (pressedKeys.has("s")) ans("Down2");

    if (pressedKeys.has("Enter")) ans("Select");
    if (pressedKeys.has("Escape")) ans("Back");
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
