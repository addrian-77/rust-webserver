var xhr = new XMLHttpRequest();
function get(elem) {
    var value = elem.getAttribute("name");
    xhr.open(value, "/" , true);
    xhr.send();
}

function ans(value) {
    xhr.open(value, "/" , true);
    xhr.send();
}

window.addEventListener("keydown", (e) => {
    if(e.key === "ArrowUp") {
        ans("Up");
    } else if (e.key === "ArrowDown") {
        ans("Down");
    } else if (e.key == "ArrowLeft") {
        ans("Left");
    } else if (e.key == "ArrowRight") {
        ans("Right");
    } else if(e.key === "w") {
        ans("Up2");
    } else if(e.key === "s") {
        ans("Down2");
    } else if(e.key === "a") {
        ans("Left2");
    } else if(e.key === "d") {
        ans("Right2");
    }
})