pipeline {
    agent any
    stages {
        stage('intialize') {
            steps {
                sh 'echo "PATH= ${PATH}"'
                }
            }
        stage('Build') {
            steps { 
                  sh 'cargo build'
                }
            }                   
        }
        stage('Test my functions') {
            steps { 
                  sh 'python -m robot test_my_functions.robot'
                }
            }                   
        }
}