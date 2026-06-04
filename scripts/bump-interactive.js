#!/usr/bin/env node

/*
* npm wrapper script to run the bump script with an interactive prompt for the version to bump to.
*/

import readline from 'readline';
import { spawn } from 'child_process';
import { platform } from 'process';

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

const question = (q) => new Promise(resolve => rl.question(q, ans => resolve(ans)));

try {
    const ver = (await question('Enter version to bump to: ')).trim();
    rl.close();
    if (!ver) process.exit(1);
    const cmd = platform === 'win32' ? 'npm.cmd' : 'npm';
    const child = spawn(cmd, ['run', 'bump', '--', ver], { stdio: 'inherit' });
    child.on('close', code => process.exit(code));
} catch (e) {
    rl.close();
    console.error(e);
    process.exit(1);
}