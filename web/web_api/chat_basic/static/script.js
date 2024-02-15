let roomListDiv = document.getElementById('room-list');
let messagesDiv = document.getElementById('messages');
let newMessageForm = document.getElementById('new-message');
let newRoomForm = document.getElementById('new-room');
let statusDiv = document.getElementById('status');

let roomTemplate = document.getElementById('room');
let messageTemplate = document.getElementById('message');

let messageField = newMessageForm.querySelector("#message");
let usernameField = newMessageForm.querySelector("#username");
let roomNameField = newRoomForm.querySelector("#name");

var STATE = {
    room: "lobby",
    rooms: {},
    connected: false,
}

function hashColor(str) {
    let hash = 0;
    for (var i = 0; i < str.length; i++) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
        hash = hash & hash;
    }

    return `hsl(${hash % 360}, 100%, 70%)`;
}

function addRoom(name) {
    if (STATE[name]) {
        changeRoom(name)
        return false
    }


}

function changeRoom(name) {
    if (STATE.room == name) return;
}

function addMessage(room, username, message, push = false) {
    if (push) {
        STATE[room].push({ username, message })
    }


}

function subscribe(uri) {
    var retryTime = 1
}

function setConnectedStatus(status) {

}

function init() {
    addRoom("lobby")
    addRoom("rocket")
    changeRoom("lobby")
    addMessage("lobby", "Rocket", "Rocket tab", true)
    addMessage("rocket", "Rocket", "Another Rocket tab", true)
}

init()