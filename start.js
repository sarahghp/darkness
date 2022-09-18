#!/usr/bin/env node

const { exec } = require("child_process");

exec("darkness-check", (error, stdout, stderr) => {
  stdout && console.log(stdout);
  stderr && console.log(stderr);
  if (error !== null) {
    console.log(`exec error: ${error}`);
  }
});    
    