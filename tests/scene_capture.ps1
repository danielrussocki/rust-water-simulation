param([string]$Executable, [string]$Project)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class SceneWindow {
    [StructLayout(LayoutKind.Sequential)]
    public struct Rect { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr hwnd, out Rect rect);
    [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr hwnd, ref Point point);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hwnd);
    [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr hwnd, IntPtr insertAfter, int x, int y, int cx, int cy, uint flags);
    [StructLayout(LayoutKind.Sequential)]
    public struct Point { public int X, Y; }
}
'@
$process = Start-Process -FilePath $Executable -WorkingDirectory $Project -PassThru
try {
    for ($i = 0; $i -lt 100; $i++) {
        $process.Refresh()
        if ($process.HasExited) { throw 'La aplicación terminó antes de abrir la escena.' }
        if ($process.MainWindowHandle -ne [IntPtr]::Zero) { break }
        Start-Sleep -Milliseconds 100
    }
    if ($process.MainWindowHandle -eq [IntPtr]::Zero) { throw 'No se abrió una ventana.' }
    $null = [SceneWindow]::SetForegroundWindow($process.MainWindowHandle)
    Start-Sleep -Seconds 2
    # La escena usa el tamaño disponible del contenedor GPUI; verificarlo en una
    # ventana real evita aceptar una imagen fija con el tamaño inicial.
    $null = [SceneWindow]::SetWindowPos(
        $process.MainWindowHandle, [IntPtr]::Zero, 0, 0, 640, 520, 0x0006)
    Start-Sleep -Milliseconds 500
    $rect = New-Object SceneWindow+Rect
    $point = New-Object SceneWindow+Point
    if (-not [SceneWindow]::GetClientRect($process.MainWindowHandle, [ref]$rect) -or
        -not [SceneWindow]::ClientToScreen($process.MainWindowHandle, [ref]$point)) {
        throw 'No se pudo medir la ventana.'
    }
    if ($rect.Right -lt 500 -or $rect.Bottom -lt 350) {
        throw "La vista no se redimensionó: $($rect.Right)x$($rect.Bottom)."
    }
    function Capture {
        $image = [System.Drawing.Bitmap]::new($rect.Right, $rect.Bottom)
        $graphics = [System.Drawing.Graphics]::FromImage($image)
        try { $graphics.CopyFromScreen($point.X, $point.Y, 0, 0, $image.Size) }
        finally { $graphics.Dispose() }
        return $image
    }
    $first = Capture
    Start-Sleep -Milliseconds 500
    $second = Capture
    try {
        $colors = @{}
        $different = 0
        $total = 0
        for ($y = 10; $y -lt $rect.Bottom - 10; $y += 4) {
            for ($x = 10; $x -lt $rect.Right - 10; $x += 4) {
                $a = $first.GetPixel($x, $y).ToArgb()
                $b = $second.GetPixel($x, $y).ToArgb()
                $colors[$a] = $true
                if ($a -ne $b) { $different++ }
                $total++
            }
        }
        if ($colors.Count -lt 100) { throw "No se observa una escena con recursos: $($colors.Count) colores." }
        if ($different -lt 100) { throw "No se observa evolución: $different de $total muestras cambiaron." }
    } finally {
        $first.Dispose()
        $second.Dispose()
    }
} finally {
    if (-not $process.HasExited) {
        if (-not $process.CloseMainWindow()) { throw 'No se pudo cerrar la ventana.' }
        if (-not $process.WaitForExit(10000)) { throw 'La aplicación no se cerró.' }
    }
}
