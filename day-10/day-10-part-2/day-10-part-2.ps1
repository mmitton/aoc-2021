$file = Get-Content ../input.txt

$answers = New-Object Collections.Generic.List[int64]
foreach ($line in $file -split "\n") {
    $stack = New-Object Collections.Generic.Stack[char]
    $invalid = $false
    for ($i = 0; $i -lt $line.length; $i++) {
        switch ($line[$i]) {
            '(' { $stack.Push('('); }
            '[' { $stack.Push('['); }
            '{' { $stack.Push('{'); }
            '<' { $stack.Push('<'); }
            ')' { if ($stack.Pop() -ne '(') { $answer += 3; $invalid = $true; break; } }
            ']' { if ($stack.Pop() -ne '[') { $answer += 57; $invalid = $true; break; } }
            '}' { if ($stack.Pop() -ne '{') { $answer += 1197; $invalid = $true; break; } }
            '>' { if ($stack.Pop() -ne '<') { $answer += 25137; $invalid = $true; break; } }
        }
    }

    if (!$invalid) {
        $points = 0

        while ($stack.Count -ne 0) {
            $points *= 5;
            switch ($stack.Pop()) {
                '(' { $points += 1; }
                '[' { $points += 2; }
                '{' { $points += 3; }
                '<' { $points += 4; }
            }
        }

        $answers.Add($points);
    }
}

$answers.Sort()
$answer = $answers[$answers.Count / 2]
Write-Host "Answer: $answer  Answers: $answers"
