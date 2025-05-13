*** Settings ***
Library    python_script.py

*** Test Cases ***
Test Calculate Factorial 0 With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   0
    Should Be Equal As Numbers    ${result}    1

Test Calculate Factorial 1 With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   1
    Should Be Equal As Numbers    ${result}    1

Test Calculate Factorial 2 With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   2
    Should Be Equal As Numbers    ${result}    2

Test Calculate Factorial 3 With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   3
    Should Be Equal As Numbers    ${result}    6

Test Calculate Factorial 4 With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   4
    Should Be Equal As Numbers    ${result}    24

Test Calculate Factorial 5 With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   5
    Should Be Equal As Numbers    ${result}    120

Test Calculate Factorial 0 Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   0
    Should Be Equal As Numbers    ${result}    1

Test Calculate Factorial 1 Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   1
    Should Be Equal As Numbers    ${result}    1

Test Calculate Factorial 2 Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   2
    Should Be Equal As Numbers    ${result}    2

Test Calculate Factorial 3 Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   3
    Should Be Equal As Numbers    ${result}    6

Test Calculate Factorial 4 Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   4
    Should Be Equal As Numbers    ${result}    24

Test Calculate Factorial 5 Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   5
    Should Be Equal As Numbers    ${result}    120

Test Calculate Fibonacci 1 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   1
    Should Be Equal As Numbers    ${result}    1

Test Calculate Fibonacci 2 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   2
    Should Be Equal As Numbers    ${result}    1

Test Calculate Fibonacci 3 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   3
    Should Be Equal As Numbers    ${result}    2

Test Calculate Fibonacci 4 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   4
    Should Be Equal As Numbers    ${result}    3

Test Calculate Fibonacci 5 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   5
    Should Be Equal As Numbers    ${result}    5

Test Calculate Fibonacci 6 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   6
    Should Be Equal As Numbers    ${result}    8

Test Calculate Fibonacci 7 With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   7
    Should Be Equal As Numbers    ${result}    13

Test Calculate Fibonacci 1 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   1
    Should Be Equal As Numbers    ${result}    1

Test Calculate Fibonacci 2 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   2
    Should Be Equal As Numbers    ${result}    1

Test Calculate Fibonacci 3 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   3
    Should Be Equal As Numbers    ${result}    2

Test Calculate Fibonacci 4 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   4
    Should Be Equal As Numbers    ${result}    3

Test Calculate Fibonacci 5 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   5
    Should Be Equal As Numbers    ${result}    5

Test Calculate Fibonacci 6 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   6
    Should Be Equal As Numbers    ${result}    8

Test Calculate Fibonacci 7 Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   7
    Should Be Equal As Numbers    ${result}    13