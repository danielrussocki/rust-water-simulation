param([int]$ProcessId)

$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Runtime.WindowsRuntime
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class WindowCapture {
    [StructLayout(LayoutKind.Sequential)]
    public struct Rect { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hwnd, out Rect rect);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hwnd);
}
'@

# GetWindowRect and CopyFromScreen must use the same physical pixel coordinates.
$null = [WindowCapture]::SetProcessDPIAware()

[Windows.Storage.StorageFile,Windows.Storage,ContentType=WindowsRuntime] > $null
[Windows.Storage.FileAccessMode,Windows.Storage,ContentType=WindowsRuntime] > $null
[Windows.Graphics.Imaging.BitmapDecoder,Windows.Graphics.Imaging,ContentType=WindowsRuntime] > $null
[Windows.Media.Ocr.OcrEngine,Windows.Foundation,ContentType=WindowsRuntime] > $null

function Await($operation, [Type]$resultType) {
    $method = [System.WindowsRuntimeSystemExtensions].GetMethods() |
        Where-Object { $_.Name -eq 'AsTask' -and $_.IsGenericMethod -and $_.GetParameters().Count -eq 1 -and $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1' } |
        Select-Object -First 1
    $task = $method.MakeGenericMethod($resultType).Invoke($null, @($operation))
    $task.GetAwaiter().GetResult()
}

$process = Get-Process -Id $ProcessId
$handle = $process.MainWindowHandle
if ($handle -eq [IntPtr]::Zero) { throw 'La aplicación no tiene ventana.' }
$null = [WindowCapture]::SetForegroundWindow($handle)
Start-Sleep -Seconds 2
$rect = New-Object WindowCapture+Rect
if (-not [WindowCapture]::GetWindowRect($handle, [ref]$rect)) { throw 'No se pudo obtener el tamaño de la ventana.' }
$bitmap = [System.Drawing.Bitmap]::new($rect.Right - $rect.Left, $rect.Bottom - $rect.Top)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$file = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.IO.Path]::GetRandomFileName() + '.png')
try {
    $graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, $bitmap.Size)
    # En S01 el área de contenido de error solo tiene texto sobre fondo uniforme.
    # Una escena visible a tamaño normal cambia sustancialmente esa superficie.
    $background = 0
    $samples = 0
    for ($y = 50; $y -lt $bitmap.Height - 15; $y += 4) {
        for ($x = 15; $x -lt $bitmap.Width - 15; $x += 4) {
            $pixel = $bitmap.GetPixel($x, $y)
            if ($pixel.R -eq 23 -and $pixel.G -eq 23 -and $pixel.B -eq 23) { $background++ }
            $samples++
        }
    }
    if ($samples -eq 0 -or ($background / $samples) -lt 0.65) {
        throw "El área de contenido no muestra solo el mensaje de error: $background / $samples píxeles de fondo."
    }
    $bitmap.Save($file, [System.Drawing.Imaging.ImageFormat]::Png)
    $storage = Await ([Windows.Storage.StorageFile]::GetFileFromPathAsync($file)) ([Windows.Storage.StorageFile])
    $stream = Await ($storage.OpenAsync([Windows.Storage.FileAccessMode]::Read)) ([Windows.Storage.Streams.IRandomAccessStream])
    $decoder = Await ([Windows.Graphics.Imaging.BitmapDecoder]::CreateAsync($stream)) ([Windows.Graphics.Imaging.BitmapDecoder])
    $image = Await ($decoder.GetSoftwareBitmapAsync()) ([Windows.Graphics.Imaging.SoftwareBitmap])
    $engine = [Windows.Media.Ocr.OcrEngine]::TryCreateFromUserProfileLanguages()
    if ($null -eq $engine) { throw 'No hay un idioma OCR de Windows disponible.' }
    $result = Await ($engine.RecognizeAsync($image)) ([Windows.Media.Ocr.OcrResult])
    Write-Output $result.Text
    $stream.Dispose()
} finally {
    $graphics.Dispose()
    $bitmap.Dispose()
    Remove-Item -LiteralPath $file -ErrorAction SilentlyContinue
}
