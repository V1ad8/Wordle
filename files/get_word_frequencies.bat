@echo off
setlocal enabledelayedexpansion

rem Input and output file paths
set "inputFile=words.txt"
set "outputFile=output.csv"

rem Define loading animation characters
set "loadingChars=\|/-"

rem Count lines in inputFile
for /f %%L in ('find /c /v "" ^< "%inputFile%"') do set "lineCount=%%L"

rem Initialize line counter
set /a "lineNum=0"

rem The line number where the script stopped last time
rem Last line with a word on it
set /a "done=14855"
set /a "current_max = lineCount - done"

rem Hide cursor
echo off

cls

rem Read each word from input file
for /f "usebackq delims=" %%A in ("%inputFile%") do (
    set /a "lineNum+=1"
    set "word=%%A"

    if !lineNum! gtr !done! (
        rem Display loading animation
        set /a "current_progress=((lineNum - done - 1) * 100000) / (lineCount - done)"
        set "first_half=!current_progress:~0,-3!"
        if "!first_half!" equ "" set "first_half=0"
        set "second_half=!current_progress:~-3!"
        if "!second_half!" equ "" set "second_half=000"
        if !second_half! lss 10 set "second_half=0!second_half!"
        if !second_half! lss 100 set "second_half=0!second_half!"
        set "current_progress=!first_half!.!second_half!"

        set /a "progress=((lineNum - 1) * 100000) / lineCount"
        set "first_half=!progress:~0,-3!"
        if "!first_half!" equ "" set "first_half=0"
        set "second_half=!progress:~-3!"
        if "!second_half!" equ "" set "second_half=000"
        if !second_half! lss 10 set "second_half=0!second_half!"
        if !second_half! lss 100 set "second_half=0!second_half!"
        set "progress=!first_half!.!second_half!"

        set /a "current_index = lineNum - done"

        cls

        rem Clear the current line and print the loading status
        echo Total: Processing word !lineNum! of !lineCount!: !progress!%%
        echo Current: Processing word !current_index! of !current_max!: !current_progress!%%
    
        rem Use wolframscript to query WordFrequencyData and fetch the frequency of the word
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
            echo !result!>>"%outputFile%"
        )   
    )
)

rem Show cursor again
echo on

echo.
echo Word frequencies have been saved to "%outputFile%"
