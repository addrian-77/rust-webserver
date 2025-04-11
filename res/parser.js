function send(elem) {
    var value = elem.getAttribute("name");
    var xhr = new XMLHttpRequest();
    xhr.open(value, "/" , true);
    xhr.send();
}
