const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let sendPort;

async function get() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("get");



}

async function send() {
  greetMsgEl.textContent = await invoke("send", { port: sendPort.value, data: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  document
      .querySelector("#get-button")
      .addEventListener("click", () => get());
});

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#send-input");
  greetMsgEl = document.querySelector("#greet-msg");
  sendPort = document.querySelector("#send-port")

  document
      .querySelector("#send-button")
      .addEventListener("click", () => send());
});
