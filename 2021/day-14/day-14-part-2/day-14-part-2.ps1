$file = Get-Content ../input.txt
$lines = $file -split "\n";

$template = $lines[0];
$rules = New-Object System.Collections.Generic.Dictionary"[String, String]"
for ($i = 2; $i -lt $lines.length; $i++) {
    $line = $lines[$i].trim();
    if ($line -eq "") {
        continue
    }
    $rules.Add($line.SubString(0, 2), $line.SubString(6, 1));
}

$pairs = New-Object System.Collections.Generic.Dictionary"[String, Int64]"
for ($i = 0; $i -lt $template.length - 1; $i++) {
    $pairs[$template.SubString($i, 2)] += 1
}

for ($i = 0; $i -lt 40; $i++) {
    $new_pairs = New-Object System.Collections.Generic.Dictionary"[String, Int64]"

    foreach ($key in $pairs.Keys) {
        $insert = $rules[$key]
        foreach ($new_key in ($key[0]+$insert), ($insert+$key[1])) {
            $new_pairs[$new_key] += $pairs[$key]
        }
    }
    $pairs = $new_pairs
}

$letters = New-Object System.Collections.Generic.Dictionary"[String, Int64]"
foreach ($key in $pairs.Keys) {
    foreach ($letter in $key[0], $key[1]) {
        $letters[$letter] += $pairs[$key]
    }
}


$min = $null
$max = $null 
Write-Host "letters:"
foreach ($key in $letters.Keys) {
    $letters[$key] /= 2
    if ($key -eq $template[0] -or $key -eq $template[$template.length - 1]) {
        $letters[$key] += 1
    }
    if ($min -eq $null -or $letters[$key] -lt $min) {
        $min = $letters[$key]
    }
    if ($max -eq $null -or $letters[$key] -gt $max) {
        $max = $letters[$key]
    }
    Write-Host "  $key => $($letters[$key])"
}

Write-Host "max:$max min:$min  $($max - $min)"
