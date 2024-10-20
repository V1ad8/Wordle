@echo off
setlocal enabledelayedexpansion

set "word=%1"

for /f "delims=" %%B in ('wolframscript -code "WordFrequencyData[\"!word!\"]"') do (
            set "frequency=%%B"
            
            rem Split the input into two parts based on '*^'
            for /f "tokens=1,2 delims=*^" %%a in ("!frequency!") do (
                set "part1=%%a"
                set "part2=%%b"
            )

            rem Add 10 to the exponent part
            set /a "new_exponent=part2 + 10"

            rem Split the part1 around the dot (.)
            for /f "tokens=1,2 delims=." %%c in ("!part1!") do (
                set "mantissa=%%c"
                set "digits=%%d"
            )

            rem Adjust mantissa and digits based on new_exponent
            if !new_exponent! gtr 0 (
                rem Move digits from digits to mantissa
                set "adjusted_mantissa=!mantissa!"
                set "adjusted_digits=!digits!"

                for /l %%i in (1,1,!new_exponent!) do (
                    if defined adjusted_digits (
                        set "adjusted_mantissa=!adjusted_mantissa!!adjusted_digits:~0,1!"
                        set "adjusted_digits=!adjusted_digits:~1!"
                    )
                )
            ) else if !new_exponent! lss 0 (
                rem Move digits from mantissa to digits
                set "adjusted_mantissa=!mantissa!"
                set "adjusted_digits=!digits!"

                for /l %%i in (!new_exponent!,1,0) do (
                    if defined adjusted_mantissa (
                        set "adjusted_digits=!adjusted_mantissa:~-1!!adjusted_digits!"
                        set "adjusted_mantissa=!adjusted_mantissa:~0,-1!"
                    ) else (
                        set "adjusted_digits=0!adjusted_digits!"
                    )
                )

                rem If mantissa becomes empty, set it to '0'
                if "!adjusted_mantissa!" equ "" set "adjusted_mantissa=0"
            ) else (
                rem new_exponent is 0, no adjustment needed
                set "adjusted_mantissa=!mantissa!"
                set "adjusted_digits=!digits!"
            )

            rem Truncate adjusted_mantissa to 6 digits after the dot
            set "adjusted_digits=!adjusted_digits:~0,6!"

            rem Construct the final output with scientific notation
            set "finalFrequency=!adjusted_mantissa!.!adjusted_digits!"

            rem Format result: word,finalFrequency
            set "result=!word!,!finalFrequency!"
            
            rem Append to output file
            echo !result!
)