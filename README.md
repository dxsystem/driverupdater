# DriverUpdater (Rust + Tauri v2)

Aplicación de escritorio para Windows que permite:

- Buscar actualizaciones de drivers desde Microsoft Update (PSWindowsUpdate).
- Instalar actualizaciones de drivers.
- Respaldar drivers instalados (`Export-WindowsDriver`).
- Validar e reinstalar drivers desde un respaldo (`pnputil`).

## Requisitos

- Windows 10/11.
- PowerShell 5+.
- Permisos de administrador para instalación/reinstalación.
- Node.js 18+.
- Rust estable.

## Desarrollo

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Flujo de actualización usado

El backend ejecuta un script PowerShell equivalente a:

```powershell
Install-Module PSWindowsUpdate -Force
Import-Module PSWindowsUpdate
Get-WindowsUpdate -MicrosoftUpdate
Install-WindowsUpdate -MicrosoftUpdate -AcceptAll -AutoReboot
```

> Nota: la app valida inputs de rutas, pero depende de herramientas nativas de Windows y puede requerir ejecutar la app como Administrador.
