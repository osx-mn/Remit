import fs from 'node:fs';
import path from 'node:path';
import { spawn } from 'node:child_process';

const args = process.argv.slice(2);
const separatorIndex = args.indexOf('--');
const optionArgs = separatorIndex >= 0 ? args.slice(0, separatorIndex) : args;
const commandArgs = separatorIndex >= 0 ? args.slice(separatorIndex + 1) : [];
const target = commandArgs[0] ?? null;
const platformIndex = optionArgs.indexOf('--platform');
const platform = platformIndex >= 0 ? optionArgs[platformIndex + 1] : 'desktop';

const envOverrides = {};
for (let i = 0; i < args.length; i += 1) {
  if (args[i] === '--env' && args[i + 1]) {
    const raw = args[i + 1];
    const [key, ...rest] = raw.split('=');
    envOverrides[key] = rest.join('=');
    i += 1;
  }
}

if (!target || commandArgs.length === 0) {
  console.error('Uso: node scripts/run-with-log.js --platform <desktop|android> [--env KEY=VALUE] -- <comando> [args...]');
  process.exit(1);
}

const logsDir = path.resolve(process.cwd(), 'logs');
fs.mkdirSync(logsDir, { recursive: true });

const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
const logFile = path.join(logsDir, `${platform}-${timestamp}.log`);
const stream = fs.createWriteStream(logFile, { flags: 'a' });

const append = (text) => {
  const line = `[${new Date().toISOString()}] ${text}\n`;
  process.stdout.write(line);
  stream.write(line);
};

append(`Iniciando ejecución: ${commandArgs.join(' ')}`);
append(`Plataforma: ${platform}`);
append(`Log: ${logFile}`);

const childEnv = { ...process.env, ...envOverrides };

const child = spawn(target, commandArgs.slice(1), {
  shell: true,
  stdio: ['inherit', 'pipe', 'pipe'],
  env: childEnv,
});

child.stdout.on('data', (data) => {
  const text = data.toString();
  process.stdout.write(text);
  stream.write(text);
});

child.stderr.on('data', (data) => {
  const text = data.toString();
  process.stderr.write(text);
  stream.write(text);
});

child.on('close', (code, signal) => {
  append(`Proceso finalizado con código ${code ?? 'null'} y señal ${signal ?? 'null'}`);
  stream.end();
  process.exit(code ?? 0);
});

child.on('error', (error) => {
  append(`Error al iniciar proceso: ${error.message}`);
  stream.end();
  process.exit(1);
});
