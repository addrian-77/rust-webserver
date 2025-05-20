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


window.addEventListener("keydown", (e) => {
    const key = e.key;
    console.log(key);

    switch (key) {
        case "ArrowUp": ans("Up"); break;
        case "ArrowDown": ans("Down"); break;
        case "ArrowLeft": ans("Left"); break;
        case "ArrowRight": ans("Right"); break;

        case "w": ans("Up2"); break;
        case "s": ans("Down2"); break;
        case "a": ans("Left2"); break;
        case "d": ans("Right2"); break;

        case "Enter": ans("Select"); break;
        case "Escape": ans("Back"); break;
    }
});
