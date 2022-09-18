#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { exec } = require("child_process");

const cargoDir = path.dirname("$HOME" + ".cargo");

// check if directory exists
if (fs.existsSync(cargoDir)) {
  //   console.log("Cargo found.");
} else {
  const setCargo = 'PATH="/$HOME/.cargo/bin:${PATH}"';
  console.log("Installing deps [cargo].");

  exec(
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && ${setCargo}`,
    (error) => {
      if (error) {
        console.log(
          "curl failed! Curl may not be installed on the OS. View https://curl.se/download.html to install."
        );
        console.log(error);
      }
    }
  );
}
    
const binp = path.join(cargoDir, "bin", "darkness-check");

if (!fs.existsSync(binp)) {
  console.log("Installing and compiling darkness-check...");
  exec(`cargo install darkness-check`, (error, stdout, stderr) => {
    console.log(stdout);
    if (error || stderr) {
      console.log(error || stderr);
    } else {
      console.log("install finished!");
    }
  });
} else {
  console.log("darkness-check found install skipping!");
}

    