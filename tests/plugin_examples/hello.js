const readline = require("readline");

const NAME = "js";

function send(message) {
  process.stdout.write(JSON.stringify(message) + "\n");
}

function render(text) {
  const segment = { text, color: "#82C0FF", bold: true };
  send({ type: "render", lines: [{ segments: [segment] }] });
}

function handleKey(key) {
  if (key === "q") {
    send({ type: "close" });
  } else if (key === "o") {
    send({ type: "open_file", path: "/tmp/cera-plugin-test.txt" });
  } else if (key === "s") {
    send({ type: "status", message: `status ${NAME}`, kind: "error" });
  } else {
    render(`key ${NAME} ${key}`);
  }
}

process.stdout.write(`debug: starting ${NAME}\n`);

const input = readline.createInterface({ input: process.stdin });

input.on("line", (raw) => {
  let message;
  try {
    message = JSON.parse(raw);
  } catch (error) {
    return;
  }
  switch (message.type) {
    case "init":
      render(`hello ${NAME} ${message.mode || "fullscreen"} ${message.screen_rows}x${message.screen_cols}`);
      break;
    case "resize":
      render(`resize ${NAME} ${message.screen_rows}x${message.screen_cols}`);
      break;
    case "key":
      handleKey(message.key);
      break;
    case "click":
      render(`click ${NAME} ${message.row}`);
      break;
    case "focus":
      render(`focus ${NAME} ${message.focused ? "true" : "false"}`);
      break;
    case "shutdown":
      process.exit(0);
  }
});
