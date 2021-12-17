$file = Get-Content ../input.txt
$line = ($file -split "\n")[0]
$line = $line -replace "target area: x="
$line = $line -replace "\.\.", " "
$line = $line -replace ", y=", " "
$nums = $line -split " "
$x0 = [int]$nums[0]
$x1 = [int]$nums[1]
$y0 = [int]$nums[2]
$y1 = [int]$nums[3]

Write-Host "$x0..$x1  $y0..$y1"
$total_solutions = 0
for ($y_initial = -$y0; $y_initial -ge $y0; $y_initial--) {
    for ($x_initial = 0; $x_initial -le $x1; $x_initial++) {
        $x = 0
        $y = 0
        $x_vel = $x_initial;
        $y_vel = $y_initial;
        $max_height = 0;

        for (;;) {
            $x += $x_vel
            $y += $y_vel

            $y_vel -= 1
            if ($x_vel -gt 0) {
                $x_vel -= 1
            } elseif ($x_vel -lt 0) {
                $x_vel += 1
            }

            if ($y -gt $max_height) {
                $max_height = $y
            }

            if ($y -lt $y0) {
                break
            }
            if ($x -gt $x1) {
                break
            }

            if ($y -ge $y0 -and $y -le $y1 -and $x -ge $x0 -and $x -le $x1) {
                Write-Host "$x_initial,$y_initial  Max Height: $max_height";
                $total_solutions += 1
                break
            }
        }
    }
}

Write-Host "Total Solutions: $total_solutions"

