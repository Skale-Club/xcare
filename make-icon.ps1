# Draws icon-source.png (1024 px): an "X" with a sparkle, in the Skale blues
# on the near-black square shared by the Skale X apps.
# Then run:  npx tauri icon icon-source.png -o src-tauri/icons
param([string]$Out = (Join-Path $PSScriptRoot 'icon-source.png'))
Add-Type -AssemblyName System.Drawing
$size = 1024
$bmp = New-Object System.Drawing.Bitmap $size, $size
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = 'AntiAlias'
$g.Clear([System.Drawing.Color]::Transparent)

$r = 230; $m = 32; $w = $size - 2 * $m
$path = New-Object System.Drawing.Drawing2D.GraphicsPath
$path.AddArc($m, $m, $r, $r, 180, 90)
$path.AddArc($m + $w - $r, $m, $r, $r, 270, 90)
$path.AddArc($m + $w - $r, $m + $w - $r, $r, $r, 0, 90)
$path.AddArc($m, $m + $w - $r, $r, $r, 90, 90)
$path.CloseFigure()
$g.FillPath((New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(255, 12, 13, 18))), $path)

$soft = [System.Drawing.Color]::FromArgb(255, 143, 169, 238)
$brand = [System.Drawing.Color]::FromArgb(255, 81, 115, 214)

# "X": soft stroke under the CTA-blue stroke, like the other X apps
foreach ($pair in @(@($soft, 690, 330, 270, 790), @($brand, 270, 330, 690, 790))) {
  $pen = New-Object System.Drawing.Pen $pair[0], 132
  $pen.StartCap = 'Round'; $pen.EndCap = 'Round'
  $g.DrawLine($pen, $pair[1], $pair[2], $pair[3], $pair[4])
}

# Four-point sparkle, top right
function Sparkle($cx, $cy, $r, $color) {
  $k = $r * 0.22
  $pts = [System.Drawing.PointF[]]@(
    (New-Object System.Drawing.PointF $cx, ($cy - $r)), (New-Object System.Drawing.PointF ($cx + $k), ($cy - $k)),
    (New-Object System.Drawing.PointF ($cx + $r), $cy), (New-Object System.Drawing.PointF ($cx + $k), ($cy + $k)),
    (New-Object System.Drawing.PointF $cx, ($cy + $r)), (New-Object System.Drawing.PointF ($cx - $k), ($cy + $k)),
    (New-Object System.Drawing.PointF ($cx - $r), $cy), (New-Object System.Drawing.PointF ($cx - $k), ($cy - $k)))
  $g.FillPolygon((New-Object System.Drawing.SolidBrush $color), $pts)
}
Sparkle 790 230 120 $soft
Sparkle 860 420 52 $soft

$g.Dispose()
$bmp.Save($Out, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose()
"saved $Out"
