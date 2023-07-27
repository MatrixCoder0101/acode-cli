const shelljs = require("shelljs");

const { exec } = shelljs;

// function to execute commands in cli
function shellExec(command, silentMode = true) {
  const result = exec(command, { silent: silentMode });
  if (result.code !== 0) {
    throw new Error(`Error executing command: ${command}`);
  }
}

// export the shellExec function
module.exports = shellExec;
