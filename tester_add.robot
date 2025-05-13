*** Settings ***
Library    python_script.py

*** Test Cases ***
Test Add Numbers
    ${result}=    add    3    5
    Should Be Equal As Numbers    ${result}    8