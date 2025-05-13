*** Settings ***
Library    python_script.py

*** Test Cases ***
Test Factorial With Loop
    ${result}=    python_script.Function Calculate Factorial With Loop   4
    Should Be Equal As Numbers    ${result}    24

Test Factorial Recursively
    ${result}=    python_script.Function Calculate Factorial Recursively   5
    Should Be Equal As Numbers    ${result}    120

Test Fibonacci With Loop
    ${result}=    python_script.Function Calculate Fibonacci With Loop   6
    Should Be Equal As Numbers    ${result}    8

Test Fibonacci Recursively
    ${result}=    python_script.Function Calculate Fibonacci Recursively   7
    Should Be Equal As Numbers    ${result}    13