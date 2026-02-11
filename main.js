import { invoke } from '@tauri-apps/api/core';

const output = document.querySelector('#output');
const scanBtn = document.querySelector('#scan-btn');
const installBtn = document.querySelector('#install-btn');
const backupBtn = document.querySelector('#backup-btn');
const validateBtn = document.querySelector('#validate-btn');
const restoreBtn = document.querySelector('#restore-btn');
const backupPathInput = document.querySelector('#backup-path');
const restorePathInput = document.querySelector('#restore-path');

async function runAction(label, command, payload = {}) {
  output.textContent = `${label}...`;
  try {
    const result = await invoke(command, payload);
    output.textContent = `${label} completado:\n\n${result || '(sin salida)'}`;
  } catch (error) {
    output.textContent = `${label} falló:\n\n${String(error)}`;
  }
}

scanBtn.addEventListener('click', () => runAction('Escaneo', 'scan_driver_updates'));

installBtn.addEventListener('click', () => runAction('Instalación', 'install_driver_updates'));

backupBtn.addEventListener('click', () => {
  runAction('Respaldo', 'backup_drivers', { destination: backupPathInput.value.trim() });
});

validateBtn.addEventListener('click', () => {
  runAction('Validación', 'validate_backup_folder', { source: restorePathInput.value.trim() });
});

restoreBtn.addEventListener('click', () => {
  runAction('Reinstalación', 'reinstall_drivers', { source: restorePathInput.value.trim() });
});
