var xhr = new XMLHttpRequest();
function send(elem) {
    var value = elem.getAttribute("name");
    xhr.open(value, "/" , true);
    xhr.send();
}
